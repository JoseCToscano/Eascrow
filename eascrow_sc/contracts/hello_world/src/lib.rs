#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, testutils::arbitrary::arbitrary::unstructured::Int, token, vec, Address, ConversionError, Env, Symbol, TryFromVal, Val, Vec};

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum DataKey {
    NativeToken = 0,
    Buyer = 1,
    Seller = 2,
    Price = 3,
    IsFunded = 4
}

impl TryFromVal<Env, DataKey> for Val {
    type Error = ConversionError;

    fn try_from_val(_env: &Env, v: &DataKey) -> Result<Self, Self::Error> {
        Ok((*v as u32).into())
    }
}

#[contract]
pub struct EascrowContract;

#[contractimpl]
impl EascrowContract {
    // Called by eascrow platform every time a customer wants to buy something
    pub fn initialize(env: Env, buyer: Address, seller: Address, token: Address, price: i128) {
        env.storage().instance().set(&DataKey::Buyer, &buyer);
        env.storage().instance().set(&DataKey::Seller, &seller);
        env.storage().instance().set(&DataKey::IsFunded, &false);
        env.storage().instance().set(&DataKey::NativeToken, &token);
        env.storage().instance().set(&DataKey::Price, &price);
    }

    // Called by customer when he add money to the contract
    pub fn fund(env: Env, buyer: Address, tokens_to_transfer: i128) {
        let is_funded: bool = env.storage().instance().get(&DataKey::IsFunded).unwrap_or(false);
        if is_funded {
            panic!("Contract is already funded");
        }

        // Customer add money to the contract
        buyer.require_auth();
        let contract = env.current_contract_address();
        let token_address: Address = env.storage().instance().get(&DataKey::NativeToken).unwrap();
        let native_token = token::Client::new(&env, &token_address);
        native_token.transfer(&buyer, &contract, &tokens_to_transfer);

        // If the contract have enough money, is_funded is set to true and it notifies the eascrow platform
        // Otherwise, just wait for the customer to add mode money
        let contract_balance = native_token.balance(&contract);
        let price: i128 = env.storage().instance().get(&DataKey::Price).unwrap();
        if contract_balance >= price {
            env.storage().instance().set(&DataKey::IsFunded, &true);
        }
    }

    // Called by Eascrow platform when the customer confirms it has received the service
    pub fn release_funds(env: Env, to: Address) {
        let is_funded: bool = env.storage().instance().get(&DataKey::IsFunded).unwrap_or(false);
        if !is_funded {
            panic!("Contract is not funded");
        }

        let seller: Address = env.storage().instance().get(&DataKey::Seller).unwrap();
        if to != seller {
            panic!("Funds can only be released to the seller");
        }

        // Logic to transfer the funds to the seller would go here
        let contract = env.current_contract_address();
        let token_address: Address = env.storage().instance().get(&DataKey::NativeToken).unwrap();
        let native_token = token::Client::new(&env, &token_address);
        let price: i128 = env.storage().instance().get(&DataKey::Price).unwrap();
        native_token.transfer(&contract, &seller, &price);
        
        // Transfer fund to seller account and wait for transaction complete aknowledgment
        // If transaction is failed, error management (procedure to be defined)
        // If transaction is successful, notifies Eascrow platform

        env.storage().instance().set(&DataKey::IsFunded, &false);
    }

    // Called by Eascrow platform when the customer raises dispute
    pub fn refund(env: Env, to: Address) {
        let is_funded: bool = env.storage().instance().get(&DataKey::IsFunded).unwrap_or(false);
        if !is_funded {
            panic!("Contract is not funded");
        }

        let seller: Address = env.storage().instance().get(&DataKey::Buyer).unwrap();
        if to != seller {
            panic!("Funds can only be released to the buyer");
        }

        // Logic to transfer the funds to the buyer would go here
        // Transfer fund to buyer account and wait for transaction complete aknowledgment
        // If transaction is failed, error management (procedure to be defined)
        // If transaction is successful, notifies Eascrow platform

        env.storage().instance().set(&DataKey::IsFunded, &false);
    }
}

mod test;
