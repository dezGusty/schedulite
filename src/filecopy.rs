// filecopy.rs
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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

/// Replaces special variables' placeholders with their values
/// E.g. ":DD" => Day of month, zero-padded (01 to 31)
/// E.g. ":MM" => Month, zero-padded (01 to 12)
/// E.g. ":YYYY" => Year, four digits (e.g. 2021)
/// E.g. ":HH" => Hour (24-hour clock), zero-padded (00 to 23)
/// E.g. ":mm" => Minute, zero-padded (00 to 59)
pub fn replace_special_variables(input: &str) -> String {
    let ts = chrono::offset::Local::now();

    let result = input
        .replace(":{DD}", &ts.format("%d").to_string())
        .replace(":{MM}", &ts.format("%m").to_string())
        .replace(":{YYYY}", &ts.format("%Y").to_string())
        .replace(":{HH}", &ts.format("%H").to_string())
        .replace(":{mm}", &ts.format("%M").to_string());
    result
}

pub fn copy_file_adv(source_path: &str, destination_path: &str) -> Result<(), Box<dyn Error>> {
    
    // Parse the source and destination paths, replacing special variables' placeholders
    let source_path = replace_special_variables(source_path);
    let destination_path = replace_special_variables(destination_path);

    let source_path = Path::new(&source_path);
    let mut source_file = File::open(source_path)?;
    let mut contents = Vec::new();
    source_file.read_to_end(&mut contents)?;

    let destination_path = Path::new(&destination_path);
    let mut destination_file = File::create(destination_path)?;
    destination_file.write_all(&contents)?;

    Ok(())
}
