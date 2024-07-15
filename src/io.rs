use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::error::Error;

pub fn read_source_file(path: &Path) -> Result<String, Error> {
    let mut f = File::open(path)?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}
