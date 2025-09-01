// some credits goes to https://github.com/purpleprotocol/mimalloc_rust/blob/master/libmimalloc-sys/build.rs

use std::env;

fn main() {
    let crate_directory = env::var("CARGO_MANIFEST_DIR").expect("manifest_dir not defined");

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("target_os not defined!");
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").expect("target_family not defined!");
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").expect("target_arch not defined!");

    // this is windows only
    if target_os != "windows" {
        return;
    }

    // this is 32 bit only
    match target_arch.as_str() {
        "x86" | "i686" | "i586" => {}

        _ => panic!(
            "unsupported architecture {}. this only works on 32 bit!",
            target_arch
        ),
    }
}
