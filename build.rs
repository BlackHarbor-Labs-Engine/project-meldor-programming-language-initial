// Build script for Meldor language compiler

fn main() {
    println!("cargo:rustc-link-search=native=/path/to/native/libs");
    println!("cargo:rustc-link-lib=dylib= meldor_compiler");
}
