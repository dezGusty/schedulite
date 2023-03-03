use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;

pub fn copy_file(source_path: &str, destination_path: &str) -> Result<(), Box<dyn Error>> {
    let source_path = Path::new(source_path);
    let mut source_file = File::open(source_path)?;
    let mut contents = Vec::new();
    source_file.read_to_end(&mut contents)?;

    let destination_path = Path::new(destination_path);
    let mut destination_file = File::create(destination_path)?;
    destination_file.write_all(&contents)?;

    Ok(())
}
