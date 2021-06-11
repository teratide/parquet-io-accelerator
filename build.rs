use reqwest::blocking;
use std::{env, fs::File, path::PathBuf, process::Command};

const PARQUET_FORMAT_VERSION: &str = "2.9.0";
const PARQUET_FORMAT_THRIFT_FILE: &str = "parquet.thrift";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only re-run build if this file changed.
    println!("cargo:rerun-if-changed=build.rs");

    // Download parquet.thrift file.
    let path = PathBuf::from(env::var("OUT_DIR").unwrap()).join(PARQUET_FORMAT_THRIFT_FILE);
    blocking::get(format!(
        "https://raw.githubusercontent.com/apache/parquet-format/apache-parquet-format-{}/src/main/thrift/{}",
        PARQUET_FORMAT_VERSION, PARQUET_FORMAT_THRIFT_FILE
    ))
        .expect("parquet.thrift file download failed")
        .copy_to(&mut File::create(&path)?)?;

    // Run Thrift compiler.
    Command::new("thrift")
        .arg("--gen")
        .arg("rs")
        .arg("-out")
        .arg("src/metadata")
        .arg(path)
        .output()?;

    Ok(())
}
