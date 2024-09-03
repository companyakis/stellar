// Task:
// Import the following types from soroban sdk: contract, contractimpl, symbol_short, vec, Env, Symbol, Vec.

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};





// All contracts should begin with #![no_std] to ensure that the Rust standard library is not included in the build. 
// The Rust standard library is large and not well suited to being deployed into small programs like those deployed to blockchains.
// The contract imports the types and macros that it needs from the soroban-sdk crate.
// Many of the types available in typical Rust programs, such as std::vec::Vec, are not available, as there is no allocator and no heap memory in Soroban contracts. 
// The soroban-sdk provides a variety of types like Vec, Map, Bytes, BytesN, Symbol, that all utilize the Soroban environment's memory and native capabilities. 
// Primitive values like u128, i128, u64, i64, u32, i32, and bool can also be used. Floats and floating point math are not supported.
