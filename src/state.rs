use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::Item;

use cosmwasm_std::Addr;

/// Basic configuration struct
pub static CONFIG_KEY: Item<Config> = Item::new(b"config");
/// Revoked permits prefix key
pub static PREFIX_REVOKED_PERMITS: Item<String> = Item::new(b"revoked");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub contract_address: Addr,
}
