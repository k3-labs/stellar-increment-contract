#![cfg(test)]

use super::{IncrementContract, IncrementContractClient};
use soroban_sdk::{
    testutils::{Events, Logs},
    Env,
};

extern crate std;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, IncrementContract);
    let client = IncrementContractClient::new(&env, &contract_id);
    assert_eq!(client.increment_u64(), 1);
    assert_eq!(client.increment_by_u64(&33), 34);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);

    assert_eq!(client.decrement(), 2);
    assert_eq!(client.increment_by(&10), 12);
    assert_eq!(client.increment_by(&10), 22);
    assert_eq!(client.decrement_by(&2), 20);

    assert_eq!(client.get_current_value(), 20);
    assert_eq!(client.set_value(&69), 69);
    assert_eq!(client.get_current_value(), 69);

    assert_eq!(client.get_current_value_u64(), 34);
    let events = client.env.events().all();
    for e in events {
        std::dbg!(e);
    }
    std::println!("{}", env.logs().all().join("\n"));
}
