use crate::error::ContractError;
use crate::msg::{AdminResp, ExecuteMsg, InstantiateMsg, QueryMsg, RelayerResp};
use crate::state::{MintAttempt, MINT_ATTEMPTS, RELAYER_ASSOCIATED_ADDR, RELAYER_POINTER_ADDR};
use crate::SUPPORTED_DENOM;
use cosmwasm_std::{
    coin, entry_point, to_json_binary, wasm_execute, BankMsg, Binary, CosmosMsg, Deps, DepsMut,
    Env, MessageInfo, Response,
};
use cw721_base::ExecuteMsg as Cw721ExecuteMsg;
use cw_ownable::{assert_owner, get_ownership, initialize_owner};

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
    info: MessageInfo,
    pointer_address: String,
    associated_address: String,
) -> Result<Response, ContractError> {
    assert_owner(deps.storage, &info.sender)?;

    let pointer_addr = deps.api.addr_validate(&pointer_address)?;
    let associated_addr = deps.api.addr_validate(&associated_address)?;

    RELAYER_POINTER_ADDR.save(deps.storage, &pointer_addr)?;
    RELAYER_ASSOCIATED_ADDR.save(deps.storage, &associated_addr)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "set_relayer"),
        ("pointer_address", &pointer_address),
        ("associated_address", &associated_address),
    ]))
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

    let relayer_associated_addr = RELAYER_ASSOCIATED_ADDR
        .load(deps.storage)
        .map_err(|_| ContractError::RelayerNotConfigured {})?;
    let relayer_pointer_addr = RELAYER_POINTER_ADDR
        .load(deps.storage)
        .map_err(|_| ContractError::RelayerNotConfigured {})?;

    // send funds to relayer
    let mint_fund_amount = if info.funds.len() == 1 && info.funds[0].denom == SUPPORTED_DENOM {
        info.funds[0].amount.u128()
    } else {
        return Err(ContractError::InvalidFundsReceived {});
    };
    let send_msg: CosmosMsg = BankMsg::Send {
        to_address: relayer_associated_addr.to_string(),
        amount: vec![coin(mint_fund_amount, SUPPORTED_DENOM)],
    }
    .into();

    let mint_attempt = MintAttempt::new(deps, recipient.to_string(), quantity, mint_fund_amount)?;
    let mint_approval_msg = wasm_execute(
        &relayer_pointer_addr,
        &Cw721ExecuteMsg::<(), ()>::Approve {
            spender: mint_attempt.minter.to_string(),
            token_id: mint_attempt.id.to_string(),
            expires: Some(cw721::Expiration::Never {}),
        },
        vec![],
    )?
    .into();

    Ok(Response::new()
        .add_messages(vec![send_msg, mint_approval_msg])
        .add_attributes(vec![
            ("action", "mint"),
            ("recipient", &recipient),
            ("quantity", &quantity.to_string()),
            ("funds", &mint_fund_amount.to_string()),
        ]))
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
        pointer_address: RELAYER_POINTER_ADDR.load(deps.storage).ok(),
        associated_address: RELAYER_ASSOCIATED_ADDR.load(deps.storage).ok(),
    };
    Ok(relayer)
}
