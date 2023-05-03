use num_rational::Ratio;
use std::io::{BufReader, Read};

pub fn read_string<R>(reader: &mut BufReader<R>, size: usize) -> String
where
    R: Read,
{
    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(size, 0);
    reader.read_exact(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

pub struct FixedPointU8(Ratio<u16>);

impl FixedPointU8 {
    pub fn new_raw(val: u16) -> Self {
        Self(Ratio::new_raw(val, 0x100))
    }

    pub fn value(&self) -> u8 {
        self.0.to_integer() as u8
    }
}

pub struct FixedPointU16(Ratio<u32>);

impl FixedPointU16 {
    pub fn new_raw(val: u32) -> Self {
        Self(Ratio::new_raw(val, 0x10000))
    }

    pub fn value(&self) -> u16 {
        self.0.to_integer() as u16
    }
}
