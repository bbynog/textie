use chrono::prelude::*;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

const DEFAULT_OUTPUT_DIR: &'static str = include_str!("./constants.txt");

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

fn get_file_name(default_file_name: &str) -> String {
    let mut file_name = String::new();

    println!(
        "What's the name of the txt file? (DEFAULT: {})",
        default_file_name
    );
    std::io::stdin()
        .read_line(&mut file_name)
        .expect("Failed to read input");

    let file_name = file_name.trim();

    if file_name.is_empty() {
        default_file_name.to_string()
    } else {
        file_name.to_string()
    }
}

fn create_and_write_file(output_dir: &str, file_name: &str, content: &str) -> Result<()> {
    let current_time_formatted = Local::now().format("%a - %d %b %Y - %T");
    let txt_content = format!("{} {}\n", content, current_time_formatted);

    let mut text_file =
        File::create(format!("{}/{}.txt", output_dir, file_name)).expect("Failed to create file");

    text_file.write_all(txt_content.as_bytes())?;

    Ok(())
}

fn open_file(output_dir: &str, file_name: &str) -> Result<()> {
    Command::new("open")
        .arg(format!("{}/{}.txt", output_dir, file_name))
        .spawn()?;

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

    create_and_write_file(&output_dir, &file_name, "")?;
    open_file(&output_dir, &file_name)?;

    Ok(())
}
