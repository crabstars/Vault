mod encryption_and_decryption;

use clap::{Parser, Subcommand};
use encryption_and_decryption::argon::{encrypt_text, decrypt_file};
use serde::{Deserialize, Serialize};
use std::{env, time::SystemTime};
use chrono::{DateTime, Utc, Local};

use crate::encryption_and_decryption::argon::decrypt_text;

#[derive(Parser)]
#[clap(version = "0.1", author = "Daniel Waechtler https://github.com/LamaKami")]
struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    New(New), // Syntax is needed later to give a reference to access the props from the struct
    Open(Open),
}

#[derive(Parser)]
struct New{
    /// FileName for new created database
    #[clap(short, long)]
    database_name: String,

    /// Absolute path for file is required
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>
}

#[derive(Parser)]
struct Open{
    /// FileName from the existing database
    #[clap(short, long)]
    database_name: String,

    /// Absolute path for file is required
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>
}

#[derive(Serialize, Deserialize, Clone)]
enum EntryType{
    ClassicPassowrd,
    EnvironmentVariable
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PassowrdEntry{
    name: String, // like username or variable name
    value: String, //or password
    comment: String,
    entry_type: EntryType,
    last_modified:  DateTime<Local>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config{
    // all the argon props and more
    comment: String,
    author: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct DatabaseFile{
    entries: Vec<PassowrdEntry>,
    config: Config,
    last_access: DateTime<Local>
}


fn main() -> Result<(), anyhow::Error> {
    // let command = Command::parse();
    // let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

    
    // match command.subcmd{
    //     SubCommand::New(sc) => create_new_database(sc)?,
    //     SubCommand::Open(sc) => todo!(),
    // }

    //encrypt_text("text von mir", "bla.text.decrypted", "password")?;
    println!("{}",decrypt_text("bla.text.decrypted", "password")?);
    Ok(())
}



fn create_new_database(mut args: New) -> Result<(), anyhow::Error>{
    if args.path.is_none(){
        args.path = Some(env::current_dir()?);
    }

    let mut password = String::new();
    loop {
         password = rpassword::prompt_password_stdout("Please enter the password:")?;
        if password != rpassword::prompt_password_stdout("Please re-enter the password:")?{
            println!("Your passwords didn't match, pls try again!")
        } else {
            break;
        }
    }
    
    // TODO switch to overview from database where u can add and remove things
    Ok(())
}