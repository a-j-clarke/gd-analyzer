use std::io::{self, Cursor, Error, ErrorKind};

use binrw::{binrw, NullString};
use clap::error::Result;
use lz4::Decoder;

#[binrw]
#[derive(Debug)]
struct Header {
    version: u32, // not sure yet what this number represents, calling it version for now
    file_count: u32,
    active_count: u32, // also not sure what this number represents but it seems like it has
    // something to do with the file count
    offsets_length: u32, // better name?
    file_list_length: u32,
    offsets_begin: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
struct OffsetTable {
    offset: u32,
    compressed_size: u32,
    uncompressed_size: u32,
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
struct FileIndex {
    version: u32,
    data_offset: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    buffer: [u8; 16],
    position: u32,
    filename_length: u32,
    file_list_offset: u32,
}

impl FileIndex {
    fn get_buffer(&self) -> Vec<u8> {
        self.buffer.to_vec()
    }
}

#[binrw]
#[brw(little, magic = b"ARC\0")]
#[derive(Debug)]
pub struct Archive {
    #[brw(pad_size_to = Archive::DATAOFFSET - 0x4)]
    header: Header,
    #[br(count = header.offsets_begin - Archive::DATAOFFSET)]
    data: Vec<u8>,
    #[br(count = header.file_count)]
    offset_table: Vec<OffsetTable>,
    #[br(count = header.file_count)]
    file_list: Vec<NullString>,
    #[br(count = header.file_count)]
    index: Vec<FileIndex>,
}

impl Archive {
    const DATAOFFSET: u32 = 0x800;

    pub fn get_buffers(&self) -> Vec<Vec<u8>> {
        self.index.iter().map(|x| x.get_buffer()).collect()
    }

    pub fn get_filenames(&self) -> Vec<String> {
        self.file_list.iter().map(|x| x.to_string()).collect()
    }

    pub fn decode(&self, filename: &String) -> Result<Vec<u8>, io::Error> {
        match self
            .file_list
            .iter()
            .position(|x| x.to_string() == *filename)
        {
            Some(pos) => {
                let slice_begin = (self.offset_table[pos].offset - Archive::DATAOFFSET) as usize;
                let slice_end = slice_begin + self.offset_table[pos].compressed_size as usize;

                let buff = Cursor::new(&self.data[slice_begin..slice_end]);
                let mut outbuff =
                    Vec::with_capacity(self.offset_table[pos].uncompressed_size as usize);
                let mut decoder = Decoder::new(buff)?;

                match io::copy(&mut decoder, &mut outbuff) {
                    Ok(u) => print!("{u} bytes copied"),
                    Err(e) => println!("No bytes copied! Err: {e}"),
                }

                Ok(outbuff)
            }
            None => Err(Error::new(ErrorKind::NotFound, "{filename} not found")),
        }
    }
}
