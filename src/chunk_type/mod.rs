use crate::Error;
use std::{fmt::Display, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_critical(&self) -> bool {
        false
    }
    fn is_public(&self) -> bool {
        false
    }
    fn is_err(&self) -> bool {
        false
    }
    fn is_reserved_bit_valid(&self) -> bool {
        false
    }
    fn is_safe_to_copy(&self) -> bool {
        false
    }
    fn is_valid(&self) -> bool {
        false
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    // type Error;
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for i in value {
            if !((i >= 'a' as u8 && i <= 'z' as u8) || (i >= 'A' as u8 && i <= 'Z' as u8)) {
                return Err("invalid char");
            }
        }
        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("length is invalid");
        }

        let v = s
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| *x as u8)
            .collect::<Vec<u8>>();
        Ok(ChunkType {
            bytes: [v[0], v[1], v[2], v[3]],
        })
    }
}

/* impl ToString for ChunkType {
    fn to_string(&self) -> String {
        String::from("cest")
    }
} */

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Ok(());
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
