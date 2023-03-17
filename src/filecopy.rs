// filecopy.rs
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Copies a file from the source path to the destination path.
///
/// # Arguments
///
/// * `source_path` - A string slice containing the source file path.
/// * `destination_path` - A string slice containing the destination file path.
///
/// # Examples
///
/// ```
/// use schedulite::filecopy::copy_file;
///
/// let result = copy_file("source.txt", "destination.txt");
/// assert!(result.is_ok());
/// ```
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

/// Moves a file from the source path to the destination path.
///
/// # Arguments
///
/// * `source_path` - A string slice containing the source file path.
/// * `destination_path` - A string slice containing the destination file path.
///
/// # Examples
///
/// ```
/// use schedulite::filecopy::move_file;
///
/// let result = move_file("source.txt", "destination.txt");
/// assert!(result.is_ok());
/// ```
pub fn move_file(source_path: &str, destination_path: &str) -> Result<(), Box<dyn Error>> {
    let source_path = Path::new(source_path);
    let destination_path = Path::new(destination_path);

    fs::rename(source_path, destination_path)?;

    Ok(())
}

/// Replaces special variables' placeholders with their corresponding timestamp values.
///
/// The function takes an input string and replaces the placeholders with the current timestamp
/// values formatted according to the specified pattern.
///
/// The following placeholders are supported:
/// * `:{DD}` - Day of the month, zero-padded (01 to 31)
/// * `:{MM}` - Month, zero-padded (01 to 12)
/// * `:{YYYY}` - Year, four digits (e.g., 2021)
/// * `:{HH}` - Hour (24-hour clock), zero-padded (00 to 23)
/// * `:{mm}` - Minute, zero-padded (00 to 59)
///
/// # Arguments
///
/// * `input` - A string slice containing the input text with placeholders to be replaced.
///
/// # Examples
///
/// ```
/// use schedulite::filecopy::replace_special_variables;
///
/// let input = "File created on :{YYYY}-:{MM}-:{DD} at :{HH}:{:mm}";
/// let result = replace_special_variables(input);
/// println!("{}", result); // e.g., "File created on 2023-03-17 at 14:05"
/// ```
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


#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::Local;

    #[test]
    fn test_replace_special_variables() {
        let ts = Local::now();
        let input = ":{YYYY}-:{MM}-:{DD}T:{HH}::{mm}";
        let expected_output = ts.format("%Y-%m-%dT%H:%M").to_string();

        let result = replace_special_variables(input);
        assert_eq!(result, expected_output);
    }
}
