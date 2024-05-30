use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: String,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    SetRelayer {
        // MintRelayer's CW pointer contract address
        pointer_address: String,
        // MintRelayer's associated Sei address
        associated_address: String,
    },
    Mint {
        // The NFT minter
        recipient: String,
        // Number of tokens to mint
        quantity: u32,
    },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(RelayerResp)]
    Relayer {},
    #[returns(crate::state::MintAttempt)]
    GetMintAttempt { attempt_id: u32 },
}

#[cw_serde]
pub struct RelayerResp {
    pub associated_address: Option<Addr>,
    pub pointer_address: Option<Addr>,
}

#[cw_serde]
pub struct AdminResp {
    pub admin: Addr,
}
