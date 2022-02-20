fn main() {
    // Compile cheri_compressed_cap.c
    // Based on https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a
    let src = [
        "../../cheri_compressed_cap.c"
    ];
    let mut builder = cc::Build::new();
    let build = builder
        .files(src.iter())
        .include("include");
    build.compile("cheri_compressed_cap");
}