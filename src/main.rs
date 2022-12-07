use std::fs;
use std::str::FromStr;

use clap::Parser;
use commands::{Cli, Commands};

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Encode {
            filepath,
            chunktype,
            message,
            output,
        }) => {
            let mut png = Png::from(filepath);
            let output_path;
            if let Some(i) = output {
                output_path = i
            } else {
                output_path = filepath
            };
            png.append_chunk(Chunk::new(
                ChunkType::from_str(chunktype).expect("Cannot append chunk type"),
                message.clone().into_bytes(),
            ));
            fs::write(output_path, png.as_bytes()).unwrap();

            println!("Successfully added a secret message to file");
        }

        Some(Commands::Decode {
            filepath,
            chunktype,
        }) => {
            let png = Png::from(filepath);
            let chunk = png
                .chunk_by_type(chunktype)
                .expect("Chunk type doesn't exist");
            println!(
                "{}",
                chunk.data_as_string().expect("Chunk format is not correct")
            );
        }

        Some(Commands::Remove {
            filepath,
            chunktype,
        }) => {
            let mut png = Png::from(filepath);
            let removed_chunk = png.remove_chunk(chunktype).expect("Unable to remove chunk");
            fs::write(filepath, png.as_bytes()).expect("Cannot write file");

            println!("Chunk {} is removed!", removed_chunk.chunk_type);
        }

        Some(Commands::Print { filepath }) => {
            println!(
                "{}",
                Png::from(filepath)
                    .chunks
                    .into_iter()
                    .map(|chunk| chunk.chunk_type.to_string())
                    .collect::<Vec<String>>() // turbofish
                    .join(" ")
            );
        }

        None => {
            // must have
            unreachable!();
        }
    };
}
