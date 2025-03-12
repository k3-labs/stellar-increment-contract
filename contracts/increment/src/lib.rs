#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");
const COUNTER_64: Symbol = symbol_short!("COUNTER64");
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
        env.events().publish((symbol_short!("increment"), 1u32), count);

        // Return the count to the caller.
        count
    }

    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);

        count -= 1;

        env.storage().persistent().set(&COUNTER, &count);
        env.events().publish((symbol_short!("decrement"), 1u32), count);

        count
    }

    pub fn increment_by(env: Env, num: u32) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        count += num;

        env.storage().persistent().set(&COUNTER, &count);
        env.events().publish((symbol_short!("increment"), num), count);

        count
    }

    pub fn decrement_by(env: Env, num: u32) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        count -= num;

        env.storage().persistent().set(&COUNTER, &count);
        env.events().publish((symbol_short!("decrement"), num), count);

        count
    }

    pub fn set_value(env: Env, num: u32) -> u32 {
        let mut count: u32 = env.storage().persistent().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        count = num;

        env.storage().persistent().set(&COUNTER, &count);
        env.events().publish((symbol_short!("set_value"), num), count);

        count
    }

    pub fn get_current_value(env: Env) -> u32 {
        env.storage().persistent().get(&COUNTER).unwrap_or(0)
    }

    // u64 test
    pub fn increment_u64(env: Env) -> u64 {
        // Get the current count.
        let mut count: u64 = env.storage().persistent().get(&COUNTER_64).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().persistent().set(&COUNTER_64, &count);
        env.events().publish((symbol_short!("inc_u64"), 1u64), count);

        // Return the count to the caller.
        count
    }

    pub fn decrement_u64(env: Env) -> u64 {
        let mut count: u64 = env.storage().persistent().get(&COUNTER_64).unwrap_or(0);
        log!(&env, "count: {}", count);

        count -= 1;

        env.storage().persistent().set(&COUNTER_64, &count);
        env.events().publish((symbol_short!("dec_u64"), 1u64), count);

        count
    }

    pub fn increment_by_u64(env: Env, num: u64) -> u64 {
        let mut count: u64 = env.storage().persistent().get(&COUNTER_64).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

        count += num;

        env.storage().persistent().set(&COUNTER, &count);

        count
    }

    pub fn get_current_value_u64(env: Env) -> u64 {
        env.storage().persistent().get(&COUNTER_64).unwrap_or(0)
    }
}

mod test;
