#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_storage::collections::{Vec, HashMap, Stash, Bitvec};


#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        // Store some AccountId
        // my_account: AccountId,
        // Store some Balance
        // my_balance: Balance,
        number: u32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            // Contract Constructor
            Self{
                number: init_value,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                number: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) {
            // Contract Message
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            // Test Your Contract
        }
    }
}
