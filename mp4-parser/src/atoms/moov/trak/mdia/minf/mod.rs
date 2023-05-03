mod dinf;
mod smhd;
mod vmhd;

use crate::{atoms::undef::Undef, utils::name::BoxType};

use self::{dinf::Dinf, smhd::Smhd, vmhd::Vmhd};

use super::{hdlr::Hdlr, BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub struct Minf {
    base: BaseBox,
    atoms: Vec<Box<dyn Mp4Atom>>,
}

impl Mp4Atom for Minf {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let mut child_base = base.child(reader);
        let mut atoms: Vec<Box<dyn Mp4Atom>> = Vec::new();
        let end = base.offset + base.size;
        while reader.stream_position().unwrap() < end {
            match child_base.name() {
                BoxType::Smhd => {
                    atoms.push(Box::new(Smhd::parse(child_base.clone(), reader)));
                }
                BoxType::Vmhd => {
                    atoms.push(Box::new(Vmhd::parse(child_base.clone(), reader)));
                }
                BoxType::Hdlr => {
                    atoms.push(Box::new(Hdlr::parse(child_base.clone(), reader)));
                }
                BoxType::Dinf => {
                    atoms.push(Box::new(Dinf::parse(child_base.clone(), reader)));
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
