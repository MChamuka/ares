extern "C" {
    fn start_sandbox();
}

fn main() {
    println!("Starting virus guard...");
    unsafe {
        start_sandbox();
    }
}
