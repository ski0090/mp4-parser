pub mod ftyp;
pub mod moov;

use crate::utils::name::BoxType;
use std::{
    fmt::{Debug, Display},
    io::{BufReader, Read, Seek, SeekFrom},
};

pub trait Mp4Atom: Debug {
    fn parse<R>(base: BaseBox, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
        Self: Sized;

    fn print_atom(&self);
}

#[derive(Debug, Clone)]
pub struct BaseBox {
    offset: u64,
    size: u64,
    name: BoxType,
    depth: u32,
}

impl BaseBox {
    pub fn new<R>(reader: &mut std::io::BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let (name, size) = Self::parse_header(reader);
        Self {
            offset: 0,
            size,
            name,
            depth: 0,
        }
    }

    pub fn next<R>(&self, reader: &mut BufReader<R>) -> Self
    where
        R: Read + Seek,
    {
        let offset = self.offset + self.size;
        reader.seek(SeekFrom::Start(offset)).unwrap();
        let (name, size) = Self::parse_header(reader);
        Self {
            offset,
            size,
            name,
            depth: self.depth,
        }
    }

    fn parse_header<R>(reader: &mut BufReader<R>) -> (BoxType, u64)
    where
        R: Read + Seek,
    {
        let mut buf = [0u8; 8];
        reader.read_exact(&mut buf).unwrap();

        let s = buf[0..4].try_into().unwrap();
        let size = u32::from_be_bytes(s);

        let t = buf[4..8].try_into().unwrap();
        let type_ = u32::from_be_bytes(t);

        if size == 1 {
            reader.read_exact(&mut buf).unwrap();
            let largesize = u64::from_be_bytes(buf);
            let name = crate::utils::name::BoxType::from(type_);
            let size = match largesize {
                0 => 0,
                1..=15 => panic!("64-bit box size too small"),
                16..=u64::MAX => largesize - 8,
            };

            (name, size)
        } else {
            let name = crate::utils::name::BoxType::from(type_);
            let size = size as u64;

            (name, size)
        }
    }

    pub fn name(&self) -> BoxType {
        self.name
    }

    pub fn print(&self) {
        self.print_depth();
        println!("name: {}", self.name.as_ref());
        self.print_depth();
        println!("size: {}", self.size);
    }

    pub fn print_depth(&self) {
        for _ in 0..self.depth {
            print!("\t");
        }
    }
}

impl Display for BaseBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.name.as_ref(), self.size)
    }
}
