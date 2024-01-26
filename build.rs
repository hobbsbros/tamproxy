//! Build Script.
//!
//! This build script compiles the Teensy Loader CLI

use std::process::Command;

fn main() {
    // If the Teensy Loader CLI changes, rerun this build script.
    println!("cargo:rerun-if-changed=teensy_loader_cli/teensy_loader_cli");

    // Make Teensy Loader CLI
    let mut make = Command::new("make");
    make.current_dir("teensy_loader_cli");
    
    match make.spawn() {
        Ok (_) => (),
        Err (_) => println!("cargo:warning=Could not make Teensy Loader CLI"),
    };
}