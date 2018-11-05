extern crate rustc_version;

#[macro_use]
extern crate bitflags;

extern crate core_extensions;

use rustc_version::{version, Version};

pub mod test;

fn main() {
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

    println!("cargo:warning=RERUNNING BUILD SCRIPT");

    println!("cargo:rerun-if-changed=build");

    const RERUNNING_BUILD_SCRIPT:()=();

    self::test::build_tests().unwrap();
}
