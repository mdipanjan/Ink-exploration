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

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
        //  It is through self that you gain access to all your contract functions and storage items. to only read value 
        //  we pass &self instead of &mut self
        #[ink(message)]
        pub fn my_getter(&self) -> bool {
            self.value
        }
        // But if you want to modify storage items, you will need to explicitly mark it as mutable, &mut self.

        #[ink(message)]
        pub fn my_setter(&mut self, new_value: bool) {
            self.value = new_value;
        }
    }
}
