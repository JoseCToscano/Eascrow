#![no_std]
use soroban_sdk::{contract, contractimpl, token, Address, ConversionError, Env, TryFromVal, Val};

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

fn get_contract_address(env: &Env) -> Address {
    env.current_contract_address()
}

fn get_token_address(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::NativeToken).unwrap()
}

fn get_buyer_address(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Buyer).unwrap()
}

fn get_seller_address(env: &Env) -> Address {
    env.storage().instance().get(&DataKey::Seller).unwrap()
}

fn get_is_funded(env: &Env) -> bool {
    env.storage().instance().get(&DataKey::IsFunded).unwrap_or(false)
}

fn get_price(env: &Env) -> i128 {
    env.storage().instance().get(&DataKey::Price).unwrap()
}

fn get_contract_balance(env: &Env) -> i128 {
    let native_token = token::Client::new(&env, &get_token_address(env));
    native_token.balance(&get_contract_address(env))
}

fn get_token_client(env: &Env) -> token::TokenClient<'_> {
    token::Client::new(&env, &get_token_address(env))
}

fn set_buyer_address(env: &Env, buyer: Address) {
    env.storage().instance().set(&DataKey::Buyer, &buyer);
}

fn set_seller_address(env: &Env, seller: Address) {
    env.storage().instance().set(&DataKey::Seller, &seller);
}

fn set_is_funded(env: &Env, is_funded: bool) {
    env.storage().instance().set(&DataKey::IsFunded, &is_funded);
}

fn set_native_token(env: &Env, native_token: Address) {
    env.storage().instance().set(&DataKey::NativeToken, &native_token);
}

fn set_price(env: &Env, price: i128) {
    env.storage().instance().set(&DataKey::Price, &price);
}

fn transfer(env: &Env, from: &Address, to: &Address, amount: &i128) {
    let native_token: token::TokenClient<'_> = get_token_client(&env);
    native_token.transfer(&from, &to, &amount);
}

#[contract]
pub struct EascrowContract;

#[contractimpl]
impl EascrowContract {
    // Called by eascrow platform every time a customer wants to buy something
    pub fn initialize(env: Env, buyer: Address, seller: Address, token: Address, price: i128) {
        set_buyer_address(&env, buyer);
        set_seller_address(&env, seller);
        set_is_funded(&env, false);
        set_native_token(&env, token);
        set_price(&env, price);
    }

    // Called by customer when he add money to the contract
    pub fn fund(env: Env, buyer: Address, tokens_to_transfer: i128) {
        let is_funded: bool = get_is_funded(&env);
        if is_funded {
            panic!("Contract is already funded");
        }

        buyer.require_auth();
        let contract = get_contract_address(&env);
        transfer(&env, &buyer, &contract, &tokens_to_transfer);

        let contract_balance = get_contract_balance(&env);
        let price: i128 = get_price(&env);
        if contract_balance >= price {
            set_is_funded(&env, true);
        }
    }

    // Called by Eascrow platform when the customer confirms it has received the service
    pub fn release_funds(env: Env) {
        let is_funded: bool = get_is_funded(&env);
        if !is_funded {
            panic!("Contract is not funded");
        }

        let seller: Address = get_seller_address(&env);
        let contract = get_contract_address(&env);
        let price: i128 = get_price(&env);
        transfer(&env, &contract, &seller, &price);

        set_is_funded(&env, false);
    }

    // Called by Eascrow platform when the customer raises dispute
    pub fn refund(env: Env) {
        let is_funded: bool = get_is_funded(&env);
        if !is_funded {
            panic!("Contract is not funded");
        }

        let buyer: Address = get_buyer_address(&env);
        let contract = get_contract_address(&env);
        let price: i128 = get_price(&env);
        transfer(&env, &contract, &buyer, &price);

        set_is_funded(&env, false);
    }
}

mod test;
