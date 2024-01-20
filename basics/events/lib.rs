#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    /// Defines an event that is emitted
    /// every time value is transferred.
    
    #[ink(event)]
    pub struct Transfer {
        from: Option<AccountId>,
        to: Option<AccountId>,
        value: Balance,
    }
    
    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
    }

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(initial_supply: Balance) -> Self {
            let caller = Self::env().caller();
            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: initial_supply,
            });
            Self {
                total_supply: initial_supply,
            }
        }
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

    }
    
   
}
