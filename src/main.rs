mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::{arg, App, AppSettings, Arg};
use std::path::PathBuf;

use crate::args::EncodeArgs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    commands::PngMeCmd::new().match_handler();
    Ok(())
}
