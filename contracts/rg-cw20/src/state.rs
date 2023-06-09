use crate::contract::*;
use crate::msg::ExecuteMsg;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, MessageInfo, Uint128};
use cw20::{AllowanceResponse, Logo, MarketingInfoResponse};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub mint: Option<RgMinterData>,
}

impl TokenInfo {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }
}

pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");

// Avida specific fields
/// The list of trusted issuer for the SubProofReqParams credentials
pub const TRUSTED_ISSUERS: Item<Vec<String>> = Item::new("trusted issuers");
/// The subproofs required as part of the proof to be verified
pub use avida_verifier::state::proof_request_data::SUB_PROOF_REQ_PARAMS;
///  The pending transation message whilst Avida verifier is verifying
pub const PENDING_VERIFICATION: Item<(MessageInfo, ExecuteMsg)> = Item::new("pending-verification");
///  Increments everytime a send / transfer tx is done
pub const VC_NONCE: Map<&Addr, u64> = Map::new("nonce");
/// The address of the Avida launchpad contract
pub const LAUNCHPAD: Item<Addr> = Item::new("launchpad");
