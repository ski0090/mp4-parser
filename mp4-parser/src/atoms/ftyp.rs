use byteorder::*;
use mp4_macros::Printer;
use std::io::{Read, Seek};

use crate::utils::helper::read_string;

use super::{BaseBox, Mp4Atom};

#[derive(Debug, Printer)]
pub struct Ftyp {
    base: BaseBox,
    #[print_comp()]
    major: String,
    #[print_comp()]
    minor: u32,
    #[print_comp(iter)]
    brands: Vec<String>,
}

impl Mp4Atom for Ftyp {
    fn parse<R>(base: BaseBox, reader: &mut std::io::BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let major = read_string(reader, 4);
        let minor = reader.read_u32::<BigEndian>().unwrap();
        let brand_count = (base.size - 16) / 4; // header + major + minor
        let mut brands = Vec::new();
        (0..brand_count).for_each(|_| {
            let brand = read_string(reader, 4);
            brands.push(brand);
        });

        Self {
            base,
            major,
            minor,
            brands,
        }
    }

    fn print_comp(&self) {
        self.base.print();
        self.print_major();
        self.print_minor();
        self.print_brands();
    }
}
