use bidding_token::state::InstantiateMsg;
use cosmwasm_schema::write_api;
use cw20_base::msg::{ExecuteMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg
    }
}
