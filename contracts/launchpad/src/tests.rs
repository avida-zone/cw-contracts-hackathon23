use crate::{contract::*, msg::*, state::*};
pub use avida_verifier::{state::launchpad::MintOptions, types::WProof};
use cosmwasm_std::{
    coin,
    testing::{mock_dependencies, mock_env, mock_info},
    Addr, Coin, CosmosMsg, SubMsg, Uint128,
};

#[test]
fn query_returns_correct_conntract_transform_only() {
    let mut deps = mock_dependencies();
    let token_addr = Addr::unchecked("some-addr");
    let originator = Addr::unchecked("some-other-addr");
    let launch_type = LaunchType::Transform("some-denom".into());

    let options = LaunchpadOptions {
        originator,
        launch_type,
    };

    RG_TRANSFORM
        .save(deps.as_mut().storage, token_addr.clone(), &options)
        .unwrap();

    let res = query_contract(deps.as_ref(), token_addr.to_string()).unwrap();
    assert_eq!(res.contract_address, token_addr)
}

#[test]
fn query_returns_correct_conntract_new_only() {
    let mut deps = mock_dependencies();
    let token_addr = Addr::unchecked("some-addr");
    let originator = Addr::unchecked("some-other-addr");
    let launch_type = LaunchType::New(MintOptions {
        cap: Some(Uint128::new(123u128)),
        price: vec![coin(10u128, "test")],
    });

    let options = LaunchpadOptions {
        originator: originator,
        launch_type: launch_type.clone(),
    };

    RG_CONTRACTS
        .save(deps.as_mut().storage, token_addr.clone(), &options)
        .unwrap();

    let res = query_contract(deps.as_ref(), token_addr.to_string()).unwrap();
    assert_eq!(res.options.launch_type, launch_type);
    assert_eq!(res.contract_address, token_addr)
}

#[test]
#[should_panic]
fn query_does_not_return_on_zero_contracts() {
    let deps = mock_dependencies();
    let token_addr = Addr::unchecked("some-addr");
    query_contract(deps.as_ref(), token_addr.to_string()).unwrap();
}

#[test]
fn can_mint_new_tokens_with_funds() {
    let mint_amount = Uint128::new(1u128);
    let mint_price = coin(10u128, "test");
    let funds = coin(10u128, "test");
    let not_enough_funds = coin(1u128, "test");
    let too_much_funds = coin(100u128, "test");
    mint_setup(mint_price.clone(), mint_amount, funds).unwrap();
    mint_setup(mint_price.clone(), mint_amount, not_enough_funds).unwrap_err();
    mint_setup(mint_price, mint_amount, too_much_funds).unwrap_err();
}

fn mint_setup(
    mint_price: Coin,
    mint_amount: Uint128,
    funds: Coin,
) -> Result<Response, ContractError> {
    let mut deps = mock_dependencies();
    let token_addr = Addr::unchecked("some-addr");
    let originator = Addr::unchecked("some-other-addr");
    let launch_type = LaunchType::New(MintOptions {
        cap: Some(Uint128::new(123u128)),
        price: vec![mint_price.clone()],
    });

    let info = mock_info("sender", &[funds]);
    let options = LaunchpadOptions {
        originator,
        launch_type: launch_type.clone(),
    };

    RG_CONTRACTS
        .save(deps.as_mut().storage, token_addr.clone(), &options)
        .unwrap();

    let proof = WProof::mock();

    exec_mint(
        deps.as_mut(),
        info,
        token_addr.to_string(),
        mint_amount,
        proof,
    )
}
