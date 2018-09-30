extern crate rustc_version;
use rustc_version::{version, Version};

fn main() {
    let rver = version().unwrap();

    if Version::new(1, 22, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_22");
    }
}
