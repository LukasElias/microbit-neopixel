use std::process::Command;
use std::path::Path;

fn main() {
    // Assemble the assembly file
    let asm_file = "src/neopixelsend.s";
    let obj_file = "target/arm-none-eabi/libneopixelsend.o";
    let staticlib_file = "target/arm-none-eabi/libneopixelsend.a";
    
    let assembler_output = Command::new("arm-none-eabi-as")
        .arg("-march=armv7-m")
        .arg("-mfloat-abi=hard")
        .arg("-o")
        .arg(obj_file)
        .arg(asm_file)
        .output()
        .expect("Failed to execute assembler");

    if !assembler_output.status.success() {
        println!("Assembler output: {:?}", assembler_output.stdout);
        println!("Assembler error: {:?}", assembler_output.stderr);
        panic!("Assembler failed");
    }

    let archive_output = Command::new("ar")
        .arg("rcs")
        .arg(staticlib_file)
        .arg(obj_file)
        .output()
        .expect("Failed to execute archiver");

    if !archive_output.status.success() {
        println!("Archiver output: {:?}", archive_output.stdout);
        println!("Archiver error: {:?}", archive_output.stderr);
        panic!("Archiver failed");
    }

    // Link the assembly object file
    println!("cargo:rustc-link-search=native={}", Path::new(&staticlib_file).parent().unwrap().display());
    println!("cargo:rustc-link-lib=static=neopixelsend");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=neopixelsend.s");
}
