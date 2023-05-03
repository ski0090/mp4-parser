mod edts;
mod mdia;
mod tkhd;

use self::{edts::Edts, mdia::Mdia, tkhd::Tkhd};
use crate::{
    atoms::{undef::Undef, BaseBox, Mp4AtomParse, Mp4AtomPrint},
    utils::name::BoxType,
};
use mp4_macros::ImplMp4AtomPrint;
use std::io::{BufReader, Read, Seek};

#[derive(Debug, ImplMp4AtomPrint)]
pub struct Trak {
    base: BaseBox,
    #[print_comp(atom_container)]
    atoms: Vec<Box<dyn Mp4AtomPrint>>,
}

impl Mp4AtomParse for Trak {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let mut child_base = base.child(reader);
        let mut atoms: Vec<Box<dyn Mp4AtomPrint>> = Vec::new();
        let end = base.offset + base.size;
        while reader.stream_position().unwrap() < end {
            match child_base.name() {
                BoxType::Tkhd => {
                    atoms.push(Box::new(Tkhd::parse(child_base.clone(), reader)));
                }
                BoxType::Edts => {
                    atoms.push(Box::new(Edts::parse(child_base.clone(), reader)));
                }
                BoxType::Mdia => {
                    atoms.push(Box::new(Mdia::parse(child_base.clone(), reader)));
                }
                _ => {
                    atoms.push(Box::new(Undef::parse(child_base.clone(), reader)));
                }
            }
            child_base = child_base.next(reader);
        }
        Self { base, atoms }
    }
}
