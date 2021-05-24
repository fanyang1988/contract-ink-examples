#![cfg_attr(not(feature = "std"), no_std)]

mod module;

pub use ::contract::{
    Env,
    EnvAccess,
};

pub use module::Data;

mod owner {
    use super::Data;
    use ::contract::{
        Env,
        EnvAccess,
        Module,
    };

    pub trait EventEmit<E: Env>: EnvAccess<E> {
    }

    pub trait Impl<E: Env>: Module<Data<E>> + EventEmit<E> {
        // logics
        fn new_impl(&mut self) {
            let caller = Self::caller();
            self.get_module_mut().set_ownership(&Some(caller));
        }

        fn renounce_ownership(&mut self) {
            self.ensure_caller_is_owner();

            //Self::env().emit_event(OwnershipTransferred {
            //    previous_owner: *self.get_ownership(),
            //    new_owner: None,
            //});

            self.get_module_mut().set_ownership(&None);
        }

        fn transfer_ownership(&mut self, new_owner: E::AccountId) {
            self.ensure_caller_is_owner();

            //Self::env().emit_event(OwnershipTransferred {
            //    previous_owner: *self.get_ownership(),
            //    new_owner: Some(new_owner),
            //});

            self.get_module_mut().set_ownership(&Some(new_owner));
        }

        /// Return the owner AccountId
        fn owner(&self) -> &Option<E::AccountId> {
            self.get_module().get_ownership()
        }

        /// Panic if `owner` is not an owner
        fn ensure_owner(&self, owner: &E::AccountId) {
            assert!(&self.get_module().get_ownership().clone().unwrap() == owner);
        }

        /// Panic if caller is not an owner
        fn ensure_caller_is_owner(&self) {
            self.ensure_owner(&Self::caller());
        }

        /// Panic the contract owner is not renounced,
        fn ensure_owner_renounce(&self) {
            assert!(self.get_module().get_ownership().is_none());
        }
    }

    impl<E: Env, T: Module<Data<E>> + EventEmit<E>> Impl<E>
        for T
    {
    }
}

pub use owner::{
    EventEmit,
    Impl,
};
