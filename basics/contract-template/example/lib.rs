// If the `std` feature from the `Cargo.toml` is not enabled
// we switch on `no_std`, this has the effect of Rusts standard
// library not being included in our contract.
//
// The Rust standard library is OS-dependent and Wasm is
// architecture independent.
#![cfg_attr(not(feature = "std"), no_std, no_main)]

// This (#[ink::contract]) is the ink! macro, the starting point for our contract.
// Everything below it might look like Rust code, but it is actually
// run through a parser in ink!.
#[ink::contract]
mod example {

    /// Defines the storage of our contract.
    /// to add new static storage fields to our contract.
    #[ink(storage)]
    pub struct Example {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }

    impl Example {
        /// A constructor that the contract can be initialized with.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// An alternative constructor that the contract can be
        /// initialized with        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A state-mutating function that the contract exposes to the
        /// outside world.
        ///
        /// By default functions are private, they have to be annotated
        /// with `#[ink(message)]` and `pub` to be available from the
        /// outside.

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        /// A public contract function that has no side-effects.
        ///
        /// Note that while purely reading functions can be invoked
        /// by submitting a transaction on-chain, this is usually
        /// not done as they have no side-effects and the transaction
        /// costs would be wasted.
        /// Instead those functions are typically invoked via RPC to
        /// return a contract's state.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// This attribute denotes that the test is executed in
        /// a simulated, mocked blockchain environment. There are
        /// functions available to influence how the test environment
        /// is configured (e.g. setting an account to a specified balance).        
        
        #[ink::test]
        fn default_works() {
            let example = Example::default();
            assert_eq!(example.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut example = Example::new(false);
            assert_eq!(example.get(), false);
            example.flip();
            assert_eq!(example.get(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
        
        /// With this attribute the contract will be compiled and deployed
        /// to a Substrate node that is required to be running in the
        /// background.
        ///
        /// We offer API functions that enable developers to then interact
        /// with the contract. ink! will take care of putting contract calls
        /// into transactions that will be submitted to the Substrate chain.
        ///
        /// Developers can define assertions on the outcome of their transactions,
        /// such as checking for state mutations, transaction failures or
        /// incurred gas costs.
       
        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ExampleRef::default();

            // When
            let contract_account_id = client
                .instantiate("example", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<ExampleRef>(contract_account_id.clone())
                .call(|example| example.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = ExampleRef::new(false);
            let contract_account_id = client
                .instantiate("example", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<ExampleRef>(contract_account_id.clone())
                .call(|example| example.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<ExampleRef>(contract_account_id.clone())
                .call(|example| example.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<ExampleRef>(contract_account_id.clone())
                .call(|example| example.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
