use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use secret_toolkit::storage::Item;

use cosmwasm_std::Addr;

/// Basic configuration struct
pub static CONFIG_KEY: Item<Config> = Item::new(b"config");
/// Revoked permits prefix key
pub const PREFIX_REVOKED_PERMITS: &str = "revoked_permits";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub contract_address: Addr,
}
