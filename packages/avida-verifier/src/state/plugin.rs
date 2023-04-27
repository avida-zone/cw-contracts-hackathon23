use crate::types::WCredentialPubKey;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

/// Self issued credential definition
pub const SELF_ISSUED_CRED_DEF: Item<WCredentialPubKey> = Item::new("self issued cred def");

/// The Vectis Account that instantiated this
pub const VECTIS_ACCOUNT: Item<Addr> = Item::new("vectis account linked to this plugin");
