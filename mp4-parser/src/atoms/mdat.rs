use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub struct Mdat {
    base: BaseBox,
}

impl Mp4Atom for Mdat {
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
