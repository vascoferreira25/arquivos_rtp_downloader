use mylib::video_downloader::VideoDownloader;

// Use a channel to keep data between threads
#[tokio::main]
async fn main() {
    let video_downloader = VideoDownloader::new();

    // Get the cmd args as links to download
    let links: Vec<String> = std::env::args().collect();
    if links.len() == 1 {
        panic!("No links were given.");
    }
    
    for link in links[1..].iter() {
        video_downloader.add_video(link.clone());
    }

    video_downloader.download().await;
}
