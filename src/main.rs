#![feature(fs_try_exists)]

use chrono::prelude::*;
use std::error;
use std::fs::{File, try_exists};
use std::io::prelude::*;
use std::process::Command;

const DEFAULT_OUTPUT_DIR: &'static str = include_str!("./constants.txt");

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

type IoResult<T> = std::result::Result<T, std::io::Error>;

fn get_output_dir() -> String {
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
        update_output_dir(output_dir).expect("Failed to update output dir");
        return output_dir.to_string();
    }
}

fn file_exists(file_name: &str) -> IoResult<bool> {
    match try_exists(format!("{DEFAULT_OUTPUT_DIR}/{file_name}.txt")) {
        Ok(true) => Ok(true),
        Ok(false) => Ok(false),
        Err(e) => Err(e)
    }
}

fn ask_user(default_file_name: &str) -> String {
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

fn validate_file_name(users_input: &str, default_file_name: &str) -> String {
    let mut is_first_iteration = true;
    loop {
        let file_name: String = if is_first_iteration {
            is_first_iteration = false;
            users_input.to_string()
        } else {
            ask_user(default_file_name)
        };

        if file_name.is_empty() {
            return default_file_name.to_string()
        }

        match file_exists(&file_name.to_string()) {
            Ok(true) => {
                println!("\nFile already exists on the specified dir! Try another name");
                continue
            },
            Ok(false) => { 
                break file_name.to_string()
            },
            Err(e) => panic!("Error on checkin dir, {}", e),
        }
    }
}

fn get_file_name(default_file_name: &str) -> String {
    let file_name = ask_user(default_file_name);
    if file_name.is_empty() {
        return String::new();
    }

    validate_file_name(&file_name, &default_file_name)
}

fn create_and_write_file(output_dir: &str, input_file_name: &str, default_file_name: &str, content: &str) -> Result<String> {
    let current_time_formatted = Local::now().format("%a - %d %b %Y - %T");

    let file_name;
    let txt_content = if input_file_name.is_empty() {
        file_name = String::from(format!("{}", default_file_name));
        format!("{}\n{}\n", current_time_formatted, content)
    } else {
        file_name = String::from(format!("{}", input_file_name));
        format!("{} - {}\n{}\n", input_file_name, current_time_formatted, content)
    };
    
    let mut text_file =
        File::create(format!("{}/{}.txt", output_dir, file_name)).expect("Failed to create file");

    text_file.write_all(txt_content.as_bytes())?;

    Ok(String::from(file_name))
}

fn open_file(output_dir: &str, file_name: &str) -> Result<()> {
    Command::new("open")
        .arg(format!("{}/{}.txt", output_dir, file_name))
        .spawn()?;

    println!("File created successfully at {output_dir}/{file_name}.txt");

    Ok(())
}
fn update_output_dir(output_dir: &str) -> Result<()> {
    std::fs::write("./src/constants.txt", output_dir)?;
    Ok(())
}

fn main() -> Result<()> {
    let output_dir = get_output_dir();

    let default_file_name = Local::now().format("%d-%m-%Y_%H:%M:%S").to_string();
    let file_name = get_file_name(&default_file_name);

    let file_name = create_and_write_file(&output_dir, &file_name, &default_file_name, "")?;
    open_file(&output_dir, &file_name)?;

    Ok(())
}
