use std::fs::File;
use std::io::prelude::*;

// Use a channel to keep data between threads
use std::sync::mpsc;

use regex::Regex;

mod queue;
use queue::Queue;

#[derive(Debug)]
struct Video {
    id: String,
    title: String,
    link: String,
    format: String,
    parts: u64
}

impl Video {
    async fn new(url: String) -> Video {
        let re_video_info = Regex::new(r#"(<meta property="og:title" content="(?P<video_title>.*)"/>)[\n\d\w\W]*(file: '(?P<video_link>https://streaming-arquivo-ondemand.rtp.pt.*)(?P<video_index>index.*streams=)(?P<video_id>.*)(?P<video_format>\.mp4))"#).unwrap();

        let resp = reqwest::get(&url).await.unwrap();
        let resp_body = resp.text().await.unwrap();

        // Get the video name
        let video_info = re_video_info.captures(&resp_body).unwrap();
        
        let index = video_info["video_index"].to_string();
        let id = video_info["video_id"].to_string();
        let title = video_info["video_title"].to_string();
        let title = title.replace("&#8211;", "-");
        
        let link = video_info["video_link"].to_string();
        let format = video_info["video_format"].to_string();

        let video_index_link = format!("{}{}{}{}", link, index, id, format);

        let re_video_index = Regex::new(r".*-(?P<last_part>.*)\.ts\n#EXT-X-ENDLIST").unwrap();
        
        let index_resp = reqwest::get(&video_index_link).await.unwrap();
        let index_resp_body = index_resp.text().await.unwrap();
        let parts = re_video_index.captures(&index_resp_body)
            .unwrap()["last_part"].to_string()
            .parse::<u64>().unwrap();
        
        Video {
            id,
            title,
            link,
            format,
            parts
        }       
    }
    
    fn get_part_download_link(&self, part: u64) -> String {
        format!("{}{}-{}.ts", self.link, self.id, part)
    }

    async fn fetch_video_info(url: String, tx: mpsc::Sender<Video>) {
        let video = Video::new(url).await;
        tx.send(video).unwrap();
    }
}

async fn download_video(video: Video) {
    // Create a video file
    let file_name = format!("./{}.ts", video.title);
    let mut file = File::create(file_name).unwrap();

    let client = reqwest::Client::new();

    println!("Downloading video: {}", video.title);
    for part in 1..=video.parts {
        // Add the part number placement
        let link = video.get_part_download_link(part);

        // Fetch the video as bytes
        let resp = client.get(&link).send().await;
        match resp {
            Ok(r) => {
                match r.status() {
                    reqwest::StatusCode::OK => {
                        let video = r.bytes().await.unwrap();
                        let _ = file.write(video.as_ref()).unwrap();
                    }
                    _ => {
                        break;
                    }
                }
            }
            _ => {
                break;
            }
        }
        // println!("Downloading {}: {:.2}%", part, part as f64 / video.parts as f64);
    }
    println!("Finished downloading video: {}.", video.title);
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let max_work_async: usize = 3;

    let video_queue = Queue::new();

    // Get the cmd args as links to download
    let links: Vec<String> = std::env::args().collect();
    if links.len() == 1 {
        panic!("No links were given.");
    }
    
    for link in links[1..].iter() {
        video_queue.add_work(link).unwrap();
    }
    
    while video_queue.length().unwrap() > 0 {
        let mut current_work = 0;
        let mut tasks = Vec::new();
        while current_work < max_work_async {
            if let Some(link) = video_queue.get_work() {
                let tx_clone = tx.clone();
                let task = tokio::spawn(Video::fetch_video_info(link.clone(), tx_clone));
                tasks.push(task);
                current_work += 1;
            } else {
                break;
            }
        }
        
        // Wait for all the threads to close
        for task in tasks {
            task.await.unwrap();
        }
    }
    
    // Drop tx as it is no longer needed
    // This will allow the channel to close
    // And get all the data in the rx.
    drop(tx);

    let download_queue = Queue::new();
    for video in rx {
        download_queue.add_work(video).unwrap();
    }

    while download_queue.length().unwrap() > 0 {
        let mut current_work = 0;
        let mut tasks = Vec::new();
        
        while current_work < max_work_async {
            if let Some(video) = download_queue.get_work() {
                let task = tokio::spawn(download_video(video));
                tasks.push(task);
                current_work += 1;
            } else {
                break;
            }
        }
        
        // Wait for all the threads to close
        for task in tasks {
            task.await.unwrap();
        }
    }
}
