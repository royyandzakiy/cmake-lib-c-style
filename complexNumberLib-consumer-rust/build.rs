fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/lib");
    println!("cargo:rustc-link-lib=complexNumbers");
    println!("cargo:rerun-if-changed=src/wrapper.h");
}