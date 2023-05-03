use super::{BaseBox, Mp4Atom};
use crate::utils::helper::{FixedPointU16, FixedPointU8};
use byteorder::{BigEndian, ReadBytesExt};
use mp4_macros::Printer;
use std::io::{BufReader, Read, Seek, SeekFrom};

#[derive(Debug, Printer)]
pub struct Tkhd {
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
    track_id: u32,
    #[print_comp()]
    duration: u64,
    #[print_comp()]
    layer: u16,
    #[print_comp()]
    alternate_group: u16,
    #[print_comp()]
    volume: u8,
    #[print_comp()]
    width: u16,
    #[print_comp()]
    height: u16,
}

impl Mp4Atom for Tkhd {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let version = reader.read_u8().unwrap();
        let flags = reader.read_u24::<BigEndian>().unwrap();
        let (creation_time, modification_time, track_id, _reserved, duration) = if version == 1 {
            (
                reader.read_u64::<BigEndian>().unwrap(),
                reader.read_u64::<BigEndian>().unwrap(),
                reader.read_u32::<BigEndian>().unwrap(),
                reader.read_u32::<BigEndian>().unwrap(),
                reader.read_u64::<BigEndian>().unwrap(),
            )
        } else if version == 0 {
            (
                reader.read_u32::<BigEndian>().unwrap() as u64,
                reader.read_u32::<BigEndian>().unwrap() as u64,
                reader.read_u32::<BigEndian>().unwrap(),
                reader.read_u32::<BigEndian>().unwrap(),
                reader.read_u32::<BigEndian>().unwrap() as u64,
            )
        } else {
            panic!("version must be 0 or 1");
        };

        //skip reserved
        reader.seek(SeekFrom::Current(8)).unwrap();

        let layer = reader.read_u16::<BigEndian>().unwrap();
        let alternate_group = reader.read_u16::<BigEndian>().unwrap();
        let volume = FixedPointU8::new_raw(reader.read_u16::<BigEndian>().unwrap()).value();

        //skip reserved
        reader.seek(SeekFrom::Current(2)).unwrap();
        //skip matrix
        reader.seek(SeekFrom::Current(36)).unwrap();

        let width = FixedPointU16::new_raw(reader.read_u32::<BigEndian>().unwrap()).value();
        let height = FixedPointU16::new_raw(reader.read_u32::<BigEndian>().unwrap()).value();

        Self {
            base,
            version,
            flags,
            creation_time,
            modification_time,
            track_id,
            duration,
            layer,
            alternate_group,
            volume,
            width,
            height,
        }
    }

    fn print_comp(&self) {
        self.base.print();
        self.print_version();
        self.print_flags();
        self.print_creation_time();
        self.print_modification_time();
        self.print_track_id();
        self.print_duration();
        self.print_layer();
        self.print_alternate_group();
        self.print_volume();
        self.print_width();
        self.print_height();
    }
}
