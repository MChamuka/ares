use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn scan_file(path: &Path) {
    let virus_hashes = load_virus_hashes("virus_hashes.txt");

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

fn load_virus_hashes(file_path: &str) -> HashSet<String> {
    let file = fs::File::open(file_path).expect("⚠️ Could not open virus hash list!");
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}
