use byteorder::{BigEndian, ReadBytesExt};
use mp4_macros::Printer;

use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug, Printer)]
pub struct Vmhd {
    base: BaseBox,
    #[print_comp()]
    version: u8,
    #[print_comp()]
    flags: u32,
    #[print_comp()]
    graphic_mode: u16,
    #[print_comp()]
    opcolor: u64,
}

impl Mp4Atom for Vmhd {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let version = reader.read_u8().unwrap();
        let flags = reader.read_u24::<BigEndian>().unwrap();
        let graphic_mode = reader.read_u16::<BigEndian>().unwrap();
        let opcolor = reader.read_u48::<BigEndian>().unwrap();
        Self {
            base,
            version,
            flags,
            graphic_mode,
            opcolor,
        }
    }

    fn print_comp(&self) {
        self.base.print();
        self.print_version();
        self.print_flags();
        self.print_graphic_mode();
        self.print_opcolor();
    }
}
