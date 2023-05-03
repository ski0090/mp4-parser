use byteorder::{BigEndian, ReadBytesExt};
use mp4_macros::Printer;

use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug, Printer)]
pub struct Mdhd {
    base: BaseBox,
    #[print_comp()]
    version: u8,
    #[print_comp()]
    flags: u32,
    #[print_comp()]
    creation_time: u64,
    #[print_comp()]
    modification_time: u64,
    #[print_comp()]
    time_scale: u64,
    #[print_comp()]
    duration: u64,
    #[print_comp()]
    language: u16,
    #[print_comp()]
    quality: u16,
}

impl Mp4Atom for Mdhd {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let version = reader.read_u8().unwrap();
        let flags = reader.read_u24::<BigEndian>().unwrap();

        let (creation_time, modification_time, time_scale, duration, language, quality) =
            if version == 1 {
                (
                    reader.read_u64::<BigEndian>().unwrap(),
                    reader.read_u64::<BigEndian>().unwrap(),
                    reader.read_u64::<BigEndian>().unwrap(),
                    reader.read_u64::<BigEndian>().unwrap(),
                    reader.read_u16::<BigEndian>().unwrap(),
                    reader.read_u16::<BigEndian>().unwrap(),
                )
            } else if version == 0 {
                (
                    reader.read_u32::<BigEndian>().unwrap() as u64,
                    reader.read_u32::<BigEndian>().unwrap() as u64,
                    reader.read_u32::<BigEndian>().unwrap() as u64,
                    reader.read_u32::<BigEndian>().unwrap() as u64,
                    reader.read_u16::<BigEndian>().unwrap(),
                    reader.read_u16::<BigEndian>().unwrap(),
                )
            } else {
                panic!("version must be 0 or 1");
            };
        Self {
            base,
            version,
            flags,
            creation_time,
            modification_time,
            time_scale,
            duration,
            language,
            quality,
        }
    }

    fn print_comp(&self) {
        self.base.print();
        self.print_version();
        self.print_flags();
        self.print_creation_time();
        self.print_modification_time();
        self.print_time_scale();
        self.print_duration();
        self.print_language();
        self.print_quality();
    }
}
