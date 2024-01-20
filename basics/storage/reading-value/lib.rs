#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod example {

    #[ink(storage)]
    pub struct Example {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Example {
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }
        // In !ink we can use the #[ink(message)] attribute to mark a function as callable from outside the contract.
        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }

        /// Private function not exposed as a message.
        fn my_private_function(&self) {
            /* --snip-- */
        }
    }

  
}
