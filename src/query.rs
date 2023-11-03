use cosmwasm_std::{Deps, StdResult};
use cw_storage_plus::Bound;

use crate::state::{all_secret_ids, Secret, SECRETS};


// Default and Max page sizes for paginated queries
const MAX_LIMIT: u32 = 100;
const DEFAULT_LIMIT: u32 = 10;

pub fn query_secret_ids(
    deps: Deps, 
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<Vec<String>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start = start_after.as_ref().map(|s| Bound::exclusive(s.as_str()));
    let secret_list: Vec<String> = all_secret_ids(deps.storage, start, limit)?;
    Ok(secret_list)
}

pub fn query_details(deps: Deps, id: String) -> StdResult<Secret> {
    let details: Secret = SECRETS.load(deps.storage, &id)?;
    Ok(details)
}