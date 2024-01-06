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
}