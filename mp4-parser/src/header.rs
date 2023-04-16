use std::{
    fs::File,
    io::{BufReader, Seek},
    path::Path,
};

use crate::{
    atoms::{ftyp::Ftyp, moov::Moov, BaseBox, Mp4Atom},
    utils::name::BoxType,
};

#[derive(Debug)]
pub struct Mp4Header {
    atoms: Vec<Box<dyn Mp4Atom>>,
}

impl Mp4Header {
    pub fn parse<P>(path: P) -> Mp4Header
    where
        P: AsRef<Path>,
    {
        let f = File::open(path).unwrap();
        let mut reader = BufReader::new(f);
        let mut base = BaseBox::new(&mut reader);
        let mut atoms: Vec<Box<dyn Mp4Atom>> = Vec::new();

        loop {
            match base.name() {
                BoxType::Ftyp => {
                    atoms.push(Box::new(Ftyp::parse(base.clone(), &mut reader)));
                }
                BoxType::Moov => {
                    atoms.push(Box::new(Moov::parse(base.clone(), &mut reader)));
                }
                BoxType::UnknownBox(_) => break,
                _ => {}
            }
            base = base.next(&mut reader);
        }

        Mp4Header { atoms }
    }
}
