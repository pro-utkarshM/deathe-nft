#![cfg(test)]

use cosmwasm_std::{coins, Addr, Empty, Uint128};
use cw20::MinterResponse;
use cw721_base::msg::ExecuteMsg as Cw721ExecuteMsg;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};


pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw721_base::entry::execute,
        cw721_base::entry::instantiate,
        cw721_base::entry::query,
    );
    Box::new(contract)
}

pub fn contract_cw20() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

pub fn FractionalNftContract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        crate::contract::entry_points::execute,
        crate::contract::entry_points::instantiate,
        crate::contract::entry_points::query,
    );
    Box::new(contract)
}

// Initial contract setup
fn setup_contracts() -> App {
    let admin = Addr::unchecked(ADMIN);

    let init_funds = coins(2000, "ustars");

    let mut router = App::new(|router, _, storage| {
        router
            .bank
            .init_balance(storage, &admin, init_funds)
            .unwrap();
    });

    // Set up CW721 contract
    let cw721_id = router.store_code(contract_cw721());
    let msg = cw721_base::msg::InstantiateMsg {
        name: String::from("Bad Kids"),
        symbol: String::from("BAD"),
        minter: admin.to_string(),
    };

    let cw721_addr = router
        .instantiate_contract(cw721_id, admin.clone(), &msg, &[], "BAD_CW721", None)
        .unwrap();

    // Set up CW20 contract
    let cw20_id = router.store_code(contract_cw20());
    let msg = cw20_base::msg::InstantiateMsg {
        name: String::from("Fractionalized Bad Kids"),
        symbol: String::from("BAD"),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter: FRAC721.to_string(),
            cap: None,
        }),
        marketing: None,
    };
    let cw20_addr = router
        .instantiate_contract(cw20_id, admin.clone(), &msg, &[], "BAD_CW20", None)
        .unwrap();

    // Set up Frac721 contract
    let frac721_id = router.store_code(FractionalNftContract());
    let msg = crate::contract::InstantiateMsg {
        collection_address: cw721_addr.to_string(),
        cw20_config: None,
    };

    let frac721_addr = router
        .instantiate_contract(frac721_id, admin.clone(), &msg, &[], "BAD_FRAC721", None)
        .unwrap();

    // Execute set_cw20_address on Frac721
    let msg = crate::contract::ExecMsg::SetTokenAddress {
        cw20_address: cw20_addr.to_string(),
    };

    router
        .execute_contract(admin.clone(), frac721_addr, &msg, &[])
        .unwrap();

    router
}

// Mint a CW721 NFT to an address
fn mint_cw721(router: &mut App, addr: Addr, token_id: &str) {
    let msg: Cw721ExecuteMsg<Empty, Empty> = Cw721ExecuteMsg::Mint {
        token_id: token_id.to_string(),
        owner: addr.to_string(),
        token_uri: None,
        extension: Empty {},
    };

    router
        .execute_contract(Addr::unchecked(ADMIN), Addr::unchecked(CW721), &msg, &[])
        .unwrap();
}

#[test]
fn proper_initialization() {
    setup_contracts();
}