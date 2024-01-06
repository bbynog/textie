use chrono::prelude::*;
use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use std::path::Path;

const DEFAULT_OUTPUT_DIR: &'static str = include_str!("./constants.txt");
const PRETTY_TIME_FORMAT: &'static str = "%a - %d %b %Y - %T";
const FILE_NAME_TIME_FORMAT: &'static str = "%d-%m-%Y_%H:%M:%S";

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct FileName {
    default: String,
    users_input: String
}

#[derive(Debug)]
struct TextFile {
    name: FileName,
    dir: String,
    content: String
}

impl TextFile {
    fn build_file() -> TextFile{
        TextFile {
            name: FileName {
                default: String::new(),
                users_input: String::new(),
            },
            dir: String::new(),
            content: String::new(),
        }
    }
}

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

fn file_exists(file_name: &str) -> bool {
    let path = format!("{DEFAULT_OUTPUT_DIR}/{file_name}.txt");

    let path = Path::new(&path);
    path.exists()
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

fn get_file_name(default_file_name: &str) -> String {
    let file_name = ask_user(default_file_name);
    if file_name.is_empty() {
        return String::new();
    }

    validate_file_name(&file_name, &default_file_name)
}

fn create_and_write_file(file: &TextFile) -> Result<String> {
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
    let mut text_file = TextFile::build_file();

    text_file.name.default = Local::now().format(FILE_NAME_TIME_FORMAT).to_string();
    text_file.dir = get_output_dir();
    text_file.name.users_input = get_file_name(&text_file.name.default);

    let file_name = create_and_write_file(&text_file)?;
    println!("{:#?}", text_file);

    open_file(&text_file.dir, &file_name)?;

    Ok(())
}
