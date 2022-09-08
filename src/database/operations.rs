use std::path::PathBuf;
use std::env;
use anyhow::Ok;
use chrono::Local;
use uuid::Uuid;

use crate::{New, Open};
use crate::database::structures::{Config, DatabaseFile, EntryType};
use crate::encryption_and_decryption::argon::{decrypt_text, encrypt_text};
use crate::utils::terminal_interactions::{prompt_user, prompt_password};

use super::structures::PasswordEntry;


pub fn open_database(args: &mut Open) -> Result<DatabaseFile, anyhow::Error>{
    if args.path.is_none(){
        args.path = Some(env::current_dir()?.join(&args.database_name));
    }
    
    let password = rpassword::prompt_password("Please enter the password:")?;
    let text = decrypt_text(args.path.as_ref().unwrap_or(&PathBuf::new().join(args.database_name.to_owned()+".vault")), &password)?;
    let mut db: DatabaseFile = serde_json::from_str(&text)?;
    db.last_access = Local::now();
    println!("{:?}",db);

    Ok(db)
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
    encrypt_text(&serialized_db, args.path.clone()
                 .unwrap_or_else(|| PathBuf::new().join(String::from(&args.database_name)+".vault")), &password)?;
    Ok(())
}

pub fn add_entry(db: &mut DatabaseFile) -> Result<(), anyhow::Error>{ 
    let title = String::from(""); 
    let name = String::from("");
    let url = String::from("");
    let comment = String::from("");
    let value = String::from("");

    let id = Uuid::new_v4().to_string();
    db.entries.push(PasswordEntry{id, title, name, value, url, comment, 
        entry_type: EntryType::ClassicPassword, last_modified: Local::now().to_string()});
    Ok(())
}

pub fn remove_entry(db: &mut DatabaseFile, index: usize) -> Result<(), anyhow::Error>{
    db.entries.remove(index);
    Ok(())
}

pub fn get_password_entires(db: &DatabaseFile) -> Vec<PasswordEntry>{
    let mut pw: Vec<PasswordEntry> = Vec::new();
    pw.append(&mut db.entries.clone());    
    pw
}


pub fn save_database(db: &DatabaseFile, path: &Option<std::path::PathBuf>, database_name: &String)-> Result<(), anyhow::Error>{
    let serialized_db = serde_json::to_string(&db)?;
    encrypt_text(&serialized_db, path.clone().unwrap_or_else(|| PathBuf::new().join(database_name.to_owned()+".vault")), &db.password)?;
    Ok(())
}

pub fn update_entry(db: &mut DatabaseFile, index_entry: usize, index_detail: usize, message: Vec<String>){
    let error_string = "error while parsing text";
    match index_detail {
            0 => {db.entries[index_entry].title = message.last().unwrap_or(&String::from(error_string)).to_owned()} 
            1 => {db.entries[index_entry].name = message.last().unwrap_or(&String::from(error_string)).to_owned()}
            2 => {db.entries[index_entry].value = message.last().unwrap_or(&String::from(error_string)).to_owned()}
            3 => {db.entries[index_entry].url = message.last().unwrap_or(&String::from(error_string)).to_owned()}
            4 => {db.entries[index_entry].comment = message.last().unwrap_or(&String::from(error_string)).to_owned()}
            _ => {}
    }
    db.entries[index_entry].last_modified = Local::now().to_string();
}

pub fn get_value_from_selected_detail(db: &DatabaseFile, index_entry: usize, index_detail: usize) -> String{
    match index_detail {
            0 => {db.entries[index_entry].title.clone()} 
            1 => {db.entries[index_entry].name.clone()} 
            2 => {db.entries[index_entry].value.clone()}
            3 => {db.entries[index_entry].url.clone()}
            4 => {db.entries[index_entry].comment.clone()}
            _ => String::from("'")
    }
}
