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
        // number: u32,
        value: i32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // Contract Constructor
            Self{
                value: init_value,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
            }
        }


        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
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
