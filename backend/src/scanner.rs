use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn scan_file_and_log(path: &Path) {
    let virus_hashes = load_virus_hashes("virus_hashes.txt");

    if path.is_file() {
        match fs::read(path) {
            Ok(content) => {
                let mut hasher = Sha256::new();
                hasher.update(&content);
                let result = hasher.finalize();
                let hash = hex::encode(result);

                let status;
                if virus_hashes.contains(&hash) {
                    println!("🚨 VIRUS FOUND in {} ❗", path.display());
                    status = "VIRUS FOUND";
                } else {
                    println!("✅ Scanned {} -> {}", path.display(), hash);
                    status = "CLEAN";
                }

                log_scan_result(path, &hash, status);
            }
            Err(e) => {
                println!("⚠️ Failed to read {}: {}", path.display(), e);
            }
        }
    }
}

fn log_scan_result(path: &Path, hash: &str, status: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("scan_logs.csv")
        .expect("Could not open scan_logs.csv");

    let log_entry = format!("{},{},{}\n", path.display(), hash, status);
    file.write_all(log_entry.as_bytes())
        .expect("Failed to write log");
}

fn load_virus_hashes(file_path: &str) -> HashSet<String> {
    let file = fs::File::open(file_path).expect("⚠️ Could not open virus hash list!");
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}
pub fn show_scan_history() {
    match fs::read_to_string("scan_logs.csv") {
        Ok(content) => {
            println!("📄 Scan History:");
            for line in content.lines() {
                println!("{}", line);
            }
        }
        Err(_) => {
            println!("⚠️ No scan history found.");
        }
    }
}
