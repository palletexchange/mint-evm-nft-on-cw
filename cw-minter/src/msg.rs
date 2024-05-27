use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InstantiateMsg {
    pub admin: String,
}

#[derive(Serialize, Deserialize)]
pub struct RelayerResp {
    pub associated_address: String,
    pub pointer_address: String,
}


#[derive(Serialize, Deserialize)]
pub struct AdminResp  {
    pub admin: Addr,
}

#[derive(Serialize, Deserialize)]
pub enum ExecuteMsg {
    UpdateAdmin { admin: String },
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

#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
    Admin {},
    Relayer {},
    GetMintAttempt {
        attempt_id: u32,
    },
}
