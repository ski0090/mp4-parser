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
