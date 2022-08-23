use std::{fmt::Display, str::FromStr};

/// [introduction](http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html#Chunk-naming-conventions)
#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    bytes: [u8; 4],
}

#[derive(Debug)]
pub enum ChunkTypeError {
    InvalidLength,
    invalidByte(u8),
}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::InvalidLength => {
                write!(f, "Chunk types must be 4 bytes long!")
            }
            ChunkTypeError::invalidByte(b) => {
                write!(f, "invalid bit: {}", b)
            }
        }
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    fn is_critical(&self) -> bool {
        (self.bytes[0] & 0x20) != 0x20
    }
    fn is_public(&self) -> bool {
        (self.bytes[1] & 0x20) != 0x20
    }

    fn is_reserved_bit_valid(&self) -> bool {
        (self.bytes[2] & 0x20) != 0x20
    }
    fn is_safe_to_copy(&self) -> bool {
        (self.bytes[3] & 0x20) == 0x20
    }
    fn is_valid(&self) -> bool {
        for i in self.bytes.as_ref().iter() {
            if !((*i >= 'a' as u8 && *i <= 'z' as u8) || (*i >= 'A' as u8 && *i <= 'Z' as u8)) {
                return false;
            }
        }
        if self.bytes[2] >= 'A' as u8 && self.bytes[2] <= 'Z' as u8 {
            true
        } else {
            false
        }
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    // type Error;
    type Error = ChunkTypeError;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for i in value {
            if !((i >= 'a' as u8 && i <= 'z' as u8) || (i >= 'A' as u8 && i <= 'Z' as u8)) {
                return Err(ChunkTypeError::invalidByte(i));
            }
        }
        Ok(ChunkType { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = ChunkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(ChunkTypeError::InvalidLength);
        }

        let v = s
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|x| *x as u8)
            .collect::<Vec<u8>>();
        let bytes = [v[0], v[1], v[2], v[3]];
        ChunkType::try_from(bytes)
        /* Ok(ChunkType {
            bytes: [v[0], v[1], v[2], v[3]],
        }) */
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from(std::str::from_utf8(&self.bytes).unwrap())
        )
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
        // is_err() likes is_some()
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
