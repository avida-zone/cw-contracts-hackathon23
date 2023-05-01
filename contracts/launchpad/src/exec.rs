use avida_verifier::{
    msg::rg_cw20::{InstantiateMsg as RgCw20InstantiateMsg, RgMinterData},
    types::WProof,
};
use cosmwasm_std::{BankMsg, Coin};

use crate::contract::*;

pub fn instantiate_rg_cw20(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mut msg: RgCw20InstantiateMsg,
    label: String,
    launch_type: LaunchType,
) -> Result<Response, ContractError> {
    let reply_id = match launch_type.clone() {
        LaunchType::New(options) => {
            msg.mint = Some(RgMinterData {
                minter: Some(env.contract.address.clone()),
                cap: options.cap,
            });
            INST_REPLY_ID
        }
        LaunchType::Transform(_) => {
            msg.mint = Some(RgMinterData {
                minter: Some(env.contract.address.clone()),
                cap: None,
            });
            TRANS_REPLY_ID
        }
    };

    // We save this to list the token after it has been instantiated successfully
    PENDING_INST.save(
        deps.storage,
        &LaunchpadOptions {
            launch_type,
            originator: info.sender.clone(),
        },
    )?;

    // The wasm message containing the `wallet_proxy` instantiation message
    let instantiate_msg = WasmMsg::Instantiate {
        admin: Some(info.sender.to_string()),
        code_id: RG_CW_20_CODE_ID.load(deps.storage)?,
        msg: to_binary(&msg)?,
        funds: vec![],
        label,
    };

    let msg = SubMsg::reply_on_success(instantiate_msg, reply_id);
    let res = Response::new().add_submessage(msg);
    Ok(res)
}

pub fn exec_mint(
    deps: DepsMut,
    info: MessageInfo,
    rg_token_addr: String,
    amount: Uint128,
    proof: WProof,
) -> Result<Response, ContractError> {
    let validated_addr = deps.api.addr_validate(&rg_token_addr)?;
    let launch_option = RG_CONTRACTS.load(deps.storage, validated_addr)?;
    if let LaunchType::New(options) = launch_option.launch_type {
        if info.funds.len() != 1 {
            Err(ContractError::MultipleDenom)
        } else {
            // Check exact funds has been sent
            let price = options.price;
            let mut funds = info.funds[0];
            let divided = info.funds[0].amount.checked_div(amount)?;
            funds.amount = divided;
            if !price.contains(&funds) {
                Err(ContractError::MultipleDenom)
            } else {
                let msg = avida_verifier::msg::rg_cw20::ExecuteMsg::Mint {
                    amount,
                    recipient: info.sender,
                    proof,
                };
                let mint_msg = WasmMsg::Execute {
                    contract_addr: rg_token_addr,
                    msg: to_binary(&msg)?,
                    funds: vec![],
                };

                let to_originator = BankMsg::Send {
                    to_address: launch_option.originator.to_string(),
                    amount: info.funds,
                };

                Ok(Response::new()
                    .add_message(mint_msg)
                    .add_message(to_originator))
            }
        }
    } else {
        Err(ContractError::NotMintable)
    }
}

pub fn exec_transform(
    deps: DepsMut,
    info: MessageInfo,
    rg_token_addr: String,
    proof: WProof,
) -> Result<Response, ContractError> {
    // check this is one token only, and that the denom is expected
    let validated_addr = deps.api.addr_validate(&rg_token_addr)?;
    let launch_option = RG_TRANSFORM.load(deps.storage, validated_addr)?;
    if let LaunchType::Transform(denom) = launch_option.launch_type {
        if info.funds.len() != 1 {
            Err(ContractError::MultipleDenom)
        } else {
            if info.funds[0].denom != denom {
                Err(ContractError::InvalidDenom)
            } else {
                let msg = avida_verifier::msg::rg_cw20::ExecuteMsg::Mint {
                    amount: info.funds[0].amount,
                    recipient: info.sender.to_string(),
                    proof,
                };
                let mint_msg = WasmMsg::Execute {
                    contract_addr: rg_token_addr,
                    msg: to_binary(&msg)?,
                    funds: vec![],
                };
                Ok(Response::new().add_message(mint_msg))
            }
        }
    } else {
        Err(ContractError::NotMintable)
    }
}

pub fn exec_revert(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    recipient: String,
) -> Result<Response, ContractError> {
    // This message can only be sent from rgToken that were transformed,
    // on the side of the rgToken it is assumed to be burnt
    // This contract will not hold rgTokens
    let options = RG_TRANSFORM.load(deps.storage, info.sender)?;
    if let LaunchType::Transform(denom) = options.launch_type {
        let msg = BankMsg::Send {
            to_address: recipient,
            amount: vec![Coin { denom, amount }],
        };
        Ok(Response::new().add_message(msg))
    } else {
        Err(ContractError::UnexpectedLaunchType)
    }
}
