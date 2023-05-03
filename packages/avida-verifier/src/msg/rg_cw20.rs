use crate::types::{WProof, WSubProofReqParams};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Binary, StdError, StdResult, Uint128};
use cw20::{
    AllAccountsResponse, BalanceResponse, Cw20Coin, DownloadLogoResponse, Logo,
    MarketingInfoResponse,
};

#[cw_serde]
pub enum ExecuteMsg {
    AdapterTransfer {
        sender: Addr,
        recipient: Addr,
        amount: Uint128,
    },
    /// Transfer is a base message to move tokens to another account without triggering actions
    Transfer {
        recipient: String,
        amount: Uint128,
        proof: WProof,
    },
    /// Burn is a base message to destroy tokens forever
    Burn { amount: Uint128, proof: WProof },
    /// Send is a base message to transfer tokens to a contract and trigger an action
    /// on the receiving contract.
    Send {
        contract: String,
        amount: Uint128,
        msg: Binary,
        proof: WProof,
    },
    /// Only with the "mintable" extension. If authorized, creates amount new tokens
    /// and adds to the minter balance.
    Mint {
        amount: Uint128,
        recipient: Addr,
        proof: WProof,
    },
    /// Only with the "marketing" extension. If authorized, updates marketing metadata.
    /// Setting None/null for any of these will leave it unchanged.
    /// Setting Some("") will clear this field on the contract storage
    UpdateMarketing {
        /// A URL pointing to the project behind this token.
        project: Option<String>,
        /// A longer description of the token and it's utility. Designed for tooltips or such
        description: Option<String>,
        /// The address (if any) who can update this data structure
        marketing: Option<String>,
    },
    /// If set as the "marketing" role on the contract, upload a new URL, SVG, or PNG for the token
    UploadLogo(Logo),
}

#[cw_serde]
pub struct InstantiateMarketingInfo {
    pub project: Option<String>,
    pub description: Option<String>,
    pub marketing: Option<String>,
    pub logo: Option<Logo>,
}

#[cw_serde]
pub struct RgMinterData {
    /// If the contract is mintable, the minter is the launchpad
    pub minter: Option<Addr>,
    /// cap is how many more tokens can be issued by the minter
    pub cap: Option<Uint128>,
}

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub initial_balances: Vec<Cw20Coin>,
    pub mint: Option<RgMinterData>,
    pub marketing: Option<InstantiateMarketingInfo>,
    // These are the attributes, predicates  required  for the proof to be verified
    // it is deteremined by the launch of the contract
    pub req_params: Vec<WSubProofReqParams>,
    // These are the trusted issuers who are recognised by this contract who can issue the
    // above defined credentials
    pub trusted_issuers: Vec<String>,
}

impl InstantiateMsg {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }

    pub fn validate(&self) -> StdResult<()> {
        // Check name, symbol, decimals
        if !is_valid_name(&self.name) {
            return Err(StdError::generic_err(
                "Name is not in the expected format (3-50 UTF-8 bytes)",
            ));
        }
        if !is_valid_symbol(&self.symbol) {
            return Err(StdError::generic_err(
                "Ticker symbol is not in expected format [a-zA-Z\\-]{3,12}",
            ));
        }
        if self.decimals > 18 {
            return Err(StdError::generic_err("Decimals must not exceed 18"));
        }
        Ok(())
    }
}

fn is_valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    if bytes.len() < 3 || bytes.len() > 50 {
        return false;
    }
    true
}

fn is_valid_symbol(symbol: &str) -> bool {
    let bytes = symbol.as_bytes();
    if bytes.len() < 3 || bytes.len() > 12 {
        return false;
    }
    for byte in bytes.iter() {
        if (*byte != 45) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
            return false;
        }
    }
    true
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns the current balance of the given address, 0 if unset.
    /// Return type: BalanceResponse.
    #[returns(BalanceResponse)]
    Balance { address: String },
    /// Returns the nonce for the next proof for this account owner
    #[returns(u64)]
    ProofNonce { address: String },
    /// Returns metadata on the contract - name, decimals, supply, verifier.
    /// Return type: TokenInfoResponse.
    #[returns(TokenInfoResponse)]
    TokenInfo {},
    #[returns(RgMinterData)]
    Minter {},
    #[returns(AllAccountsResponse)]
    AllAccounts {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(MarketingInfoResponse)]
    MarketingInfo {},
    #[returns(DownloadLogoResponse)]
    DownloadLogo {},
    #[returns(Vec<String>)]
    TrustedIssuers {},
}

#[cw_serde]
pub struct TokenInfoResponse {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub verifier: String,
}
