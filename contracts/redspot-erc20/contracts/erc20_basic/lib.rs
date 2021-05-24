#![cfg_attr(not(feature = "std"), no_std)]

mod module;

pub use ::contract::{
    Env,
    EnvAccess,
};

pub use module::{
    Data,
    ModuleAccess,
};

mod erc20_basic {
    use ::contract::{
        Env,
        EnvAccess,
    };
    use super::{
        ModuleAccess,
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

    pub trait Erc20EventEmit<E: Env>: EnvAccess<E> {
        fn emit_event_transfer(
            &mut self,
            from: Option<E::AccountId>,
            to: Option<E::AccountId>,
            value: E::Balance,
        );

        fn emit_event_approval(
            &mut self,
            owner: E::AccountId,
            spender: E::AccountId,
            value: E::Balance,
        );
    }

    pub trait Erc20Impl<E: Env>: ModuleAccess<E> + Erc20EventEmit<E> {
        // logics
        fn new_impl(&mut self, initial_supply: E::Balance) {
            let caller = Self::caller();
            self.erc20_mut().set_total_supply(initial_supply);
            self.erc20_mut().balance_insert(caller.clone(), initial_supply);

            self.emit_event_transfer(None, Some(caller), initial_supply);
        }

        fn transfer_impl(
            &mut self,
            to: E::AccountId,
            value: E::Balance,
        ) -> Result<()> {
            let from = Self::caller();
            self.transfer_from_to_impl(from, to, value)
        }

        fn approve_impl(
            &mut self,
            spender: E::AccountId,
            value: E::Balance,
        ) -> Result<()> {
            let owner = Self::caller();
            self.erc20_mut().allowance_insert((owner.clone(), spender.clone()), value);
            self.emit_event_approval(owner, spender, value);
            Ok(())
        }

        fn transfer_from_impl(
            &mut self,
            from: E::AccountId,
            to: E::AccountId,
            value: E::Balance,
        ) -> Result<()> {
            let caller = Self::caller();
            let allowance = self.erc20().get_allowance(from.clone(), caller.clone());
            if allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to_impl(from.clone(), to, value)?;
            self.erc20_mut().allowance_insert((from, caller), allowance - value);
            Ok(())
        }

        fn transfer_from_to_impl(
            &mut self,
            from: E::AccountId,
            to: E::AccountId,
            value: E::Balance,
        ) -> Result<()> {
            let from_balance = self.erc20().get_balance(from.clone());
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }
            self.erc20_mut().balance_insert(from.clone(), from_balance - value);
            let to_balance = self.erc20().get_balance(to.clone());
            self.erc20_mut().balance_insert(to.clone(), to_balance + value);

            self.emit_event_transfer(Some(from), Some(to), value);

            Ok(())
        }
    }

    impl<E: Env, T: ModuleAccess<E> + Erc20EventEmit<E>> Erc20Impl<E> for T {}
}

pub use erc20_basic::{
    Erc20EventEmit,
    Erc20Impl,
    Error,
    Result,
};
