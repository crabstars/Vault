use std::path::PathBuf;
use std::env;
use anyhow::Ok;
use chrono::Local;
use uuid::Uuid;

use crate::New;
use crate::database::structures::{Config, DatabaseFile, EntryType};
use crate::encryption_and_decryption::argon::{decrypt_text, encrypt_text};
use crate::utils::terminal_interactions::{prompt_user, prompt_password};

use super::structures::PasswordEntry;

pub trait Database {
    fn new(path: &mut Option<PathBuf>, database_name: &str) -> Result<Box<Self>, anyhow::Error>;
    fn add_empty_entry(&mut self) -> String;
    fn remove_entry_by_id(&mut self, id: String) -> bool;
    fn save_database(&self, path: &Option<PathBuf>)-> Result<(), anyhow::Error>; 
    fn get_value_from_selected_detail(&self, index_detail: usize, id: String) -> String;
    fn update_entry(&mut self, index_detail: usize, id: String, message: Vec<String>);
    fn get_entry_by_id(&self, id: String) -> Option<&PasswordEntry>;
}

impl Database for DatabaseFile{
    
    fn new(path: &mut Option<PathBuf>, database_name: &str) -> Result<Box<DatabaseFile>, anyhow::Error>{

        if path.is_none(){
            *path = Some(env::current_dir()?.join(database_name));
        }
        
        let password = rpassword::prompt_password("Please enter the password:")?;
        let text = decrypt_text(path.as_ref().unwrap_or(&PathBuf::new().join(database_name.to_owned()+".vault")), &password)?;
        let mut db: DatabaseFile = serde_json::from_str(&text)?;
        db.last_access = Local::now();

        Ok(Box::new(db))
    } 

    fn add_empty_entry(&mut self) -> String{
        let title = String::from(""); 
        let name = String::from("");
        let url = String::from("");
        let comment = String::from("");
        let value = String::from("");
        let id = Uuid::new_v4().to_string();

        self.entries.push(PasswordEntry{id: id.clone(), title, name, value, url, comment, 
            entry_type: EntryType::ClassicPassword, last_modified: Local::now().to_string()});
        id
    }

    fn remove_entry_by_id(&mut self, id: String) -> bool{
        //TODO change function calls or design
        let count_before = self.entries.len(); 
        self.entries = self.entries.iter().filter(|x| x.id != id).cloned().collect();
        count_before < self.entries.len()
    }

    fn save_database(&self, path: &Option<PathBuf>)-> Result<(), anyhow::Error>{
        let serialized_db = serde_json::to_string(self)?;
        encrypt_text(&serialized_db, path.as_ref().unwrap(), self.password.as_str())?;
        Ok(())
    }

    fn get_value_from_selected_detail(&self, index_detail: usize, id: String) -> String{
        let entry = self.entries.iter().find(|x| x.id == id).unwrap();
        match index_detail {
                0 => {entry.title.clone()} 
                1 => {entry.name.clone()} 
                2 => {entry.value.clone()}
                3 => {entry.url.clone()}
                4 => {entry.comment.clone()}
                _ => String::from("")
        }
    }

    fn update_entry(&mut self, index_detail: usize, id: String, message: Vec<String>){
        let converted_message = message.last().unwrap_or(&String::from("error while parsing text")).to_owned(); 
        
        let entry = self.entries.iter_mut().find(|x| x.id == id).unwrap();
        match index_detail {
                0 => {entry.title = converted_message}
                1 => {entry.name = converted_message} 
                2 => {entry.value = converted_message}
                3 => {entry.url = converted_message}
                4 => {entry.comment = converted_message}
                _ => {}
        }
        entry.last_modified = Local::now().to_string();
    }

    fn get_entry_by_id(&self, id: String) -> Option<&PasswordEntry>{
        self.entries.iter().find(|x| x.id == id)
    }
}


pub fn create_new_database(mut args: New) -> Result<(), anyhow::Error>{
    if args.path.is_none(){
        args.path = Some(env::current_dir()?.join(args.database_name.to_owned() + ".vault"));
    }

    let password = prompt_password()?;
    let author = prompt_user("Please enter your name:");
    let comment = prompt_user("Please enter a description for the database:");

    let db = DatabaseFile{entries: Vec::new(),
        config: Config { comment, author}, last_access: Local::now(), password: password.clone()};
    
    let serialized_db = serde_json::to_string(&db)?;
    encrypt_text(&serialized_db, args.path.as_ref()
                 .unwrap_or(&PathBuf::new().join(String::from(&args.database_name)+".vault")), &password)?;
    Ok(())
}
