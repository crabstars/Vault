mod database;
mod encryption_and_decryption;
mod ui;
mod utils;

use crate::database::structures::DatabaseFile;
use anyhow::Ok;
use clap::{Parser, Subcommand};
use database::operations::{create_new_database, Database};
use std::path::PathBuf;

use ui::home_screen::run_gui;

#[derive(Parser)]
#[clap(
    version = "0.1",
    author = "Daniel Waechtler https://github.com/LamaKami"
)]
struct Command {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand)]
enum SubCommand {
    New(New),
    Open(Open),
}

/// Simple program to greet a person
#[derive(Parser)]
#[command(version)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Parser)]
pub struct New {
    /// FileName for new created database
    #[clap(short, long)]
    database_name: String,

    /// Absolute path for file is required
    #[clap(short, long)]
    path: Option<PathBuf>,
}

#[derive(Parser)]
pub struct Open {
    /// FileName from the existing database
    #[clap(short, long)]
    database_name: String,

    /// Absolute path for file is required
    #[clap(short, long)]
    path: Option<PathBuf>,
}

fn main() -> Result<(), anyhow::Error> {
    let command = Command::parse();

    match command.subcmd {
        SubCommand::New(sc) => create_new_database(sc)?,
        SubCommand::Open(mut sc) => {
            let mut db: DatabaseFile = *Database::new(&mut sc.path, sc.database_name.as_str())?;
            run_gui(&mut db)?;
            db.save_database(&sc.path)?;
        }
    }
    clearscreen::clear().expect("failed to clear screen");
    Ok(())
}
