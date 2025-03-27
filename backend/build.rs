fn main() {
    println!("cargo:rerun-if-changed=cpp/sandbox.cpp");
    cc::Build::new()
        .cpp(true)
        .file("cpp/sandbox.cpp")
        .compile("sandbox");
}
