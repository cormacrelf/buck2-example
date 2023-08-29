fn main() {
    println!("hello from build.rs");
    println!("cargo:rustc-cfg=bob_the_builder");
}
