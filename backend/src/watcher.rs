use crate::scanner::scan_file;
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use std::fs;
use std::sync::mpsc::channel;

pub fn watch_directory(path: &str) -> Result<()> {
    let (tx, rx) = channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        if let Ok(event) = res {
            tx.send(event).unwrap();
        }
    })?;

    watcher.watch(std::path::Path::new(path), RecursiveMode::Recursive)?;

    println!("👀 Watching folder: {}", path);

    for event in rx {
        match event.kind {
            EventKind::Create(_) => {
                for path in event.paths {
                    if path.is_file() {
                        println!("📥 New file detected: {}", path.display());
                        if let Ok(data) = fs::read(&path) {
                            scan_file(&path);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
