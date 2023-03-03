pub mod filecopy;

fn main() {
    // Copy a simple file
    let source_path = String::from("./data/test-input.txt");
    let destination_path = String::from("./data/test-output.txt");
    let result = filecopy::copy_file(source_path.as_str(), destination_path.as_str());

    match result {
        Ok(_) => println!("File copied successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
