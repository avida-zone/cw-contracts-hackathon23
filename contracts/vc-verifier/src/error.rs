use avida_verifier::types::TypeConversionError;
use cosmwasm_std::StdError;
use thiserror::Error;
use ursa::errors::UrsaCryptoError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Address cannot be validated")]
    NotAddress {},

    #[error("Verification error: {0}")]
    CannotExecuteVerify(String),

    #[error("SubProofRequestBuilder")]
    SubProofRequestBuilder {},

    #[error("Serde: {0}")]
    Serde(String),

    #[error("UrsaCryptoError: {0}")]
    UrsaCryptoError(String),

    #[error("Not AVIDA")]
    NotAvida,

    #[error("BN conversion")]
    BigNumberConversionFromDec {},

    #[error("Converstion {0}")]
    Conversion(String),

    #[error("Missing Revocation Registry")]
    MissingRevReg {},

    #[error("TypeConversion {0}")]
    TypeConversion(#[from] TypeConversionError),

    #[error("Missing wallet attribute")]
    MissingWalletAttr {},

    #[error("Invalid Wallet Proof")]
    InvalidWalletProof {},

    #[error("No Identity Plugin found on Vectis Account")]
    NoIdentityPlugin,
}

impl From<UrsaCryptoError> for ContractError {
    fn from(source: UrsaCryptoError) -> Self {
        Self::UrsaCryptoError(source.to_string())
    }
}
