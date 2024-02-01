use anyhow::Result;
use std::{env::current_exe, fs::remove_file};

pub fn disappear() -> Result<()> {
    let filename = current_exe()?;
    let _ = remove_file(filename);
    Ok(())
}
