mod scanner;
mod watcher;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "scan" if args.len() == 3 => {
                let path = Path::new(&args[2]);
                if path.exists() && path.is_file() {
                    scanner::scan_file_and_log(path);
                } else {
                    println!(
                        "❌ File does not exist or is not a file: {}",
                        path.display()
                    );
                }
            }
            "scan-folder" if args.len() == 3 => {
                let dir = Path::new(&args[2]);
                if dir.exists() && dir.is_dir() {
                    for entry in dir.read_dir().expect("Failed to read folder") {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if path.is_file() {
                                scanner::scan_file_and_log(&path);
                            }
                        }
                    }
                } else {
                    println!(
                        "❌ Folder does not exist or is not a directory: {}",
                        dir.display()
                    );
                }
            }
            _ => {
                println!("❗ Invalid command. Use:");
                println!("  cargo run -- scan <file_path>");
                println!("  cargo run -- scan-folder <folder_path>");
            }
        }
    } else {
        // fallback to auto watcher mode
        let path_to_watch = "sample_files";
        if let Err(e) = watcher::watch_directory(path_to_watch) {
            println!("❌ Failed to watch directory: {:?}", e);
        }
    }
}
