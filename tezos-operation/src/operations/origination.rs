use tezos_core::types::{encoded::ImplicitAddress, mutez::Mutez, number::Nat};
use tezos_michelson::micheline::Micheline;

use super::{OperationContentTag, TraitOperationContent, TraitOperationManagerContent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origination {
    source: ImplicitAddress,
    fee: Mutez,
    counter: Nat,
    gas_limit: Nat,
    storage_limit: Nat,
    balance: Mutez,
    delegate: Option<ImplicitAddress>,
    script: Script,
}

impl Origination {
    pub fn balance(&self) -> Mutez {
        self.balance
    }

    pub fn delegate(&self) -> Option<&ImplicitAddress> {
        self.delegate.as_ref()
    }

    pub fn script(&self) -> &Script {
        &self.script
    }

    pub fn new(
        source: ImplicitAddress,
        fee: Mutez,
        counter: Nat,
        gas_limit: Nat,
        storage_limit: Nat,
        balance: Mutez,
        delegate: Option<ImplicitAddress>,
        script: Script,
    ) -> Self {
        Self {
            source,
            fee,
            counter,
            gas_limit,
            storage_limit,
            balance,
            delegate,
            script,
        }
    }
}

impl TraitOperationContent for Origination {
    fn tag() -> OperationContentTag {
        OperationContentTag::Origination
    }
}

impl TraitOperationManagerContent for Origination {
    fn source(&self) -> &ImplicitAddress {
        &self.source
    }

    fn fee(&self) -> Mutez {
        self.fee
    }

    fn counter(&self) -> &Nat {
        &self.counter
    }

    fn gas_limit(&self) -> &Nat {
        &self.gas_limit
    }

    fn storage_limit(&self) -> &Nat {
        &self.storage_limit
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Script {
    code: Micheline,
    storage: Micheline,
}

impl Script {
    pub fn code(&self) -> &Micheline {
        &self.code
    }

    pub fn storage(&self) -> &Micheline {
        &self.storage
    }

    pub fn new(code: Micheline, storage: Micheline) -> Self {
        Self { code, storage }
    }
}
