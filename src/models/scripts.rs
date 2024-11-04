use crate::utils;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ScriptVersion {
    #[serde(alias = "plutusv1")]
    PlutusV1,
    #[serde(alias = "plutusv2")]
    PlutusV2,
    #[serde(alias = "plutusv3")]
    PlutusV3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    pub bytes: String,
    pub hash: String,
    pub json: serde_json::Value,
    pub r#type: ScriptVersion,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ScriptByHash {
    pub data: Script,
    pub last_updated: utils::LastUpdated,
}
