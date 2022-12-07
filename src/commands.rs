use std::path::PathBuf;

use clap::{arg, Parser, Subcommand};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Hide your hidden message in Png!"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}


#[derive(Subcommand)]
#[command(arg_required_else_help(true))]
pub enum Commands {
    /// encode your secret message
    Encode {
        #[arg(short, long)]
        filepath: PathBuf,
        chunktype: String,
        message: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// decode your secret message
    Decode {
        filepath: PathBuf,
        chunktype: String,
    },
    /// remove your secret message
    Remove {
        filepath: PathBuf,
        chunktype: String,
    },
    /// print your secret message
    Print {
        filepath: PathBuf,
    }
    ,
}


