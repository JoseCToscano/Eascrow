#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Address, Vec};

#[contract]
pub struct EascrowContract;

#[contractimpl]
impl EascrowContract {
    // Called by eascrow platform every time a customer wants to buy something
    pub fn initialize(env: Env, buyer: Address, seller: Address) {
        env.storage().instance().set(&"buyer", &buyer);
        env.storage().instance().set(&"seller", &seller);
        env.storage().instance().set(&"is_funded", &false);
    }

    // Called by customer when he add money to the contract
    pub fn fund(env: Env, buyer: Address) {
        let is_funded: bool = env.storage().instance().get(&"is_funded").unwrap_or(false);
        if is_funded {
            panic!("Contract is already funded");
        }

        // Customer add money to the contract
        // If the contract have enough money, is_funded is set to true and it notifies the eascrow platform
        // Otherwise, just wait for the customer to add mode money

        env.storage().instance().set(&"is_funded", &true);
    }

    // Called by Eascrow platform when the customer confirms it has received the service
    pub fn release_funds(env: Env, to: Address) {
        let is_funded: bool = env.storage().instance().get(&"is_funded").unwrap_or(false);
        if !is_funded {
            panic!("Contract is not funded");
        }

        let seller: Address = env.storage().instance().get(&"seller").unwrap();
        if to != seller {
            panic!("Funds can only be released to the seller");
        }

        // Logic to transfer the funds to the seller would go here
        // Transfer fund to seller account and wait for transaction complete aknowledgment
        // If transaction is failed, error management (procedure to be defined)
        // If transaction is successful, notifies Eascrow platform

        env.storage().instance().set(&"is_funded", &false);
    }

    // Called by Eascrow platform when the customer raises dispute
    pub fn refund(env: Env, to: Address) {
        let is_funded: bool = env.storage().instance().get(&"is_funded").unwrap_or(false);
        if !is_funded {
            panic!("Contract is not funded");
        }

        let seller: Address = env.storage().instance().get(&"buyer").unwrap();
        if to != seller {
            panic!("Funds can only be released to the buyer");
        }

        // Logic to transfer the funds to the buyer would go here
        // Transfer fund to buyer account and wait for transaction complete aknowledgment
        // If transaction is failed, error management (procedure to be defined)
        // If transaction is successful, notifies Eascrow platform

        env.storage().instance().set(&"is_funded", &false);
    }
}

mod test;
