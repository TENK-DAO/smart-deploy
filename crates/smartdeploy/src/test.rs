#![cfg(test)]

use crate::{SorobanContract, SorobanContractClient};
use loam_sdk::soroban_sdk::{testutils::Address as _, Address, Env, String};
extern crate std;

// The contract that will be deployed by the Publisher contract.
// mod contract {
//     use loam_sdk::soroban_sdk;
//     soroban_sdk::contractimport!(
//         file = "../../target/wasm32-unknown-unknown/release-with-logs/smartdeploy.wasm"
//     );
// }

// fn init() -> (Env, SorobanContractClient, Address) {
//     let env = Env::default();
//     let client = SorobanContractClient::new(&env, &env.register_contract(None, SorobanContract));
//     let address = Address::random(&env);
//     (env, client, address)
// }

pub fn name(env: &Env) -> String {
    String::from_slice(env, "publisher")
}

#[test]
fn handle_error_cases() {
    // let (env, client, address) = &init();

    // let name = &name(env);
    // let res = client.try_fetch(name, &None).unwrap_err();

    // println!("{res:#?}");
    // assert!(matches!(res, Ok(Error::NoSuchContract)));
    // let wasm_hash = env.install_contract_wasm(contract::WASM);

    // let res = client.try_fetch(name, &None).unwrap_err();
    // assert!(matches!(res, Ok(Error::NoSuchVersion)));

    // client.publish(name, address, &wasm_hash, &None, &None);
    // let res = client.try_fetch(name, &None).unwrap().unwrap();
    // assert_eq!(res, wasm_hash);

    // let other_address = Address::random(env);
    // let res = client.try_register_name(&other_address, name).unwrap_err();
    // assert!(matches!(res, Ok(Error::AlreadyRegistered)));

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);

    // let res = client.try_deploy(name, &None, &String::from_slice(env, "hello"), &None);
    // std::println!("{res:?}");
}

// #[test]
// fn returns_most_recent_version() {
//     let (env, client, address) = &init();
//     let name = &name(env);
//     // client.register_name(address, name);
//     let wasm_hash = env.install_contract_wasm(contract::WASM);

//     client.publish(name, &wasm_hash, &None, &None);
//     let fetched_hash = client.fetch(name, &None);
//     assert_eq!(fetched_hash, wasm_hash);
//     let second_hash: BytesN<32> = BytesN::random(env);
//     client.publish(name, &second_hash, &None, &None);
//     let res = client.fetch(name, &None);
//     assert_eq!(res, second_hash);

//     let third_hash: BytesN<32> = BytesN::random(env);
//     client.publish(name, &third_hash, &None, &None);
//     let res = client.fetch(name, &None);
//     assert_eq!(res, third_hash);

//     let third_hash: BytesN<32> = BytesN::random(env);
//     client.publish(name, &third_hash, &None, &None);
//     let res = client.fetch(name, &None);
//     assert_eq!(res, third_hash);
// }
