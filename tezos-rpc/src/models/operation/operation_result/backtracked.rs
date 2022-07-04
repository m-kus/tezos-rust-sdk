use {
    super::{big_map_diff::BigMapDiff, lazy_storage_diff::LazyStorageDiff, Status},
    crate::{models::balance_update::BalanceUpdate, models::error::RPCError},
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Backtracked {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RPCError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub big_map_diff: Option<Vec<BigMapDiff>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_updates: Option<Vec<BalanceUpdate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originated_contracts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_gas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumed_milligas: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_storage_size_diff: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_destination_contract: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy_storage_diff: Option<Vec<LazyStorageDiff>>,
}