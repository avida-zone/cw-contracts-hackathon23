use cosmwasm_std::Addr;
use cw_storage_plus::KeyDeserialize;
use ursa::bn::BigNumber;

#[test]
fn should_build_addr_with_vec() {
    let controller: &str = "inj1fs9skvgmwupw8qrk9flt4j5xryelz7z8k75ntk";
    let encoded_controller: &str = "57651176141717037701224828889098602471262907572213485092025342065362109513119512615244595633715049579";

    let bn_str = BigNumber::from_dec(encoded_controller).unwrap();

    let addr = Addr::from_vec(bn_str.to_bytes().unwrap()).unwrap();

    assert_eq!(controller, addr);
}

#[test]
fn test_proof_verify() {
    let controller: &str = "inj1fs9skvgmwupw8qrk9flt4j5xryelz7z8k75ntk";
    let encoded_controller: &str = "57651176141717037701224828889098602471262907572213485092025342065362109513119512615244595633715049579";

    //fn proof_verify(
    //    storage: &dyn Storage,
    //    s_proof: WProof,
    //    s_proof_req_nonce: BigNumberBytes,
    //    wallet_cred_pub_key: CredentialPublicKey,
    //    controller_addr: Addr,
    //    sub_proof_requests: Vec<SubProofReqParams>,
    //) -> Result<bool, ContractError> {
}
