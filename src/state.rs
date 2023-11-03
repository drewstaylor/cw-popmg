use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    Addr, Order, Storage, StdResult, Uint128,
};
use cw_storage_plus::{
    Bound, Item, Map,
};

// Admin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub nft_contract: Addr,
}
pub const CONFIG: Item<Config> = Item::new("config");


// Secrets
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Secret {
    pub id: String, 
    pub secret: String,
    pub depth: u32,
    pub rewards: Option<Uint128>,
    pub claimed: bool,
}

pub fn all_secret_ids<'a>(
    storage: &dyn Storage,
    start: Option<Bound<'a, &'a str>>,
    limit: usize,
) -> StdResult<Vec<String>> {
    SECRETS
        .keys(storage, start, None, Order::Ascending)
        .take(limit)
        .collect()
}
pub const SECRETS: Map<&str, Secret> = Map::new("secret");