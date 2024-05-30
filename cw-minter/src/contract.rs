use crate::error::ContractError;
use crate::msg::{AdminResp, ExecuteMsg, InstantiateMsg, QueryMsg, RelayerResp};
use crate::state::{MintAttempt, MINT_ATTEMPTS, RELAYER_ASSOCIATED_ADDR, RELAYER_POINTER_ADDR};
use cosmwasm_std::{
    coin, entry_point, to_json_binary, wasm_execute, BankMsg, Binary, Deps, DepsMut, Env,
    MessageInfo, Response,
};
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use cw_ownable::{get_ownership, initialize_owner};

const SUPPORTED_DENOM: &str = "usei";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<AdminResp, ContractError> {
    let admin = deps.api.addr_validate(&msg.admin)?;
    initialize_owner(deps.storage, deps.api, Some(admin.as_str()))?;
    Ok(AdminResp { admin })
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRelayer {
            pointer_address,
            associated_address,
        } => execute_set_relayer(deps, info, pointer_address, associated_address),
        ExecuteMsg::Mint {
            recipient,
            quantity,
        } => execute_mint(deps, info, recipient, quantity),
        ExecuteMsg::UpdateOwnership(action) => update_ownership(deps, env, info, action),
    }
}

pub fn update_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: cw_ownable::Action,
) -> Result<Response, ContractError> {
    let ownership = cw_ownable::update_ownership(deps, &env.block, &info.sender, action)?;
    Ok(Response::new().add_attributes(ownership.into_attributes()))
}

pub fn execute_set_relayer(
    deps: DepsMut,
    _info: MessageInfo,
    pointer_address: String,
    associated_address: String,
) -> Result<Response, ContractError> {
    let pointer_addr = deps.api.addr_validate(&pointer_address)?;
    let associated_addr = deps.api.addr_validate(&associated_address)?;
    RELAYER_POINTER_ADDR.save(deps.storage, &pointer_addr)?;
    RELAYER_ASSOCIATED_ADDR.save(deps.storage, &associated_addr)?;
    Ok(Response::new())
}

pub fn execute_mint(
    deps: DepsMut,
    info: MessageInfo,
    recipient: String,
    quantity: u32,
) -> Result<Response, ContractError> {
    if quantity < 1u32 {
        return Err(ContractError::InvalidMintQuantity { quantity });
    }

    let relayer_associated_addr = RELAYER_ASSOCIATED_ADDR.load(deps.storage)?;
    let relayer_pointer_addr = RELAYER_POINTER_ADDR.load(deps.storage)?;
    // if relayer_associated_addr || relayer_pointer_addr {
    //     return Err(ContractError::RelayerNotConfigured {});
    // }

    // send funds to relayer
    let mut mint_fund_amount = 0u128;
    if info.funds.len() == 1 && info.funds[0].denom == SUPPORTED_DENOM {
        mint_fund_amount = info.funds[0].amount.into();
        BankMsg::Send {
            to_address: relayer_associated_addr.to_string(),
            amount: vec![coin(mint_fund_amount, SUPPORTED_DENOM)],
        };
    }

    let mint_attempt = MintAttempt::new(deps, recipient.to_string(), quantity, mint_fund_amount)?;
    wasm_execute(
        &relayer_pointer_addr,
        &Cw721ExecuteMsg::<(), ()>::Approve {
            spender: mint_attempt.minter.to_string(),
            token_id: mint_attempt.id.to_string(),
            expires: Some(cw721::Expiration::Never {}),
        },
        vec![],
    )?;
    Ok(Response::new())
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Relayer {} => Ok(to_json_binary(&query_relayer(deps)?)?),
        QueryMsg::GetMintAttempt { attempt_id } => Ok(to_json_binary(
            &MINT_ATTEMPTS.load(deps.storage, attempt_id)?,
        )?),
        QueryMsg::Ownership {} => Ok(to_json_binary(&get_ownership(deps.storage)?)?),
    }
}

pub fn query_relayer(deps: Deps) -> Result<RelayerResp, ContractError> {
    let relayer = RelayerResp {
        pointer_address: RELAYER_POINTER_ADDR.load(deps.storage)?.to_string(),
        associated_address: RELAYER_ASSOCIATED_ADDR.load(deps.storage)?.to_string(),
    };
    Ok(relayer)
}