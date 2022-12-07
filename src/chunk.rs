#![allow(unused_variables)]
use crate::chunk_type::{self, ChunkType};
use anyhow::{ensure, Context, Error, Result, anyhow};
use crc::{Crc, CRC_32_ISO_HDLC};
use std::fmt;
use std::io::BufReader;

#[derive(Clone, PartialEq, Debug)]
pub struct Chunk {
    pub length: u32,
    pub chunk_type: ChunkType,
    pub chunk_data: Vec<u8>,
    pub crc: u32,
}

impl Chunk {
    pub fn calculate_crc(chunk_type: &ChunkType, data: &Vec<u8>) -> u32 {
        let crc32_iso: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        crc32_iso.checksum(
            &(chunk_type
                .bytes()
                .iter()
                .chain(data.iter())
                .copied()
                .collect::<Vec<u8>>()),
        )
    }
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Chunk::calculate_crc(&chunk_type, &data);

        Chunk {
            length: data.len() as u32,
            chunk_type: chunk_type,
            chunk_data: data,
            crc: crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String> {
        Ok(String::from_utf8(self.chunk_data.clone()).unwrap())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Error> {
        println!("bytes: {:?}", bytes);
        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        println!("chunk buffer size: {}", bytes.len());
        std::io::Read::read_exact(&mut reader, &mut buffer)?;
        let data_length = u32::from_be_bytes(buffer);
        println!("data_length: {}", data_length);
        std::io::Read::read_exact(&mut reader, &mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer).unwrap();
        let data_loc = 8+data_length as usize;
        let chunk_data = bytes[8..data_loc].to_vec();
        println!("chunk_data: {:?}", chunk_data);
        let crc = u32::from_be_bytes(bytes[data_loc..data_loc+4].try_into().unwrap());
        println!("crc: {:?}", crc);

        ensure!(
            data_length == chunk_data.len() as u32,
            "Invalid length, size of chunk data and its length field do not match"
        );

        if Chunk::calculate_crc(&chunk_type, &chunk_data) != crc{
            println!("some error crc incorrect");
            return Err(anyhow!("Missing attribute"))
        }

        Ok(Chunk {
            length: data_length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
