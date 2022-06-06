use std::path::PathBuf;
use std::{env};
use anyhow::Ok;
use chrono::Local;

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
    let text = decrypt_text(&args.path.as_ref().unwrap_or(&PathBuf::new().join(args.database_name.to_owned()+".vault")), &password)?;
    let mut db: DatabaseFile = serde_json::from_str(&text)?;
    db.last_access = Local::now();
    println!("{:?}",db);

    Ok(db)
}

pub fn create_new_database(mut args: New) -> Result<(), anyhow::Error>{
    if args.path.is_none(){
        args.path = Some(env::current_dir()?.join(args.database_name.to_owned()+ ".vault"));
    }

    let password = prompt_password()?;
    let author = prompt_user("Please enter your name:");
    let comment = prompt_user("Please enter a description for the database:");

    let db = DatabaseFile{entries: Vec::new(),
        config: Config { comment, author}, last_access: Local::now(), password: password.clone()};
    
    let serialized_db = serde_json::to_string(&db)?;
    encrypt_text(&serialized_db , &args.path.unwrap_or(PathBuf::new().join(args.database_name+".vault")), &password)?;
    println!("{:?}", serialized_db);
    // TODO switch to overview from database where u can add and remove things
    Ok(())
}

pub fn manage_database(db: &mut DatabaseFile, args: &Open) -> Result<(), anyhow::Error>{
    loop {
        let decision = prompt_user("What do you want to do? \n 
(1) Add Entry \n 
(2) Remove Entry \n 
(3) Get Entry \n 
(4) Show Entries \n
(5) End\n\n"); 
        
        match decision.as_str() {
            "1" => add_entry(db)?,
            "2" => remove_entry()?,
            "4" => show_entries(db),
            "5" => {save_database(db, args)?; return Ok(());},
            _ => println!("Wrong input, only: 1, 2, 3, 4, 5")
        } 
    }    
}

pub fn add_entry(db: &mut DatabaseFile) -> Result<(), anyhow::Error>{
    
    let title = prompt_user("Enter the title for the entry:");
    let name = prompt_user("Enter the corresponding name for the future password:");
    let comment  = prompt_user("Enter additional informations:");

    let value = prompt_password()?;
    db.entries.push(PasswordEntry{title, name, value, comment, entry_type: EntryType::ClassicPassword, last_modified: Local::now()});
    return Ok(())
}

pub fn remove_entry() -> Result<(), anyhow::Error>{
    // Todo before tui
    return Ok(());
}

pub fn get_entry(){
    // get Speciffic infos
}

pub fn show_entries(db: &DatabaseFile){
    // Change when implementing TUI, also dont show pw here
    for password_entry in db.entries.clone(){
        println!("{:?}", password_entry)
    }
}

pub fn save_database(db: &DatabaseFile, args: &Open)-> Result<(), anyhow::Error>{
    // Todo this two lines in extra function
    let serialized_db = serde_json::to_string(&db)?;
    encrypt_text(&serialized_db , &args.path.as_ref().unwrap_or(&PathBuf::new().join(args.database_name.to_owned()+".vault")), &db.password)?;
    return Ok(());
}