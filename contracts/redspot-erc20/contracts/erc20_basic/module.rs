#![cfg_attr(not(feature = "std"), no_std)]
pub mod module {
    #[cfg(not(feature = "ink-as-dependency"))]
    use ::ink_storage::{
        collections::HashMap as StorageHashMap,
        lazy::Lazy,
        traits::{
            PackedLayout,
            SpreadLayout,
            StorageLayout,
        },
    };
    pub use ::contract::BalanceInEnv;

    pub trait AccountIdInStorage:
        'static + Ord + Clone + scale_info::TypeInfo + SpreadLayout + PackedLayout
    {
    }

    pub trait BalanceInStorage:
        BalanceInEnv + From<i32> + scale_info::TypeInfo + SpreadLayout + PackedLayout
    {
    }

    #[cfg(not(feature = "ink-as-dependency"))]
    #[cfg_attr(feature = "std", derive(StorageLayout))]
    #[derive(SpreadLayout)]
    #[cfg_attr(test, derive(Debug))]
    pub struct Erc20<AccountId, Balance>
    where
        AccountId: AccountIdInStorage,
        Balance: BalanceInStorage,
    {
        /// Total token supply.
        total_supply: Lazy<Balance>,
        /// Mapping from owner to number of owned token.
        balances: StorageHashMap<AccountId, Balance>,
        /// Mapping of the token amount which an account is allowed to withdraw
        /// from another account.
        allowances: StorageHashMap<(AccountId, AccountId), Balance>,
    }

    // Erc20Storage<<Erc20 as ContractEnv>::Env> for
    impl<AccountId, Balance> Erc20<AccountId, Balance>
    where
        AccountId: AccountIdInStorage,
        Balance: BalanceInStorage,
    {
        fn get_balance(&self, owner: AccountId) -> Balance {
            self.balances
                .get(&owner)
                .copied()
                .unwrap_or(Balance::from(0))
        }

        fn get_allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances
                .get(&(owner, spender))
                .copied()
                .unwrap_or(Balance::from(0))
        }

        fn get_total_supply(&self) -> Balance {
            *self.total_supply
        }

        fn set_total_supply(&mut self, total_supply: Balance) {
            Lazy::set(&mut self.total_supply, total_supply);
        }

        fn balance_insert(&mut self, owner: AccountId, value: Balance) {
            self.balances.insert(owner, value);
        }

        fn allowance_insert(
            &mut self,
            owner_spender: (AccountId, AccountId),
            value: Balance,
        ) {
            self.allowances.insert(owner_spender, value);
        }
    }
}

pub use module::Erc20;
