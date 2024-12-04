// Source: https://github.com/ethanuppal/makemake/blob/main/build.rs

use std::{env, fs, io, path::PathBuf};

use cargo_toml::Manifest;

fn main() -> Result<(), io::Error> {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = PathBuf::from(manifest_dir).join("Cargo.toml");
    let manifest =
        Manifest::from_path(manifest_path).map_err(io::Error::other)?;
    let main_file = manifest
        .lib
        .and_then(|lib| lib.path)
        .or_else(|| manifest.bin.first().and_then(|bin| bin.path.clone()))
        .expect("crate setup as neither library nor binary");
    let main_file_contents = fs::read_to_string(main_file)?;
    let readme_contents = main_file_contents
        .lines()
        .flat_map(|line| {
            if line == "//!" {
                Some("")
            } else {
                line.strip_prefix("//! ")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    fs::write("README.md", readme_contents)?;
    Ok(())
}
