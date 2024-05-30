use crate::SUPPORTED_DENOM;
use cosmwasm_std::{Addr, StdError};
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Std: {0}")]
    Std(#[from] StdError),

    #[error("Ownership: {0}")]
    Ownership(#[from] OwnershipError),

    #[error("Unauthorized: {sender} is not authorized")]
    Unauthorized { sender: Addr },

    #[error("Must send funds in {SUPPORTED_DENOM}.")]
    InvalidFundsReceived {},

    #[error("Invalid mint quantity: {quantity}.")]
    InvalidMintQuantity { quantity: u32 },

    #[error("Must first configure relayer with `set_relayer`.")]
    RelayerNotConfigured {},
}
