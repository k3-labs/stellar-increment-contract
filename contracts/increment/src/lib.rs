#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract; 

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        // Get the current count.
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().persistent().set(&COUNTER, &count);

        // Return the count to the caller.
        count
    }

    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);

        count -= 1;

        env.storage().persistent().set(&COUNTER, &count);

        count
    }

    pub fn increment_by(env: Env, num: u32) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        count += num;

        env.storage().persistent().set(&COUNTER, &count);

        count
    }

    pub fn decrement_by(env: Env, num: u32) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        count -= num;

        env.storage().persistent().set(&COUNTER, &count);

        count
    }

    pub fn set_value(env: Env, num: u32) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        count = num;

        env.storage().persistent().set(&COUNTER, &count);

        count
    }

    pub fn get_current_value(env: Env) -> u32 {
        env.storage().persistent().get(&COUNTER).unwrap_or(0)
    }
}

mod test;
