use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Serialize, Deserialize, Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub enum EntryType{
    ClassicPassword,
    EnvironmentVariable
}

#[derive(Serialize, Deserialize, Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct PasswordEntry {
    pub id: String,
    pub title: String,
    pub name: String, // like username or variable name
    pub value: String, //or password
    pub url: String,
    pub comment: String,
    pub entry_type: EntryType,
    pub last_modified: String,
    pub files: Vec<CustomFile>
}

#[derive(Serialize, Deserialize, Clone, Debug, Zeroize, ZeroizeOnDrop)]
pub struct CustomFile {
    pub content: String, //Base64 encoding
    pub comment: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config{
    // maybe all the argon props and more
    pub(crate) comment: String,
    pub(crate) author: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseFile{
    pub(crate) entries: Vec<PasswordEntry>,
    pub(crate) config: Config,
    pub(crate) last_access: DateTime<Local>,
    pub(crate) password: String
}
