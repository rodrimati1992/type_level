extern crate rustc_version;

use rustc_version::{version, Version};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let rver = version().unwrap();

    if Version::new(1, 22, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_22");
    }
    if Version::new(1, 26, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_26");
    }
    if Version::new(1, 27, 0) <= rver {
        println!("cargo:rustc-cfg=rust_1_27");
    }

    println!("cargo:warning=RERUNNING type_level_examples BUILD SCRIPT");
}
