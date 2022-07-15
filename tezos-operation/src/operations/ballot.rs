use tezos_core::{
    internal::types::BytesTag,
    types::encoded::{ImplicitAddress, ProtocolHash},
};

use super::{OperationContentTag, TraitOperationContent};
use crate::{Error, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ballot {
    source: ImplicitAddress,
    period: i32,
    proposal: ProtocolHash,
    ballot: Type,
}

impl Ballot {
    pub fn source(&self) -> &ImplicitAddress {
        &self.source
    }

    pub fn period(&self) -> i32 {
        self.period
    }

    pub fn proposal(&self) -> &ProtocolHash {
        &self.proposal
    }

    pub fn ballot(&self) -> Type {
        self.ballot
    }

    pub fn new(source: ImplicitAddress, period: i32, proposal: ProtocolHash, ballot: Type) -> Self {
        Self {
            source,
            period,
            proposal,
            ballot,
        }
    }
}

impl TraitOperationContent for Ballot {
    fn tag() -> OperationContentTag {
        OperationContentTag::Ballot
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Yay,
    Nay,
    Pass,
}

impl Type {
    pub fn from_value(value: &[u8]) -> Result<Self> {
        match value {
            &[0] => Ok(Self::Yay),
            &[1] => Ok(Self::Nay),
            &[2] => Ok(Self::Pass),
            _ => Err(Error::InvalidBytes),
        }
    }
}

impl BytesTag for Type {
    fn value(&self) -> &'static [u8] {
        match self {
            Self::Yay => &[0],
            Self::Nay => &[1],
            Self::Pass => &[2],
        }
    }
}
