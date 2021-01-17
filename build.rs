use std::{env, path::Path};

fn main() {
    if env::var("CARGO_CFG_WINDOWS").is_err() {
        panic!("only Windows is supported");
    }

    let hde = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86" => "hde/hde32.c",
        "x86_64" => "hde/hde64.c",
        _ => panic!("only x86 and x86_64 architectures are supported"),
    };

    println!("cargo:rerun-if-changed=minhook/include");
    println!("cargo:rerun-if-changed=minhook/include/MinHook.h");
    println!("cargo:rerun-if-changed=minhook/src");
    println!("cargo:rerun-if-changed=minhook/src/buffer.c");
    println!("cargo:rerun-if-changed=minhook/src/buffer.h");
    println!("cargo:rerun-if-changed=minhook/src/hde");
    println!("cargo:rerun-if-changed=minhook/src/hde/hde32.c");
    println!("cargo:rerun-if-changed=minhook/src/hde/hde32.h");
    println!("cargo:rerun-if-changed=minhook/src/hde/hde64.c");
    println!("cargo:rerun-if-changed=minhook/src/hde/hde64.h");
    println!("cargo:rerun-if-changed=minhook/src/hde/pstdint.h");
    println!("cargo:rerun-if-changed=minhook/src/hde/table32.h");
    println!("cargo:rerun-if-changed=minhook/src/hde/table64.h");
    println!("cargo:rerun-if-changed=minhook/src/hook.c");
    println!("cargo:rerun-if-changed=minhook/src/trampoline.c");
    println!("cargo:rerun-if-changed=minhook/src/trampoline.h");

    let src_dir = Path::new("minhook/src");

    cc::Build::new()
        .file(src_dir.join("buffer.c"))
        .file(src_dir.join("hook.c"))
        .file(src_dir.join("trampoline.c"))
        .file(src_dir.join(hde))
        .compile("MinHook");
}
