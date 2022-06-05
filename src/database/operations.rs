use std::path::PathBuf;
use std::{env};
use chrono::Local;

use crate::{New, Open};
use crate::database::structures::{Config, DatabaseFile};
use crate::encryption_and_decryption::argon::{decrypt_text, encrypt_text};


pub fn open_database(mut args: Open) -> Result<DatabaseFile, anyhow::Error>{
    if args.path.is_none(){
        args.path = Some(env::current_dir()?.join(&args.database_name));
    }
    
    let password = rpassword::prompt_password_stdout("Please enter the password:")?;
    let text = decrypt_text(&args.path.unwrap_or(PathBuf::new().join(args.database_name+".vault")), &password)?;
    let db: DatabaseFile = serde_json::from_str(&text)?;
    println!("{:?}",db);

    Ok(db)
}

pub fn create_new_database(mut args: New) -> Result<(), anyhow::Error>{
    if args.path.is_none(){
        args.path = Some(env::current_dir()?.join(args.database_name.to_owned()+ ".vault"));
    }

    let mut password: String;
    loop {
         password = rpassword::prompt_password_stdout("Please enter the password:")?;
        if password != rpassword::prompt_password_stdout("Please re-enter the password:")?{
            println!("Your passwords didn't match, pls try again!")
        } else {
            break;
        }
    }
    let db = DatabaseFile{entries: Vec::new(),
        config: Config { comment:"todo".to_string(),
        author: "todo".to_string() }, last_access: Local::now()};
    
    let serialized_db = serde_json::to_string(&db)?;
    encrypt_text(&serialized_db , &args.path.unwrap_or(PathBuf::new().join(args.database_name+".vault")), &password)?;
    println!("{:?}", serialized_db);
    // TODO switch to overview from database where u can add and remove things
    Ok(())
}

pub fn manage_databse(db: &DatabaseFile){
    panic!("Not implemented")

}

pub fn save_database(db: DatabaseFile){
    panic!("Not implemented")
}