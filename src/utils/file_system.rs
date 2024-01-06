use crate::prelude::*;

use chrono::prelude::Local;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::path::Path;

use crate::utils::{
    text_file::TextFile, 
    io
};

pub fn file_exists(file_name: &str) -> bool {
    let path = format!("{DEFAULT_OUTPUT_DIR}/{file_name}.txt");

    let path = Path::new(&path);
    path.exists()
}

pub fn create_and_write_file(file: &TextFile) -> Result<String> {
    let current_time_formatted = Local::now().format(PRETTY_TIME_FORMAT);

    let file_name;
    let txt_content = if file.name.users_input.is_empty() {
        file_name = String::from(format!("{}", file.name.default));
        format!("{}\n{}\n", current_time_formatted, file.content)
    } else {
        file_name = String::from(format!("{}", file.name.users_input));
        format!("{} - {}\n{}\n", file.name.users_input, current_time_formatted, file.content)
    };
    
    let mut text_file =
        File::create(format!("{}/{}.txt", file.dir, file_name)).expect("Failed to create file");

    text_file.write_all(txt_content.as_bytes())?;

    Ok(String::from(file_name))
}

pub fn open_file(output_dir: &str, file_name: &str) -> Result<()> {
    Command::new("open")
        .arg(format!("{}/{}.txt", output_dir, file_name))
        .spawn()?;

    println!("File created successfully at {output_dir}/{file_name}.txt");

    Ok(())
}

pub fn update_output_dir(output_dir: &str) -> Result<()> {
    std::fs::write(CONSTANT_PATH, output_dir)?;
    Ok(())
}

pub fn validate_file_name(users_input: &str, default_file_name: &str) -> String {
    let mut is_first_iteration = true;
    loop {
        let file_name: String = if is_first_iteration {
            is_first_iteration = false;
            users_input.to_string()
        } else {
            io::ask_user(default_file_name)
        };

        if file_name.is_empty() {
            return default_file_name.to_string()
        }

        match file_exists(&file_name.to_string()) {
            true => {
                println!("\nFile already exists on the specified dir! Try another name");
                continue
            },
            false => { 
                break file_name.to_string()
            },
        }
    }
}    
