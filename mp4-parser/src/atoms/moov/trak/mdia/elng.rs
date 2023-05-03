use super::{BaseBox, Mp4AtomParse};
use crate::utils::{helper::read_string, values::HEADER_SIZE};
use byteorder::{BigEndian, ReadBytesExt};
use mp4_macros::ImplMp4AtomPrint;
use std::io::{BufReader, Read, Seek};

#[derive(Debug, ImplMp4AtomPrint)]
pub struct Elng {
    base: BaseBox,
    #[print_comp()]
    version: u8,
    #[print_comp()]
    flags: u32,
    #[print_comp()]
    language: String,
}

impl Mp4AtomParse for Elng {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let version = reader.read_u8().unwrap();
        let flags = reader.read_u24::<BigEndian>().unwrap();
        let capacity = base.size - (HEADER_SIZE + 1 + 3);
        let language = read_string(reader, capacity as usize);

        Self {
            base,
            version,
            flags,
            language,
        }
    }
}
