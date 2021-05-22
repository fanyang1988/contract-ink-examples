#![cfg_attr(not(feature = "std"), no_std)]

pub use ::contract::{
    ContractEnvAccess,
    ContractWithEnv,
};

mod erc20_basic {
    use ::ink_env::Environment;
    use ::contract::{
        ContractEnvAccess,
    };

    /// The ERC-20 error types.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if not enough balance to fulfill a request is available.
        InsufficientBalance,
        /// Returned if not enough allowance to fulfill a request is available.
        InsufficientAllowance,
    }

    /// The ERC-20 result type.
    pub type Result<T> = core::result::Result<T, Error>;

    pub trait Erc20EventEmit<E: 'static + Environment>: ContractEnvAccess<E> {
        fn emit_event_transfer(
            &mut self,
            from: Option<E::AccountId>,
            to: Option<E::AccountId>,
            value: Self::Balance,
        );

        fn emit_event_approval(
            &mut self,
            owner: E::AccountId,
            spender: E::AccountId,
            value: Self::Balance,
        );
    }

    pub trait Erc20Storage<E: 'static + Environment>: Erc20EventEmit<E> {
        // get
        fn get_balance(&self, owner: E::AccountId) -> Self::Balance;
        fn get_total_supply(&self) -> Self::Balance;
        fn get_allowance(
            &self,
            owner: E::AccountId,
            spender: E::AccountId,
        ) -> Self::Balance;

        // set
        fn set_total_supply(&mut self, total_supply: Self::Balance);

        fn balance_insert(&mut self, owner: E::AccountId, value: Self::Balance);
        fn allowance_insert(
            &mut self,
            owner_spender: (E::AccountId, E::AccountId),
            value: Self::Balance,
        );
    }

    pub trait Erc20Impl<E: 'static + Environment>: Erc20Storage<E> {
        // logics
        fn new_impl(&mut self, initial_supply: Self::Balance) {
            let caller = Self::caller();
            self.set_total_supply(initial_supply);
            self.balance_insert(caller.clone(), initial_supply);

            self.emit_event_transfer(None, Some(caller), initial_supply);
        }

        fn transfer_impl(
            &mut self,
            to: E::AccountId,
            value: Self::Balance,
        ) -> Result<()> {
            let from = Self::caller();
            self.transfer_from_to_impl(from, to, value)
        }

        fn approve_impl(
            &mut self,
            spender: E::AccountId,
            value: Self::Balance,
        ) -> Result<()> {
            let owner = Self::caller();
            self.allowance_insert((owner.clone(), spender.clone()), value);
            self.emit_event_approval(owner, spender, value);
            Ok(())
        }

        fn transfer_from_impl(
            &mut self,
            from: E::AccountId,
            to: E::AccountId,
            value: Self::Balance,
        ) -> Result<()> {
            let caller = Self::caller();
            let allowance = self.get_allowance(from.clone(), caller.clone());
            if allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to_impl(from.clone(), to, value)?;
            self.allowance_insert((from, caller), allowance - value);
            Ok(())
        }

        fn transfer_from_to_impl(
            &mut self,
            from: E::AccountId,
            to: E::AccountId,
            value: Self::Balance,
        ) -> Result<()> {
            let from_balance = self.get_balance(from.clone());
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }
            self.balance_insert(from.clone(), from_balance - value);
            let to_balance = self.get_balance(to.clone());
            self.balance_insert(to.clone(), to_balance + value);

            self.emit_event_transfer(Some(from), Some(to), value);

            Ok(())
        }
    }

    impl<E: 'static + Environment, T: Erc20Storage<E>> Erc20Impl<E> for T {}
}

pub use erc20_basic::{
    Error,
    Erc20EventEmit,
    Erc20Impl,
    Erc20Storage,
    Result,
};