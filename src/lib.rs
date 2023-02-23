#![no_std]

use core::panic;
use soroban_sdk::{contractimpl, Address, Env, Symbol, symbol};

pub struct TinyNFT;

const OWNER_KEY: Symbol = symbol!("OWNER");

#[contractimpl]
impl TinyNFT {
    pub fn init(env: Env, owner: Address) {
        if env.storage().has(&OWNER_KEY) {
            panic!()
        } 
        env.storage().set(&OWNER_KEY, &owner);
    }
    pub fn get_owner(env: Env) -> Address {
        env.storage().get(&OWNER_KEY).unwrap().unwrap()
    }
    pub fn xfer(env: Env, to: Address) {        
        Self::get_owner(env.clone()).require_auth();
        env.storage().set(&OWNER_KEY, &to);
    }
}

mod test;
