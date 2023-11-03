use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub cw721: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddSecret(AddMsg),
    // SubmitProof(ProveMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AddMsg {
    pub id: String,
    pub secret: String,
    pub depth: u32,
    pub rewards: Option<Uint128>,
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProveMsg {
    pub id: String,
    pub proof: String,
    pub depth: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// List all secret IDs
    SecretIds {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Get details for a specified secret
    Details {
        id: String,
    },
}