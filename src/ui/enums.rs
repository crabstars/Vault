use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error reading the DB file: {0}")]
    ReadDBError(#[from] io::Error),
    #[error("error parsing the DB file: {0}")]
    ParseDBError(#[from] serde_json::Error),
}

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MenuItem {
    Home,
    PasswordEntries,
    SelctedEntry
}

pub enum InputMode {
    Navigation,
    Editing,
}
