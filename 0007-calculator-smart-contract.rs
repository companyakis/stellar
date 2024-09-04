#![no_std]
use soroban_sdk::{contract, contractimpl, Env, String, log};

#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    pub fn calculator(env: Env, a: i32, b: i32, c: i32) -> (i32, i32) {

        let addition_result = Self::addition(a, b, c);

        let multiplication_result = Self::multiplication(a, b, c);

        (addition_result, multiplication_result)
    }

    fn addition(a: i32, b: i32, c: i32) -> i32 {

        a + b + c
    }

    fn multiplication(a: i32, b: i32, c: i32) -> i32 {

        a * b * c
    }

}
