use std::process::Command;
use std::path::Path;

fn main() {
    // Assemble the assembly file
    let asm_file = "src/neopixelsend.s";
    let obj_file = "target/arm-none-eabi/neopixelsend.o";
    
    let output = Command::new("arm-none-eabi-as")
        .arg("-march=armv7-m")
        .arg("-mfloat-abi=hard")
        .arg("-o")
        .arg(obj_file)
        .arg(asm_file)
        .output()
        .expect("Failed to execute assembler");

    if !output.status.success() {
        println!("Assembler output: {:?}", output.stdout);
        println!("Assembler error: {:?}", output.stderr);
        panic!("Assembler failed");
    }

    // Link the assembly object file
    println!("cargo:rustc-link-search=native={}", Path::new(&obj_file).parent().unwrap().display());
    println!("cargo:rustc-link-lib=dylib=neopixelsend");
}
