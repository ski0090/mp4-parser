use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub struct Edts {
    base: BaseBox,
}

impl Mp4Atom for Edts {
    fn parse<R>(base: BaseBox, _reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        Self { base }
    }

    fn print_comp(&self) {
        self.base.print();
    }
}
