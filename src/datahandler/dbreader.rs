use binrw::binrw;

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Header {
    version: u16,
    records_offset: u32,
    records_length: u32,
    file_count: u32,
    strings_offset: u32,
    strings_length: u32,
}

impl Header {
    fn get(&self) -> &Header {
        self
    }
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Record {
    filename_index: u32,
    name_length: u32,
    #[br(count = name_length)]
    name: Vec<u8>, // TODO: make into string
    offset: u32,
    compressed_size: u32,
    decompressed_size: u32,
    buffer: [u8; 8], // wat dis?
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct ARZString {
    length: u32,
    #[br(count = length)]
    content: Vec<u8>, // TODO: make into string
}

#[binrw]
#[brw(little)]
#[derive(Debug)]
pub struct Footer {
    buffer: [u8; 16],
}

#[binrw]
#[brw(little, magic = b"\x00\x02")]
#[derive(Debug)]
pub struct Arz {
    header: Header,
    #[br(count = header.records_offset - Arz::DATAOFFSET)]
    data: Vec<u8>, // TODO: lazy load
    #[br(count = header.records_length)]
    records: Vec<Record>,
    #[br(count = header.strings_length)]
    strings: Vec<ARZString>,
    footer: Footer,
}

impl Arz {
    const DATAOFFSET: u32 = 0x16;

    pub fn get_header(&self) -> &Header {
        &self.header
    }
}
