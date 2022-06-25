use super::enums::*;

pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub message: Vec<String>,
    /// Current index place from messages
    pub input_index: usize
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Navigation,
            message: Vec::new(),
            input_index: 0 // 1 is the first char
        }
    }
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::PasswordEntries => 1,
            MenuItem::SelctedEntry => 2,
        }
    }
}