mod database;
mod utils;
mod encryption_and_decryption;
mod ui;

use anyhow::Ok;
use clap::{Parser, Subcommand};
use database::operations::{create_new_database, open_database, manage_database, open_database_for_manage};
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
    Manage(Manage),
}

#[derive(Parser)]
pub struct New{
    /// FileName for new created database
    #[clap(short, long)]
    database_name: String,

    /// Absolute path for file is required
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>
}

#[derive(Parser)]
pub struct Open{
    /// FileName from the existing database
    #[clap(short, long)]
    database_name: String,

    /// Absolute path for file is required
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>
}

#[derive(Parser)]
pub struct Manage{
    /// FileName from the existing database
    #[clap(short, long)]
    database_name: String,

    /// Absolute path for file is required
    #[clap(short, long)]
    #[clap(parse(from_os_str))]
    path: Option<std::path::PathBuf>
}

fn main() -> Result<(), anyhow::Error> {
    let command = Command::parse();
    
    match command.subcmd{
        SubCommand::New(sc) => create_new_database(sc)?,
        SubCommand::Manage(mut sc) =>{
            let mut db = open_database_for_manage(&mut sc)?;
            manage_database(&mut db, &sc)?;
        }
        SubCommand::Open(mut sc) => {
            let mut db = open_database(&mut sc)?;
            run_gui(db);  
        },
    }

    Ok(())
}
