mod mvhd;
mod trak;

use mp4_macros::ImplMp4AtomPrint;

use crate::utils::name::BoxType;

use self::{mvhd::Mvhd, trak::Trak};

use super::{undef::Undef, BaseBox, Mp4AtomParse, Mp4AtomPrint};
use std::io::{BufReader, Read, Seek};

#[derive(Debug, ImplMp4AtomPrint)]
pub struct Moov {
    base: BaseBox,
    #[print_comp(atom_container)]
    atoms: Vec<Box<dyn Mp4AtomPrint>>,
}

impl Mp4AtomParse for Moov {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let mut child_base = base.child(reader);
        let mut atoms: Vec<Box<dyn Mp4AtomPrint>> = Vec::new();
        let end = base.offset + base.size;
        while reader.stream_position().unwrap() < end {
            match child_base.name() {
                BoxType::Mvhd => {
                    atoms.push(Box::new(Mvhd::parse(child_base.clone(), reader)));
                }
                BoxType::Trak => {
                    atoms.push(Box::new(Trak::parse(child_base.clone(), reader)));
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
