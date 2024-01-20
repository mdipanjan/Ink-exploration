#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod example {
    //The ink_prelude crate provides an efficient approach to import commonly used Rust types such as String and Vec, 
    //ensuring safe usage within an ink! contract.


    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    // Can also store custom struct
    pub struct Auction {
        /// Branded name of the auction event.
        name: String,
        /// Some hash identifying the auction subject.
        subject: Hash,
        /// Auction status.
        status: Status, // Enum: Usage shown in next section
        /// Candle auction can have no winner.
        /// If auction is finalized, that means that the winner is determined.
        finalized: bool,
        /// vector
        vector: Vec<u8>,
        
        // Store Auctions in a vec
        auctions: Vec<Auction>,

    }


    #[ink(storage)]
    pub struct Example {
        /// Stores a single `bool` value on the storage.
        value: bool,
        // Store some String
        my_string: String,
        // Store some u32 in a vec
        my_vector: Vec<u32>,
        //Here is an example of how you would store substrate types AccountId, Balance and Hash:

        // Store some AccountId
        my_account: AccountId,
        // Store some Balance
        my_balance: Balance,
        // Store some Hash
        my_hash: Hash,
        
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
    }

  
}
