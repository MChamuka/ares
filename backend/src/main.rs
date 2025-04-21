mod scanner;

fn main() {
    let folder_to_scan = "sample_files"; // Change this to any folder path
    scanner::scan_path(folder_to_scan);
}
