// Assignment: Defining contract

#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {

}




// Contract functions are defined within an impl block for the struct, which is annotated with #[contractimpl]. 
// It is important to note that contract functions should have names with a maximum length of 32 characters. 
// Additionally, if a function is intended to be invoked from outside the contract, it should be marked with the pub visibility modifier. 
// It is common for the first argument of a contract function to be of type Env, allowing access to a copy of the Soroban environment, which is typically necessary for various operations within the contract.
