use byteorder::{BigEndian, ReadBytesExt};
use mp4_macros::Printer;

use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek, SeekFrom};

#[derive(Debug, Printer)]
pub struct Mvhd {
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
    timescale: u32,
    #[print_comp()]
    duration: u64,
    #[print_comp()]
    current_time: u32,
    #[print_comp()]
    next_track_id: u32,
    #[print_comp(st)]
    duration_sec: std::time::Duration,
}

impl Mp4Atom for Mvhd {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let version = reader.read_u8().unwrap();
        let flags = reader.read_u24::<BigEndian>().unwrap();
        let (creation_time, modification_time, timescale, duration) = if version == 1 {
            (
                reader.read_u64::<BigEndian>().unwrap(),
                reader.read_u64::<BigEndian>().unwrap(),
                reader.read_u32::<BigEndian>().unwrap(),
                reader.read_u64::<BigEndian>().unwrap(),
            )
        } else if version == 0 {
            (
                reader.read_u32::<BigEndian>().unwrap() as u64,
                reader.read_u32::<BigEndian>().unwrap() as u64,
                reader.read_u32::<BigEndian>().unwrap(),
                reader.read_u32::<BigEndian>().unwrap() as u64,
            )
        } else {
            panic!("version must be 0 or 1");
        };

        //skip reserved
        reader.seek(SeekFrom::Current(10)).unwrap();

        //skip matrix
        reader.seek(SeekFrom::Current(36)).unwrap();

        //skip preview time
        reader.seek(SeekFrom::Current(4)).unwrap();

        //skip preview duration
        reader.seek(SeekFrom::Current(4)).unwrap();

        //skip poster time
        reader.seek(SeekFrom::Current(4)).unwrap();

        //skip selection time
        reader.seek(SeekFrom::Current(4)).unwrap();

        //skip selection duration
        reader.seek(SeekFrom::Current(4)).unwrap();

        let current_time = reader.read_u32::<BigEndian>().unwrap();

        let next_track_id = reader.read_u32::<BigEndian>().unwrap();

        let duration_sec = std::time::Duration::from_millis(duration * 1000 / timescale as u64);

        Self {
            base,
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            current_time,
            next_track_id,
            duration_sec,
        }
    }

    fn print_comp(&self) {
        self.base.print();
        self.print_version();
        self.print_flags();
        self.print_creation_time();
        self.print_modification_time();
        self.print_timescale();
        self.print_duration();
        self.print_current_time();
        self.print_next_track_id();
        self.print_duration_sec();
    }
}
