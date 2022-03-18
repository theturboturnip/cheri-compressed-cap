fn main() {
    println!("cargo:rerun-if-changed=../../cheri_compressed_cap.h");
    println!("cargo:rerun-if-changed=../../cheri_compressed_cap_lib.c");
    println!("cargo:rerun-if-changed=../../cheri_compressed_cap_common.h");
    println!("cargo:rerun-if-changed=../../cheri_compressed_cap_macros.h");
    println!("cargo:rerun-if-changed=../../cheri_compressed_cap_64.h");
    println!("cargo:rerun-if-changed=../../cheri_compressed_cap_128.h");

    // Compile cheri_compressed_cap.c
    cc::Build::new()
        .file("../../cheri_compressed_cap_lib.c")
        .flag_if_supported("-Wno-unused-function")
        .compile("cheri_compressed_cap_lib");
}