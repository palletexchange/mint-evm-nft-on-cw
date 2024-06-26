use crate::error::ContractError;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, StdResult};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct MintAttempt {
    pub id: u32,
    pub minter: Addr,
    pub quantity: u32,
    pub funds: u128,
}

impl MintAttempt {
    pub fn new(
        deps: DepsMut,
        minter: &Addr,
        quantity: u32,
        funds: u128,
    ) -> Result<Self, ContractError> {
        let mint_attempt_id =
            NUM_MINTS_ATTEMPTED.update(deps.storage, |n| -> StdResult<_> { Ok(n + 1) })?;
        let mint_attempt = Self {
            id: mint_attempt_id,
            minter: minter.clone(),
            quantity,
            funds,
        };
        MINT_ATTEMPTS.save(deps.storage, mint_attempt_id, &mint_attempt)?;
        Ok(mint_attempt)
    }
}

pub const RELAYER_ASSOCIATED_ADDR: Item<Addr> = Item::new("relayer_associated_addr");

pub const RELAYER_POINTER_ADDR: Item<Addr> = Item::new("relayer_pointer_addr");

pub const NUM_MINTS_ATTEMPTED: Item<u32> = Item::new("num_mints_attempted");

pub const MINT_ATTEMPTS: Map<u32, MintAttempt> = Map::new("mint_attempts");
