use crate::prelude::*;

use crate::utils::{
    io,
    file_system
};

mod error;
mod prelude;
mod utils;

use chrono::prelude::Local;

use utils::text_file::TextFile;


fn main() -> Result<()> {
    let mut text_file = TextFile::new();

    text_file.name.default = Local::now().format(FILE_NAME_TIME_FORMAT).to_string();
    text_file.dir = io::get_output_dir();
    text_file.name.users_input = text_file.get_file_name();

    let file_name = file_system::create_and_write_file(&text_file)?;
    println!("{:#?}", text_file);

    file_system::open_file(&text_file.dir, &file_name)?;

    Ok(())
}
