use regex::Regex;

use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HexString(String);

impl HexString {
    const PREFIX: &'static str = "0x";

    pub fn new(value: String) -> Result<Self> {
        if Self::is_valid(value.as_str()) {
            return Ok(Self(value));
        }
        Err(Error::InvalidHexString)
    }

    pub fn len(&self) -> usize {
        if self.0.starts_with(Self::PREFIX) {
            return self.0.len() - Self::PREFIX.len();
        }
        self.0.len()
    }

    pub fn len_with_prefix(&self) -> usize {
        if self.0.starts_with(Self::PREFIX) {
            return self.0.len();
        }
        self.0.len() + Self::PREFIX.len()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        hex::decode(&self.0).unwrap()
    }

    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Self {
        let value = hex::encode(bytes);
        Self(value)
    }

    pub fn into_string(self, with_prefix: bool) -> String {
        match (with_prefix, self.0.starts_with(Self::PREFIX)) {
            (true, true) | (false, false) => self.0,
            (true, false) => format!("0x{}", self.0),
            (false, true) => self.0[2..].into(),
        }
    }

    fn is_valid(value: &str) -> bool {
        let re = Regex::new("^(0x)?([0-9a-fA-F]{2})*$").unwrap();
        re.is_match(value)
    }
}

impl From<&[u8]> for HexString {
    fn from(bytes: &[u8]) -> Self {
        Self::from_bytes(bytes)
    }
}

impl TryFrom<String> for HexString {
    type Error = Error;

    fn try_from(value: String) -> Result<Self> {
        Self::new(value)
    }
}

impl TryFrom<&str> for HexString {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        Self::new(value.into())
    }
}