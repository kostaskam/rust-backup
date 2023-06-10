use std::path::Path;

/* 
    Parameters: 
        key: String,
        source_path: &Path

    Description: 
        Below function prints an error message about incorrect values.
*/
pub fn display_wrong_path_error(key: String, source_path: &Path) {
    
    let error_message = format!(" Make sure {key} value is correct and exists: ");
    let error_message_len = error_message.len();
    let source_path_len = source_path.display().to_string().len();
    let total_str_size = error_message_len+source_path_len+5;
    let row_with_dashes = (0..total_str_size).map(|_| "-").collect::<String>();

    println!("\n\n{}", row_with_dashes);
    println!("|{}{:?} |", error_message, source_path);
    println!("{}", row_with_dashes);
}