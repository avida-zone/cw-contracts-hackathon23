use cosmwasm_std::StdError;
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
}
