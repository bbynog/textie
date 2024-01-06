use crate::utils::{
    io, 
    file_system
};

#[derive(Debug)]
pub struct FileName {
    pub default: String,
    pub users_input: String
}

#[derive(Debug)]
pub struct TextFile {
    pub name: FileName,
    pub dir: String,
    pub content: String
}

impl TextFile {
    // Associated Functions
    pub fn new() -> TextFile{
        TextFile {
            name: FileName {
                default: String::new(),
                users_input: String::new(),
            },
            dir: String::new(),
            content: String::new(),
        }
    }

    // Methods
    pub fn get_file_name(&self) -> String {
        let file_name = io::ask_user(&self.name.default);
        if file_name.is_empty() {
            return String::new();
        }
    
        file_system::validate_file_name(&file_name, &self.name.default)
    }
}