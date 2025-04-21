use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use std::collections::HashSet;
use std::io::{BufRead, BufReader};

pub fn scan_path(path: &str) {
    println!("📁 Scanning: {}", path);
    let virus_hashes = load_virus_hashes("virus_hashes.txt");

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            match fs::read(path) {
                Ok(content) => {
                    let mut hasher = Sha256::new();
                    hasher.update(&content);
                    let result = hasher.finalize();
                    let hash = hex::encode(result);

                    if virus_hashes.contains(&hash) {
                        println!("🚨 VIRUS FOUND in {} ❗", path.display());
                    } else {
                        println!("✅ Scanned {} -> {}", path.display(), hash);
                    }
                }
                Err(e) => {
                    println!("⚠️ Failed to read {}: {}", path.display(), e);
                }
            }
        }
    }
}

fn load_virus_hashes(file_path: &str) -> HashSet<String> {
    let file = fs::File::open(file_path).expect("⚠️ Could not open virus hash list!");
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}
