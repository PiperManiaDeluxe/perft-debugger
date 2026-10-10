use std::{path::PathBuf, process::Command};

use anyhow::{Context, Result};
use which::which;

pub const HEADER_PATH: &str = "include/perft_debugger.h";

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=cbindgen.toml");

    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Generate C header
    let header_out_path = crate_dir.join(HEADER_PATH);

    cbindgen::generate(&crate_dir)
        .expect("unable to generate C bindings")
        .write_to_file(header_out_path);

    // Format the generated header
    if let Ok(format_exec_path) = which("clang-format") {
        let _subprocess = Command::new(format_exec_path)
            .current_dir(crate_dir)
            .args(["--style=file", "-i", HEADER_PATH])
            .spawn()
            .context("Failed to spawn child clang-format process")?;
    }

    Ok(())
}
