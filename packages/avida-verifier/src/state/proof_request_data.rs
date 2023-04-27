use crate::types::SubProofReqParams;
use cw_storage_plus::Item;
pub use ursa::cl::{
    CredentialPublicKey, CredentialSchema, NonCredentialSchema, RevocationKeyPublic,
    RevocationRegistry, SubProofRequest,
};

/// Contains all the subproof request
pub const SUB_PROOF_REQ_PARAMS: Item<Vec<SubProofReqParams>> = Item::new("sub_proof_requests");

/// Minimum params for satisfying any transaction in AVIDA framework
/// This is to proof of the smart contract account ownership
pub const VECTIS_SUB_PROOF_REQ: Item<SubProofRequest> = Item::new("vectis_sub_proof_request");
pub const VECTIS_CRED_SCHEMA: Item<CredentialSchema> = Item::new("vectis_cred_schema");
pub const VECTIS_NON_CRED_SCHEMA: Item<NonCredentialSchema> = Item::new("vectis_non_cred_schema");
