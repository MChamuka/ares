use crate::scanner::scan_file;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::fs;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch_directory(path: &str) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            tx.send(event).unwrap();
        }
    })?;

    watcher.watch(std::path::Path::new(path), RecursiveMode::Recursive)?;

    println!("👀 Watching folder: {}", path);
    println!("⌛ Will stop if no file is added in 20 seconds.");

    loop {
        // ⏱️ Wait for 20 seconds max for a new event
        match rx.recv_timeout(Duration::from_secs(20)) {
            Ok(event) => {
                if let EventKind::Create(_) = event.kind {
                    for path in event.paths {
                        if path.is_file() {
                            println!("📥 New file detected: {}", path.display());
                            if let Ok(_) = fs::read(&path) {
                                scan_file(&path);
                            }
                        }
                    }
                }
            }
            Err(_) => {
                // Timeout hit – no activity in the last 20 seconds
                println!("⏹️ No file added in 20 seconds. Stopping watcher.");
                break;
            }
        }
    }

    Ok(())
}
