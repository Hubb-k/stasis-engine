use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use thiserror::Error;

const MAGIC: &[u8; 4] = b"UMBR";
const VERSION: u32 = 3;

#[derive(Error, Debug)]
pub enum CrystalError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializeError(#[from] bincode::Error),

    #[error("Invalid magic bytes")]
    InvalidMagic,

    #[error("Unsupported version: {0}")]
    UnsupportedVersion(u32),
}

pub trait Stasis: Serialize + for<'a> Deserialize<'a> {}
impl<T: Serialize + for<'a> Deserialize<'a>> Stasis for T {}

pub struct Crystalizer;

impl Crystalizer {
    pub fn freeze<T: Stasis>(path: &str, data: &T) -> Result<(), CrystalError> {
        let mut file = File::create(path)?;
        file.write_all(MAGIC)?;
        file.write_all(&VERSION.to_le_bytes())?;
        file.write_all(&bincode::serialize(data)?)?;
        Ok(())
    }

    pub fn thaw<T: Stasis>(path: &str) -> Result<T, CrystalError> {
        let mut file = File::open(path)?;

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;
        if &magic != MAGIC {
            return Err(CrystalError::InvalidMagic);
        }

        let mut version_bytes = [0u8; 4];
        file.read_exact(&mut version_bytes)?;
        let version = u32::from_le_bytes(version_bytes);
        if version != VERSION {
            return Err(CrystalError::UnsupportedVersion(version));
        }

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let data = bincode::deserialize(&buffer)?;

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestData {
        value: i32,
        name: String,
    }

    #[test]
    fn test_freeze_thaw_roundtrip() {
        let data = TestData {
            value: 42,
            name: "test".to_string(),
        };

        let path = "test_stasis.bin";
        Crystalizer::freeze(path, &data).unwrap();
        let loaded: TestData = Crystalizer::thaw(path).unwrap();

        assert_eq!(data, loaded);
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_invalid_magic() {
        let path = "test_bad_magic.bin";
        std::fs::write(path, b"XXXX0000").unwrap();
        let result: Result<TestData, _> = Crystalizer::thaw(path);
        assert!(matches!(result, Err(CrystalError::InvalidMagic)));
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_unsupported_version() {
        let path = "test_bad_version.bin";
        let mut file = File::create(path).unwrap();
        file.write_all(b"UMBR").unwrap();
        file.write_all(&999u32.to_le_bytes()).unwrap();
        drop(file);

        let result: Result<TestData, _> = Crystalizer::thaw(path);
        assert!(matches!(result, Err(CrystalError::UnsupportedVersion(999))));
        std::fs::remove_file(path).ok();
    }

    #[test]
    fn test_primitive_types() {
        let path = "test_tuple.bin";
        let tuple = (100u64, 5usize, 3.14f64, 2.71f64);
        Crystalizer::freeze(path, &tuple).unwrap();
        let loaded: (u64, usize, f64, f64) = Crystalizer::thaw(path).unwrap();
        assert_eq!(tuple, loaded);
        std::fs::remove_file(path).ok();
    }
}
