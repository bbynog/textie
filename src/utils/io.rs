use crate::prelude::*;
use crate::utils::file_system;

pub fn get_output_dir() -> String {
    let mut output_dir = String::new();

    println!(
        "Where would you like to save the file? (DEFAULT: {})",
        DEFAULT_OUTPUT_DIR
    );
    std::io::stdin()
        .read_line(&mut output_dir)
        .expect("Failed to read input");

    let output_dir = output_dir.trim();

    if output_dir.is_empty() {
        String::from(DEFAULT_OUTPUT_DIR)
    } else {
        file_system::update_output_dir(output_dir).expect("Failed to update output dir");
        return output_dir.to_string();
    }
}

pub fn ask_user(default_file_name: &str) -> String {
    println!(
        "What's the name of the txt file? (DEFAULT: {})",
        default_file_name
    );

    let mut file_name = String::new();

    std::io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");

    file_name.trim().to_string()
}

pub fn get_file_name(default_file_name: &str) -> String {
    let file_name = ask_user(default_file_name);
    if file_name.is_empty() {
        return String::new();
    }

    file_system::validate_file_name(&file_name, default_file_name)
}
