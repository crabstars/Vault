use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum EntryType{
    ClassicPassword,
    EnvironmentVariable
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PasswordEntry {
    pub id: String,
    pub title: String,
    pub name: String, // like username or variable name
    pub value: String, //or password
    pub url: String,
    pub comment: String,
    pub entry_type: EntryType,
    pub last_modified:  DateTime<Local>
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
    pub(crate) last_access: DateTime<Local>,
    pub(crate) password: String
}