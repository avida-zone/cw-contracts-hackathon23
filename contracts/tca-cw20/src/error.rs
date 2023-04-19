use cosmwasm_std::StdError;
use thiserror::Error;
use vc_verifier::error::ContractError as VCVerifierError;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Cannot set to own account")]
    CannotSetOwnAccount {},

    #[error("Invalid zero amount")]
    InvalidZeroAmount {},

    #[error("Allowance is expired")]
    Expired {},

    #[error("No allowance for this account")]
    NoAllowance {},

    #[error("Minting cannot exceed the cap")]
    CannotExceedCap {},

    #[error("Logo binary data exceeds 5KB limit")]
    LogoTooBig {},

    #[error("Invalid xml preamble for SVG")]
    InvalidXmlPreamble {},

    #[error("Invalid png header")]
    InvalidPngHeader {},

    #[error("Duplicate initial balance addresses")]
    DuplicateInitialBalanceAddresses {},

    #[error("Proof Verification Error: {0}")]
    VCVError(#[from] VCVerifierError),

    #[error("Proof Invalid")]
    ProofInvalid {},
}
