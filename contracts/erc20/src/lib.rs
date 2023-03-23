#![allow(unused)]
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

// use crate::contract::*;

// use lazy_static::lazy_static;

wit_bindgen::generate!("erc20");

// lazy_static! {
//     pub static ref STATE: Arc<Mutex<State>> = Arc::new(Mutex::new(State::default()));
// }

pub struct Erc20;

// #[derive(Debug, Default)]
// pub struct State {
//     name: Option<String>,
//     symbol: Option<String>,
//     balances: HashMap<String, u64>,
//     total_supply: u64,
// }

// impl State {
//     pub fn new(state: State, name: String, symbol: String) -> State {
//         State {
//             name: Some(name),
//             symbol: Some(symbol),
//             balances: HashMap::new(),
//             total_supply: 0,
//         }
//     }
// }

export_contract!(Erc20);

impl exports::Exports for Erc20 {
    fn construct(state: exports::State, name: String, symbol: String) {}

    fn mint(account: String, amount: u64) {}

    fn transfer(to: String, amount: u64) {
        println!("to {}, amount", amount);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_constructs() {
//         let erc20 = Erc20::construct("aa".into(), "bb".into());
//         let _ = Erc20::mint("0x123".into(), 10);
//         let _ = Erc20::mint("0x123".into(), 10);
//     }
// }
