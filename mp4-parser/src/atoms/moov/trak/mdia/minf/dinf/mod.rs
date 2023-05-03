mod dref;

use crate::{atoms::undef::Undef, utils::name::BoxType};

use self::dref::Dref;

use super::{BaseBox, Mp4Atom};
use std::io::{BufReader, Read, Seek};

#[derive(Debug)]
pub struct Dinf {
    base: BaseBox,
    atoms: Vec<Box<dyn Mp4Atom>>,
}

impl Mp4Atom for Dinf {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let mut child_base = base.child(reader);
        let mut atoms: Vec<Box<dyn Mp4Atom>> = Vec::new();
        let end = base.offset + base.size;
        while reader.stream_position().unwrap() < end {
            match child_base.name() {
                BoxType::Dref => {
                    atoms.push(Box::new(Dref::parse(child_base.clone(), reader)));
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
