mod database;
mod utils;
mod encryption_and_decryption;
mod ui;

use std::path::PathBuf;
use anyhow::Ok;
use clap::{Parser, Subcommand};
use database::operations::{create_new_database, Database};
use crate::database::structures::DatabaseFile;

use ui::home_screen::run_gui;


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
pub struct New{
    /// FileName for new created database
    #[clap(short, long)]
    database_name: String, 

    /// Absolute path for file is required
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    path: Option<PathBuf>
}

#[derive(Parser)]
pub struct Open{
    /// FileName from the existing database
    #[clap(short, long)]
    database_name: String,

    /// Absolute path for file is required
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    path: Option<PathBuf>
}

fn main() -> Result<(), anyhow::Error> {
    let command = Command::parse();
    
    match command.subcmd{
        SubCommand::New(sc) => create_new_database(sc)?,
        SubCommand::Open(mut sc) => {
            let mut db: DatabaseFile = *Database::new(&mut sc)?;
            run_gui(&mut db)?;  
            db.save_database(&sc)?;
        },
    }

    Ok(())
}
