use std::convert::TryFrom;
use std::str::FromStr;

use std::fmt;
use std::str;
use crate::{Error, Result};
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    data: [u8; 4],
}

///! 块类型中，每个字节的5位为0，这个字节就是大写，否则小写
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        return self.data.try_into().unwrap();
    }

    /// Returns the property state of the first byte as described in the PNG spec
    /// 块类型，第一个字节的5位，0-关键数据块，1-辅助数据块
    pub fn is_critical(&self) -> bool {
        let a = self.data[0] & 32;
        (a >> 5) == 0
    }

    /// Returns the property state of the second byte as described in the PNG spec
    /// 块类型，第二个字节的5位，0-公开，1-私有
    pub fn is_public(&self) -> bool {
        let a = self.data[1] & 32;
        (a >> 5) == 0
    }
    /// Returns the property state of the third byte as described in the PNG spec
    /// 块类型，第三个字节的5位，必须为0
    pub fn is_reserved_bit_valid(&self) -> bool {
        let a = self.data[2] & 32;
        let d = a >> 5;
        (d) == 0
    }
    /// Returns the property state of the fourth byte as described in the PNG spec
    /// 块类型，第四个字节的5位，0-复制不安全，1-复制安全
    pub fn is_safe_to_copy(&self) -> bool {
        let a = self.data[3] & 32;
        (a >> 5) == 1

    }
    /// Returns true if the reserved byte is valid and all four bytes are represented by the characters A-Z or a-z.
    /// Note that this chunk type should always be valid as it is validated during construction.
    pub fn is_valid(&self) -> bool {
        if !self.is_reserved_bit_valid() {
            return false;
        }
        let r3 = Regex::new(r"^[a-zA-Z]*$").unwrap(); // 正则匹配a-zA-Z
        let data = &self.data;
        let da = str::from_utf8(data).unwrap();
        if r3.is_match(da) {
            return true;
        }
        false
    }

    /// Valid bytes are represented by the characters A-Z or a-z
    pub fn is_valid_byte(byte: u8) -> bool { // 
        let d = [byte; 1];
        let r3 = Regex::new(r"^[a-zA-Z]*$").unwrap(); // 正则匹配a-zA-Z
        let da = str::from_utf8(&d).unwrap();
        if r3.is_match(da) {
            return true;
        }
        false
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        Ok(ChunkType { data: bytes })
    }
}

impl fmt::Display for ChunkType {
    //
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.data).unwrap())
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {

        let r3 = Regex::new(r"^[a-zA-Z]*$").unwrap(); // 正则匹配a-zA-Z
        if r3.is_match(s) {
            let c = ChunkType {
                data: s.as_bytes().try_into().unwrap(),
            };
            return Ok(c);
        }
        return Err("格式错误".into());
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
