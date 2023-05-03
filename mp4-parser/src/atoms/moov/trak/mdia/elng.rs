use byteorder::{BigEndian, ReadBytesExt};
use mp4_macros::Printer;

use crate::utils::{helper::read_string, values::HEADER_SIZE};

use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug, Printer)]
pub struct Elng {
    base: BaseBox,
    #[print_comp()]
    version: u8,
    #[print_comp()]
    flags: u32,
    #[print_comp()]
    language: String,
}

impl Mp4Atom for Elng {
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

    fn print_comp(&self) {
        self.base.print();
        self.print_version();
        self.print_flags();
        self.print_language();
    }
}
