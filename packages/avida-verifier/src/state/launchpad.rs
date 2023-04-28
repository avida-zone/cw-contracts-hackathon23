use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const VERIFIER: Item<Addr> = Item::new("verifier");
