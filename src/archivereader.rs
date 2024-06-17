use std::io::{self, Error, ErrorKind};

use binrw::{binrw, NullString};
use clap::error::Result;
use lz4::block::decompress;

#[binrw]
#[derive(Debug)]
pub struct Header {
    version: u32, // not sure yet what this number represents, calling it version for now
    active_count: u32, // it seems like some files take up more space than others so they occupy
    // more entries?
    file_count: u32,
    offsets_length: u32, // contains offsets even for "non active" files, so care must be taken
    // with indices since they won't be 1 to 1 with other collections
    file_list_length: u32,
    offsets_begin: u32,
}

impl Header {
    fn get(&self) -> &Header {
        self
    }
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
pub struct FileIndex {
    version: u32,
    data_offset: u32,
    compressed_size: u32,
    uncompressed_size: u32,
    buffer: [u8; 12],
    number_of_files: u32,
    offset_list_position: u32,
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
    #[br(count = header.active_count)]
    file_list: Vec<NullString>,
    #[br(count = header.active_count)]
    index: Vec<FileIndex>,
}

impl Archive {
    const DATAOFFSET: u32 = 0x800;

    pub fn get_header(&self) -> &Header {
        self.header.get()
    }

    pub fn get_buffers(&self) -> Vec<Vec<u8>> {
        self.index.iter().map(|x| x.get_buffer()).collect()
    }

    pub fn get_filenames(&self) -> Vec<String> {
        self.file_list.iter().map(|x| x.to_string()).collect()
    }

    pub fn get_index(&self) -> &Vec<FileIndex> {
        &self.index
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
                let uncompressed_size = Some(self.offset_table[pos].uncompressed_size as i32);

                decompress(&self.data[slice_begin..slice_end], uncompressed_size)
            }
            None => Err(Error::new(ErrorKind::NotFound, "{filename} not found")),
        }
    }
}
