use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub struct Moov {
    base: BaseBox,
}

impl Mp4Atom for Moov {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        Self { base }
    }

    fn print(&self) {}
}
