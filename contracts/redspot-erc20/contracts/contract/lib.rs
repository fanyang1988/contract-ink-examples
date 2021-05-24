#![cfg_attr(not(feature = "std"), no_std)]

mod traits;
mod modules;

pub use traits::{
    AccountId,
    Balance,
    BlockNumber,
    ChainExtension,
    Hash,
    Timestamp,
};

pub mod contract {
    pub trait Env: 'static {
        type AccountId: super::AccountId;
        type Balance: super::Balance;
        type BlockNumber: super::BlockNumber;
        type Hash: super::Hash;
        type Timestamp: super::Timestamp;
    }

    pub trait EnvAccess<E: Env> {
        fn caller() -> E::AccountId;
        fn transferred_balance() -> E::Balance;
    }
}

pub use contract::{
    Env,
    EnvAccess,
};

pub use modules::{
    Module,
};
