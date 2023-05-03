use byteorder::{BigEndian, ReadBytesExt};
use mp4_macros::ImplMp4AtomPrint;

use crate::utils::{helper::read_string, values::HEADER_SIZE};

use super::{BaseBox, Mp4AtomParse};
use std::io::{BufReader, Read, Seek};

#[derive(Debug, ImplMp4AtomPrint)]
pub struct Hdlr {
    base: BaseBox,
    #[print_comp()]
    version: u8,
    #[print_comp()]
    flags: u32,
    #[print_comp()]
    component_type: String,
    #[print_comp()]
    component_sub: String,
    #[print_comp()]
    component_manufacturer: u32,
    #[print_comp()]
    component_flags: u32,
    #[print_comp()]
    component_mask: u32,
    #[print_comp()]
    component_name: String,
}

impl Mp4AtomParse for Hdlr {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let version = reader.read_u8().unwrap();
        let flags = reader.read_u24::<BigEndian>().unwrap();
        let component_type = read_string(reader, 4);
        let component_sub = read_string(reader, 4);
        let component_manufacturer = reader.read_u32::<BigEndian>().unwrap();
        let component_flags = reader.read_u32::<BigEndian>().unwrap();
        let component_mask = reader.read_u32::<BigEndian>().unwrap();
        let capacity = base.size - (HEADER_SIZE + 1 + 3 + 20);
        let component_name = read_string(reader, capacity as usize);

        Self {
            base,
            version,
            flags,
            component_type,
            component_sub,
            component_manufacturer,
            component_flags,
            component_mask,
            component_name,
        }
    }
}
