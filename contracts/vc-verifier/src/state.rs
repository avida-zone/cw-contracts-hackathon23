use cw_controllers::Admin;
use cw_storage_plus::Item;
pub use ursa::cl::{
    CredentialPublicKey, CredentialSchema, NonCredentialSchema, RevocationKeyPublic,
    RevocationRegistry, SubProofRequest,
};
use vectis_verifier::types::SubProofReqParams;

pub const ADMIN: Admin = Admin::new("admin");
pub const SUB_PROOF_REQ_PARAMS: Item<Vec<SubProofReqParams>> = Item::new("sub_proof_requests");
pub const WALLET_SUB_PROOF_REQ: Item<SubProofRequest> = Item::new("wallet_sub_proof_request");
pub const WALLET_CRED_SCHEMA: Item<CredentialSchema> = Item::new("wallet_cred_schema");
pub const WALLET_NON_CRED_SCHEMA: Item<NonCredentialSchema> = Item::new("wallet_non_cred_schema");
