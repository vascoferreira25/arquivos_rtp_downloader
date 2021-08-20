use std::sync::mpsc;

use super::queue::Queue;
use super::video::Video;

pub struct VideoDownloader {
    links_queue: Queue<String>,
    video_queue: Queue<Video>,
    max_work_async: usize
}

impl VideoDownloader {
    pub fn new() -> VideoDownloader {
        let links_queue: Queue<String> = Queue::new();
        let video_queue: Queue<Video> = Queue::new();
        let max_work_async: usize = 3;

        VideoDownloader {
            links_queue,
            video_queue,
            max_work_async
        }
    }

    pub fn add_video(&self, link: String) {
        self.links_queue.add_work(link).unwrap();
    }

    pub async fn download(&self) {
        let (tx, rx) = mpsc::channel();

        // Fetch video information
        while self.links_queue.length().unwrap() > 0 {
            let mut current_work: usize = 0;
            let mut tasks = Vec::new();
            while current_work < self.max_work_async {
                if let Some(link) = self.links_queue.get_work() {
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

        // Download video
        for video in rx {
            self.video_queue.add_work(video).unwrap();
        }

        while self.video_queue.length().unwrap() > 0 {
            let mut current_work: usize = 0;
            let mut tasks = Vec::new();
            
            while current_work < self.max_work_async {
                if let Some(video) = self.video_queue.get_work() {
                    let task = tokio::spawn(Video::download_video(video));
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
}

