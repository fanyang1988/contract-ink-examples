#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20swap {
    use ink_env::call::FromAccountId;
    // can add `pub use self::erc20::Erc20` to avoid this
    use erc20::erc20::Erc20 as Erc20;

    #[ink(storage)]
    pub struct ERC20Swap {
        value: bool,
        token_account_id: AccountId,
    }

    impl ERC20Swap {
        #[ink(constructor)]
        pub fn new(init_value: bool, account_id: AccountId) -> Self {
            Self { 
                value: init_value,
                token_account_id: account_id,
            }
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default(), AccountId::default())
        }

        #[ink(message)]
        pub fn flip(&mut self) {
            self.value = !self.value;
        }

        #[ink(message)]
        pub fn get(&self) -> AccountId {
            self.token_account_id
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            let token = <Erc20>::from_account_id(self.token_account_id);
            return token.balance_of(owner);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let swapper = ERC20Swap::default();
            assert_eq!(swapper.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut swapper = ERC20Swap::new(false, AccountId::default());
            assert_eq!(swapper.get(), false);
            swapper.flip();
            assert_eq!(swapper.get(), true);
        }
    }
}
