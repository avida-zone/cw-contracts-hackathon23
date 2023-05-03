use crate::msg::rg_cw20::InstantiateMsg as RgCw20InstantiateMsg;
pub use crate::state::launchpad::{LaunchType, LaunchpadOptions};
use crate::types::WProof;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub rg_cw20_code_id: u64,
}

#[cw_serde]
pub enum ExecuteMsg {
    Launch {
        /// Details for the RgCw20 Contract
        msg: RgCw20InstantiateMsg,
        /// Label for the RgCw20 Contract
        label: String,
        /// If it is a new token or transformed from existing
        launch_type: LaunchType,
    },
    Mint {
        /// The rgToken to be minted
        rg_token_addr: String,
        /// The amount to be minted
        amount: Uint128,
        /// The proof to be verified
        proof: WProof,
    },
    Transform {
        /// The rgToken to be transformed into
        rg_token_addr: String,
        /// The proof to be verified
        proof: WProof,
    },
    Revert {
        /// The amount to revert to non-rgToken
        amount: Uint128,
        /// The recipient (unchecked since non-rg)
        recipient: String,
    },
    UpdateVerifier {
        /// The address of the verifier
        address: String,
    },
    UpdateAdapter {
        /// The address of the TG Adapter
        address: String,
    },
    UpdateFee {
        /// The address of the TG Adapter
        fee: Uint128,
    },
    UpdateRgTokenCodeId {
        /// The address of the TG Adapter
        id: u64,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Vec<ContractResponse>)]
    RegisteredContracts {
        start_after: Option<String>,
        limit: Option<u64>,
        contract_type: ContractType,
    },
    #[returns(Addr)]
    Verifier {},
    #[returns(Addr)]
    Adapter {},
}

#[cw_serde]
pub enum ContractType {
    New,
    Transform,
}

#[cw_serde]
pub struct ContractResponse {
    pub contract_address: Addr,
    pub options: LaunchpadOptions,
}
