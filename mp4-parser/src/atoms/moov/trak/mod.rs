mod edts;
mod mdia;
mod tkhd;

use self::{edts::Edts, mdia::Mdia, tkhd::Tkhd};
use crate::{
    atoms::{undef::Undef, BaseBox, Mp4Atom},
    utils::name::BoxType,
};
use mp4_macros::Printer;
use std::io::{BufReader, Read, Seek};

#[derive(Debug, Printer)]
pub struct Trak {
    base: BaseBox,
    atoms: Vec<Box<dyn Mp4Atom>>,
}

impl Mp4Atom for Trak {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let mut child_base = base.child(reader);
        let mut atoms: Vec<Box<dyn Mp4Atom>> = Vec::new();
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

    fn print_comp(&self) {
        self.base.print();
        self.atoms.iter().for_each(|atom| atom.print_comp());
    }
}
