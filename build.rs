fn main() {
    #[cfg(feature = "v1")]
    cc::Build::new()
        .file("src/neopixelsend.s")
        .compile("neopixelsend");

    #[cfg(feature = "v2")]
    cc::Build::new()
        .file("src/neopixelsendv2.s")
        .compile("neopixelsend");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=neopixelsend.s");
    println!("cargo:rerun-if-changed=neopixelsendv2.s");
}
