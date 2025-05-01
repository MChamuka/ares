use crate::scanner::scan_file_and_log;
use notify::{EventKind, RecursiveMode, Result, Watcher};
use std::fs;
use std::sync::mpsc::channel;

const ALLOWED_EXTENSIONS: [&str; 4] = ["docx", "pdf", "exe", "zip"];

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
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            if ALLOWED_EXTENSIONS.contains(&ext) {
                                println!("📥 New file detected: {}", path.display());
                                if let Ok(data) = fs::read(&path) {
                                    scan_file_and_log(&path);
                                }
                            } else {
                                println!("⚠️ Skipped unsupported file: {}", path.display());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(())
}
