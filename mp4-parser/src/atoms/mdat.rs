use mp4_macros::ImplMp4AtomPrint;

use super::{BaseBox, Mp4AtomParse};
use std::io::{BufReader, Read, Seek};

#[derive(Debug, ImplMp4AtomPrint)]
pub struct Mdat {
    base: BaseBox,
}

impl Mp4AtomParse for Mdat {
    fn parse<R>(base: BaseBox, _reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        Self { base }
    }
}
