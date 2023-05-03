use byteorder::{BigEndian, ReadBytesExt};
use mp4_macros::Printer;

use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug, Printer)]
pub struct Dref {
    base: BaseBox,
    #[print_comp()]
    version: u8,
    #[print_comp()]
    flags: u32,
    #[print_comp()]
    num_of_entires: u32,
}

impl Mp4Atom for Dref {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let version = reader.read_u8().unwrap();
        let flags = reader.read_u24::<BigEndian>().unwrap();
        let num_of_entires = reader.read_u32::<BigEndian>().unwrap();

        Self {
            base,
            version,
            flags,
            num_of_entires,
        }
    }

    fn print_comp(&self) {
        self.base.print();
        self.print_version();
        self.print_flags();
        self.print_num_of_entires();
    }
}
