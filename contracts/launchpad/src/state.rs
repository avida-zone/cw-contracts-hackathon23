use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::Item;

pub const DEPLOYER: Item<CanonicalAddr> = Item::new("deployer");

pub const MULTISIG_PLUGIN: Item<CanonicalAddr> = Item::new("multisig-plugin");

pub const RG_CW_20_CODE_ID: Item<u64> = Item::new("rg_cw20_code_id");
