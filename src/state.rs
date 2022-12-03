use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use cw_utils::Expiration;

#[cw_serde]
pub struct Config {
    pub arbiter: Addr,           // Only who can ExecuteMsg::Approve{ quantity: Option<Vec<Coin>> }, which will send Coin to recipient
    pub recipient: Addr,         // Recipient/Beneficiary: only one that can receive funds when When arbitrer approves. 
    pub source: Addr,            // Creates contract. Receives all contract funds after ExecuteMsg::Refund{}
    pub expiration: Option<Expiration>,  
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);
