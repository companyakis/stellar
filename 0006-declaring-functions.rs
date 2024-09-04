// Define a private function with name private_function and print the message “This is private function”


#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, log};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    pub fn say_hi(env: Env, name: String) {
        log!(&env, "Hello {}!", name);
    }

    fn private_function(env: Env) {
        log!(&env, "This is private function");
    }

}
