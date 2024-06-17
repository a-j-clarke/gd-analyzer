use archivereader::Archive;
use binrw::BinRead;
use clap::{Parser, ValueEnum};
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of archive
    #[arg(short, long)]
    archive: String,

    /// File to perform command on
    #[arg(short, long)]
    file: Option<String>,

    /// Command to execute
    #[arg(short, long, value_enum)]
    command: Command,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Command {
    /// List files in archive
    List,
    /// Print archive header
    Header,
    /// Decode a specified file
    Decode,
    /// Print archive index
    Index,
    /// Get buffers from file index only
    Buffers,
}

// mod dbr;
mod archivereader;

fn main() {
    let args = Args::parse();
    let mut file = File::open(args.archive).unwrap();
    let arc = Archive::read(&mut file).unwrap();
    match args.command {
        Command::List => {
            let filenames = arc.get_filenames();
            for file in filenames {
                println!("{}", file);
            }
        }
        Command::Decode => match args.file {
            Some(name) => {
                if let Ok(decoded) = arc.decode(&name) {
                    println!("{}", std::str::from_utf8(&decoded).unwrap());
                }
            }
            None => println!("Please provide a filename to decode"),
        },
        Command::Header => {
            let header = arc.get_header();
            println!("{header:?}");
        }
        Command::Index => {
            let index = arc.get_index();
            for entry in index {
                println!("{entry:?}");
            }
        }
        Command::Buffers => {
            let buffers = arc.get_buffers();
            for entry in buffers {
                println!(
                    "[{}]",
                    entry
                        .iter()
                        .map(|x| format!("{x:#4?}"))
                        .collect::<Vec<_>>()
                        .join(",")
                );
            }
        }
    }
}
