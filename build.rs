//! Copies `memory.x` into the linker search path.
//!
//! `rustc`/`ld` already search the crate root for it, so this is redundant
//! for this single-crate layout — but it's cheap, matches the convention
//! used throughout the embedded Rust ecosystem (cortex-m-rt, Discovery,
//! nrf-rs/microbit), and keeps this scaffold copy-pasteable into a
//! workspace later without silently losing the linker script.

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");
}
