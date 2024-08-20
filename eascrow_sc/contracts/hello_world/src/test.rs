#![cfg(test)]

use super::*;
use soroban_sdk::{log, testutils::{Address, Logs}, Env};
extern crate std;

#[test]
fn test() {
    
    
    let env = Env::default();
    let contract_id = env.register_contract(None, EascrowContract);
    let client = EascrowContractClient::new(&env, &contract_id);
    
    let buyer = <soroban_sdk::Address as Address>::generate(&env);
    let seller = <soroban_sdk::Address as Address>::generate(&env);
    
    std::println!("{:#?}", buyer);
    std::println!("{:#?}", seller);

    client.initialize(&buyer, &seller);
    client.fund(&buyer);
    client.release_funds( &seller);

    //env.logger().print("{?#}", address1);
    //let words = client.hello(&symbol_short!("Dev"));
    //assert_eq!(
    //    words,
    //    vec![&env, symbol_short!("Hello"), symbol_short!("Dev"),]
    //);
}
