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

    // panic!("{}-{}-{}", target_arch, target_family, target_os);

    let mut build = cc::Build::new();
    let mut libs = Vec::from(["psapi", "shell32", "user32", "advapi32", "bcrypt", "ws2_32"]);

    build.cpp(true);
    // build.flag("-std=c++2c");
    build.flag("-D_CRT_SECURE_NO_WARNINGS");
    build.flag("-ferror-limit=0");

    build.include("src_cpp");
    build.include("src_cpp/sdk");

    build.file("src_cpp/Main.cpp");
    println!("cargo:rerun-if-changed=src_cpp");
    println!("cargo:rerun-if-changed=src_cpp/sdk");

    match target_arch.as_str() {
        "x86" | "i686" | "i586" => {
            // libs.push("lib/minhook/libMinHook.x86");
            // build.include("lib/minhook");

            libs.push("lib/detours/detours.x86");
            build.include("lib/detours");
        }
        "x86_64" => {
            libs.push("lib/detours/detours.x64");
            build.include("lib/detours");
        }

        _ => (),
    }

    build.compile("aisp_hook");

    // search in current directory
    println!("cargo:rustc-link-search={}", crate_directory);
    for lib in libs {
        println!("cargo:rustc-link-lib={}", lib);
    }
    // println!("cargo::rustc-link-arg=-verbose");
}

// fn update_xwin() {
//     let xwin_path = "";
//
//     use ureq;
//     use xwin;
//
//     let ctx = xwin::Ctx::with_dir(
//         xwin::PathBuf::from(".xwin-cache/compile-test"),
//         xwin::util::ProgressTarget::Hidden,
//         ureq::agent(),
//         0,
//     )
//     .unwrap();
//
//     let ctx = std::sync::Arc::new(ctx);
//
//     let hidden = indicatif::ProgressBar::new_spinner();
//
//     let manifest = xwin::manifest::get_manifest(&ctx, "17", "release", hidden.clone()).unwrap();
//     let pkg_manifest =
//         xwin::manifest::get_package_manifest(&ctx, &manifest, hidden.clone()).unwrap();
//
//     let arches = (xwin::Arch::X86_64 as u32) | (xwin::Arch::X86 as u32);
//
//     let pruned = xwin::prune_pkg_list(
//         &pkg_manifest,
//         arches,
//         xwin::Variant::Desktop as u32,
//         false,
//         false,
//         None,
//         None,
//     )
//     .unwrap();
//
//     let op = xwin::Ops::Splat(xwin::SplatConfig {
//         include_debug_libs: false,
//         include_debug_symbols: false,
//         enable_symlinks: true,
//         preserve_ms_arch_notation: false,
//         use_winsysroot_style: false,
//         map: None,
//         copy: true,
//         output: xwin::PathBuf::from(xwin_path),
//     });
//
//     ctx.execute(
//         pkg_manifest.packages.clone(),
//         pruned
//             .payloads
//             .clone()
//             .into_iter()
//             .map(|payload| xwin::WorkItem {
//                 progress: hidden.clone(),
//                 payload: std::sync::Arc::new(payload),
//             })
//             .collect(),
//         pruned.crt_version.clone(),
//         pruned.sdk_version.clone(),
//         pruned.vcr_version.clone(),
//         arches,
//         xwin::Variant::Desktop as u32,
//         op,
//     )
//     .unwrap();
// }
