use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Uint128};
use cw_storage_plus::{Item, Map};

/// The address of the vc-verifier
pub const VERIFIER: Item<Addr> = Item::new("verifier");
/// The contracts deployed by the launchpad and its mintable price
pub const RG_CONTRACTS: Map<Addr, LaunchpadOptions> = Map::new("deployed-rg-token");
/// The contracts deployed by the launchpad for transforming native tokens to rg tokens
pub const RG_TRANSFORM: Map<Addr, LaunchpadOptions> = Map::new("transformed-rg-token");
/// The adaptor for transforming into Native token
pub const ADAPTOR: Item<Addr> = Item::new("adaptor");

#[cw_serde]
pub struct LaunchpadOptions {
    pub launch_type: LaunchType,
    pub originator: Addr,
}

#[cw_serde]
pub struct MintOptions {
    pub price: Vec<Coin>,
    pub cap: Option<Uint128>,
}

#[cw_serde]
pub enum LaunchType {
    New(MintOptions),
    Transform(String),
}
