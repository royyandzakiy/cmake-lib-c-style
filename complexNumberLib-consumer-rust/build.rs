fn main() {
    println!("cargo:rustc-link-search=native=../generated_libs/complexNumber/lib");
    println!("cargo:rustc-link-lib=complexNumbers");
    println!("cargo:rerun-if-changed=../generated_libs/complexNumber/include/complexNumberLib/complexNumbers.h");
}