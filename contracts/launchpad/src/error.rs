use cosmwasm_std::{Coin, StdError};
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("StdError {0}")]
    Std(#[from] StdError),

    #[error("ParseReplyError {0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("Not implemented")]
    NotImplemented,

    #[error("Uint128 Overflow")]
    Overflow,

    #[error("Transformer Does Not Have Mint Price")]
    TransformerDoesNotHaveMintPrice,

    #[error("rgToken not mintable")]
    NotMintable,

    #[error("Multiple Denom")]
    MultipleDenom,

    #[error("Invalid Denom")]
    InvalidDenom,

    #[error("Unexpected Launch Type")]
    UnexpectedLaunchType,

    #[error("IncorrectFunds")]
    IncorrectFunds,

    #[error("Token Addr Validation  {0}")]
    TokenAddrValidation(String),

    #[error("Unauthorised")]
    Unauthorised,

    #[error("Fee required {0}")]
    FeeRequied(Coin),
}
