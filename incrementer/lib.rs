#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
//use ink_storage::collections::{Vec, HashMap, Stash, Bitvec};


#[ink::contract]
mod incrementer {
    #[ink(storage)]
    pub struct Incrementer {
        // Store HashMap of AccountID and i32
        my_value: ink_storage::collections::HashMap<AccountId, i32>,
        value: i32,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            // Contract Constructor
            Self{
                value: init_value,
                my_value: ink_storage::collections::HashMap::new(),
            }
        }


        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                value: 0,
                my_value: Default::default(),
            }
        }

        #[ink(message)]
        pub fn get(&self) -> i32 {
            self.value
        }

        #[ink(message)]
        pub fn inc(&mut self, by: i32) {
            self.value += by;
        }

        // Get the value for the calling AccountId
        #[ink(message)]
        pub fn get_mine(&self) -> i32 {
            let caller = self.env().caller();
            self.my_number_or_zero(&caller)
        }


        /// Returns the number for an AccountId or 0 if it is not set.
        fn my_number_or_zero(&self, of: &AccountId) -> i32 {
            let balance = self.my_value.get(of).unwrap_or(&0);
            *balance
        }
        
        
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);        
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(5);
            assert_eq!(contract.get(), 47);
            contract.inc(-50);
            assert_eq!(contract.get(), -3);
        }

        #[ink::test]
        fn my_value_works() {
            let contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
        }


    }
}
