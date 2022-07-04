pub mod kind;
pub mod metadata;
pub mod operation_contents_and_result;
pub mod operation_result;

use {
    self::{
        operation_contents_and_result::activate_account::ActivateAccount,
        operation_contents_and_result::ballot::Ballot,
        operation_contents_and_result::delegation::Delegation,
        operation_contents_and_result::double_baking_evidence::DoubleBakingEvidence,
        operation_contents_and_result::double_endorsement_evidence::DoubleEndorsementEvidence,
        operation_contents_and_result::double_preendorsement_evidence::DoublePreendorsementEvidence,
        operation_contents_and_result::endorsement::Endorsement,
        operation_contents_and_result::origination::Origination,
        operation_contents_and_result::preendorsement::Preendorsement,
        operation_contents_and_result::proposals::Proposals,
        operation_contents_and_result::register_global_constant::RegisterGlobalConstant,
        operation_contents_and_result::reveal::Reveal,
        operation_contents_and_result::seed_nonce_revelation::SeedNonceRevelation,
        operation_contents_and_result::set_deposits_limits::SetDepositsLimits,
        operation_contents_and_result::transaction::Transaction,
    },
    serde::{Deserialize, Serialize},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OperationGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    pub branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub contents: Vec<OperationContent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum OperationContent {
    // Present in alpha protocol
    Endorsement(Endorsement),
    Preendorsement(Preendorsement),
    SeedNonceRevelation(SeedNonceRevelation),
    DoubleEndorsementEvidence(DoubleEndorsementEvidence),
    DoublePreendorsementEvidence(DoublePreendorsementEvidence),
    ActivateAccount(ActivateAccount),
    Proposals(Proposals),
    Ballot(Ballot),
    Reveal(Reveal),
    Transaction(Transaction),
    Origination(Origination),
    Delegation(Delegation),
    RegisterGlobalConstant(RegisterGlobalConstant),
    SetDepositsLimits(SetDepositsLimits),
    // Removed in hangzhou protocol (https://tezos.gitlab.io/protocols/tenderbake.html)
    DoubleBakingEvidence(DoubleBakingEvidence),
}