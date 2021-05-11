#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20swap {
    #[ink(storage)]
    pub struct ERC20Swap {
        value: bool,
    }

    impl ERC20Swap {
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
            let mut swapper = ERC20Swap::new(false);
            assert_eq!(swapper.get(), false);
            swapper.flip();
            assert_eq!(swapper.get(), true);
        }
    }
}
