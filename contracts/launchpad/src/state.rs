pub use avida_verifier::state::launchpad::{
    LaunchpadOptions, ADAPTER, RG_CONTRACTS, RG_TRANSFORM, VERIFIER,
};
use cosmwasm_std::{CanonicalAddr, Coin};
use cw_storage_plus::Item;

pub const INST_FEE: Item<Coin> = Item::new("instantiate fee");
pub const TRANSFORM_FEE: Item<Coin> = Item::new("transform fee");
pub const DEPLOYER: Item<CanonicalAddr> = Item::new("deployer");
pub const RG_CW_20_CODE_ID: Item<u64> = Item::new("rg_cw20_code_id");
// tmp state instantiate
pub const PENDING_INST: Item<LaunchpadOptions> = Item::new("pending state");
