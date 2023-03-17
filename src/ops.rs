use log::{error, info};

use crate::filecopy;

pub async fn async_copy_op(source_path: &str, destination_path: &str) {
    // Parse the source and destination paths, replacing special variables' placeholders
    let source_path = filecopy::replace_special_variables(source_path);
    let destination_path = filecopy::replace_special_variables(destination_path);

    let result = filecopy::copy_file(&source_path, &destination_path);
    match result {
        Ok(_) => info!(
            "File copied successfully 📜({} -> {})",
            source_path, destination_path
        ),
        Err(e) => error!(
            "Error: {}, src={}, dest={}",
            e, source_path, destination_path
        ),
    }
}

pub async fn async_move_op(source_path: &str, destination_path: &str) {
    // Parse the source and destination paths, replacing special variables' placeholders
    let source_path = filecopy::replace_special_variables(source_path);
    let destination_path = filecopy::replace_special_variables(destination_path);

    let result = filecopy::move_file(&source_path, &destination_path);
    match result {
        Ok(_) => info!(
            "File moved successfully 📜({} -> {})",
            source_path, destination_path
        ),
        Err(e) => error!(
            "Error: {}, src={}, dest={}",
            e, source_path, destination_path
        ),
    }
}
