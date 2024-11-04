use crate::utils;
use serde::{Deserialize, Serialize};

use super::{addresses::Utxo, scripts::Script};

#[derive(Deserialize, Debug, Clone)]
pub struct Certificates {
    pub auth_committee_hot_certs: Vec<serde_json::Value>,
    pub mir_transfers: Vec<serde_json::Value>,
    pub pool_registrations: Vec<serde_json::Value>,
    pub pool_retirements: Vec<serde_json::Value>,
    pub reg_certs: Vec<serde_json::Value>,
    pub reg_drep_certs: Vec<serde_json::Value>,
    pub resign_committee_cold_certs: Vec<serde_json::Value>,
    pub stake_delegations: Vec<serde_json::Value>,
    pub stake_deregistrations: Vec<serde_json::Value>,
    pub stake_reg_delegations: Vec<serde_json::Value>,
    pub stake_registrations: Vec<serde_json::Value>,
    pub stake_vote_delegations: Vec<serde_json::Value>,
    pub stake_vote_reg_delegations: Vec<serde_json::Value>,
    pub unreg_certs: Vec<serde_json::Value>,
    pub unreg_drep_certs: Vec<serde_json::Value>,
    pub update_drep_certs: Vec<serde_json::Value>,
    pub vote_delegations: Vec<serde_json::Value>,
    pub vote_reg_delegations: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Redeemers {
    pub certificates: Vec<serde_json::Value>,
    pub mints: Vec<serde_json::Value>,
    pub spends: Vec<serde_json::Value>,
    pub withdrawals: Vec<serde_json::Value>,
    pub votes: Vec<serde_json::Value>,
    pub proposals: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MintAsset {
    pub unit: String,
    pub amount: serde_json::Value,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionDetail {
    pub additional_signers: Vec<String>,
    pub block_absolute_slot: i64,
    pub block_hash: String,
    pub block_height: i64,
    pub block_timestamp: i64,
    pub block_tx_index: i64,
    pub certificates: Certificates,
    pub collateral_inputs: Vec<Utxo>,
    pub collateral_return: serde_json::Value,
    pub deposit: i64,
    pub fee: i64,
    pub inputs: Vec<Utxo>,
    pub invalid_before: Option<i64>,
    pub invalid_hereafter: Option<i64>,
    pub metadata: serde_json::Value,
    pub mint: Vec<MintAsset>,
    pub outputs: Vec<Utxo>,
    pub redeemers: Vec<Redeemers>,
    pub reference_inputs: Vec<serde_json::Value>,
    pub scripts_executed: Vec<Script>,
    pub scripts_succesful: bool,
    pub size: i64,
    pub tx_hash: String,
    pub withdrawals: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionDetails {
    pub data: TransactionDetail,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionOutputFromReference {
    pub data: Utxo,
    pub last_updated: utils::LastUpdated,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TxoReference {
    pub tx_hash: String,
    pub index: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TransactionOutputsFromReferences {
    pub data: Vec<Utxo>,
    pub last_updated: utils::LastUpdated,
    pub next_cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdditionalUtxo {
    pub tx_hash: String,
    pub index: u32,
    pub txout_cbor: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EvaluateTx {
    pub cbor: String,
    pub additional_utxos: Vec<AdditionalUtxo>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ExecutionUnits {
    pub mem: i64,
    pub steps: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RedeemerEvaluation {
    pub ex_units: ExecutionUnits,
    pub redeemer_index: i32,
    pub redeemer_tag: String,
}

pub type EvaluateTxResponse = Vec<RedeemerEvaluation>;
