#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod owner {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ink_storage::{
        lazy::Lazy,
    };

    /// Contract module which provides a basic access control mechanism, where
    /// there is an account (an owner) that can be granted exclusive access to
    /// specific functions
    #[ink(storage)]
    pub struct Owner {
        _owner: Lazy<Option<AccountId>>,
    }

    /// Event emitted when Owner AccountId Transferred
    #[ink(event)]
    pub struct OwnershipTransferred {
        /// previous owner account id
        #[ink(topic)]
        previous_owner: Option<AccountId>,
        /// new owner account id
        #[ink(topic)]
        new_owner: Option<AccountId>,
    }

    impl Owner {
        /// Initializes the contract setting the owner as the initial owner.
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Some(Self::env().caller());

            Self::env().emit_event(OwnershipTransferred {
                previous_owner: None,
                new_owner: caller,
            });

            Self { _owner: Lazy::new(caller) }
        }
    }

    impl Owner {
        /// Leaves the contract without owner. It will not be possible to call
        /// `ensure_owner` functions anymore. Can only be called by the current owner.
        /// NOTE: Renouncing ownership will leave the contract without an owner,
        /// thereby removing any functionality that is only available to the owner.
        #[ink(message)]
        pub fn renounce_ownership(&mut self) {
            self.ensure_caller_is_owner();

            Self::env().emit_event(OwnershipTransferred {
                previous_owner: *self._owner,
                new_owner: None,
            });

            Lazy::set(&mut self._owner, None);
        }

        /// Transfers ownership of the contract to a new account (`newOwner`).
        /// Can only be called by the current owner.
        #[ink(message)]
        pub fn transfer_ownership(&mut self, new_owner: AccountId) {
            self.ensure_caller_is_owner();

            Self::env().emit_event(OwnershipTransferred {
                previous_owner: *self._owner,
                new_owner: Some(new_owner),
            });

            Lazy::set(&mut self._owner, Some(new_owner));
        }

        /// Get Contract 's Owner
        #[ink(message)]
        pub fn get_owner(&self) -> Option<AccountId> {
            *self._owner
        }

        /// Return the owner AccountId
        pub fn owner(&self) -> &Option<AccountId> {
            &self._owner
        }

        /// Panic if `owner` is not an owner
        pub fn ensure_owner(&self, owner: &AccountId) {
            assert!(&self._owner.unwrap() == owner);
        }

        /// Panic if caller is not an owner
        pub fn ensure_caller_is_owner(&self) {
            self.ensure_owner(&self.env().caller());
        }

        /// Panic the contract owner is not renounced,
        pub fn ensure_owner_renounce(&self) {
            assert!(self._owner.is_none());
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let accounts =
                ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                    .expect("Cannot get accounts");

            let mut owner_test = Owner::new();
            let default_owner = AccountId::from([0x01; 32]);


            let emitted_events = ink_env::test::recorded_events().collect::<Vec<_>>();
            assert_eq!(1, emitted_events.len());

            assert_eq!(owner_test.get_owner(), Some(default_owner));

            owner_test.transfer_ownership(accounts.alice);

            assert_eq!(owner_test.get_owner(), Some(accounts.alice));

            owner_test.renounce_ownership();

            assert_eq!(owner_test.get_owner(), None);
        }
    }
}
