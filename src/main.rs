use archivereader::Archive;
use binrw::BinRead;
use clap::Parser;
use std::fs::File;
use std::iter::zip;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    option: String,
    file: String,
}

// mod dbr;
mod archivereader;

fn main() {
    let args = Args::parse();
    let mut file = File::open(args.file).unwrap();
    let arc = Archive::read(&mut file).unwrap();
    let index = arc.get_buffers();
    let filenames = arc.get_filenames();
    // for (mut buffer, filename) in zip(index, filenames) {
    //     buffer.reverse();
    //     println!("{:02x?}: {}", buffer, filename);
    // }
    if let Ok(decoded) = arc.decode(filenames.last().unwrap()) {
        println!("{}", std::str::from_utf8(&decoded).unwrap());
    }
}
