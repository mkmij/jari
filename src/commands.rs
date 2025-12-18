use anyhow::Result;
use std::env::current_dir;
use std::fs::{exists, DirBuilder};
use std::path::Path;

pub fn handle_help() -> String {
    String::from("Hello, world!")
}

pub fn handle_init() -> Result<()> {
    let current_dir = current_dir()?;
    let jari_dir = Path::new(current_dir.as_path()).join(".jari");
    if !exists(jari_dir)? {
        DirBuilder::new().create(".jari")?
    }
    Err(std::io::Error::last_os_error().into())
}
