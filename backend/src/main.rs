mod scanner;
mod watcher;

fn main() {
    let path_to_watch = "sample_files"; // or your Downloads folder
    if let Err(e) = watcher::watch_directory(path_to_watch) {
        println!("❌ Failed to watch directory: {:?}", e);
    }
}
