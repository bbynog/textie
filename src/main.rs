use std::fs::File;
use std::io::prelude::*;
use chrono::prelude::*;
use std::process::Command;
use std::error;

const DEFAULT_OUTPUT_DIR: &'static str = include_str!("./constants.txt");

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn update_output_dir(output_dir: &str) -> Result<()> {
    std::fs::write("./src/constants.txt", output_dir)?;
    Ok(())
}

fn main() -> Result<()> {
    let mut file_name = String::new();
    
    let current_time = Local::now();

    let default_file_name = current_time.format("%d-%m-%Y_%H:%M:%S").to_string();

    let mut output_dir = String::new();

    println!("Where you'd like to save the file? (DEFAULT: {DEFAULT_OUTPUT_DIR})");
    std::io::stdin().read_line(&mut output_dir).unwrap();

    let mut output_dir = output_dir.trim();

    if output_dir.is_empty() {
        output_dir = DEFAULT_OUTPUT_DIR;
    } else {
        update_output_dir(output_dir)?;
    }

    println!("What's the name of the txt file? (DEFAULT: {default_file_name})");
    std::io::stdin().read_line(&mut file_name).unwrap();

    let mut file_name = file_name.trim();

    let current_time_formatted = current_time.format("%a - %d %b %Y - %T");

    let mut txt_content = format!("{current_time_formatted} \n");

    if file_name.is_empty() {
        file_name = &default_file_name;
    } else {
        txt_content = format!("{file_name} - {current_time_formatted}\n");
    }

    let mut text_file = File::create(format!("{output_dir}/{file_name}.txt")).unwrap();
    
    text_file.write_all(txt_content.as_bytes()).unwrap();

    Command::new("open")
            .arg(format!("{output_dir}/{file_name}.txt"))
            .spawn()
            .unwrap();

    Ok(())
}
