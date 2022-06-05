use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Clone, Debug)]
enum EntryType{
    ClassicPassword,
    EnvironmentVariable
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PasswordEntry {
    name: String, // like username or variable name
    value: String, //or password
    comment: String,
    entry_type: EntryType,
    last_modified:  DateTime<Local>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config{
    // all the argon props and more
    pub(crate) comment: String,
    pub(crate) author: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DatabaseFile{
    pub(crate) entries: Vec<PasswordEntry>,
    pub(crate) config: Config,
    pub(crate) last_access: DateTime<Local>
}