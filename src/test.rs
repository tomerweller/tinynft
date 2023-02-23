#![cfg(test)]
extern crate std;
use super::*;

use soroban_sdk::{testutils::{Address as _,}, Address, Env};

fn get_client(env: &Env) -> TinyNFTClient {
    let contract_id = env.register_contract(None, TinyNFT);
    TinyNFTClient::new(env, &contract_id)    
}

#[test]
fn happy_path() {
    let env = Env::default();
    let client = get_client(&env);
    let user_1 = Address::random(&env);
    let user_2 = Address::random(&env);
    
    // test init
    client.init(&user_1);
    assert_eq!(client.get_owner(), user_1);

    // test transfer
    client.xfer(&user_2);
    // verify that authorization wes provided by the owner (user_1)
    assert_eq!(env.recorded_top_authorizations()[0].0, user_1);
    assert_eq!(client.get_owner(), user_2);
}

#[test]
#[should_panic]
fn no_init(){
    let env = Env::default();
    let client = get_client(&env);
    let user_1 = Address::random(&env);
    client.xfer(&user_1);
}

#[test]
#[should_panic]
fn double_init(){
    let env = Env::default();
    let client = get_client(&env);
    let user_1 = Address::random(&env);
    client.init(&user_1);
    client.init(&user_1);
}

