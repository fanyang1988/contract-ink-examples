use core::{
    convert::{
        TryFrom,
        TryInto,
    },
    ops::{
        Add,
        AddAssign,
        Div,
        DivAssign,
        Mul,
        MulAssign,
        Sub,
        SubAssign,
    },
};
use num_traits::{
    checked_pow,
    Bounded,
    CheckedMul,
    One,
    Unsigned,
    Zero,
};

// use ink_env::Environment;

/// Types that allow for simple arithmetic operations.
///
/// Subset of all trait bounds copied over from what Substrate defines
/// for its `BaseArithmetic` types. We can extend this in the future
/// if needed.
pub trait BaseArithmetic:
    Sized
    + From<u8>
    + Bounded
    + Ord
    + PartialOrd<Self>
    + Zero
    + One
    + Bounded
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + SubAssign<Self>
    + Mul<Self, Output = Self>
    + MulAssign<Self>
    + Div<Self, Output = Self>
    + DivAssign<Self>
    + CheckedMul
    + Saturating
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<usize>
    + TryInto<u16>
    + TryInto<u32>
    + TryInto<u64>
    + TryInto<u128>
    + TryInto<usize>
// Further trait bounds from the original BaseArithmetic trait
// that we could use to extend ink!'s BaseArithmetic trait.
//
// UniqueSaturatedInto<u8> +
// UniqueSaturatedInto<u16> +
// UniqueSaturatedInto<u32> +
// UniqueSaturatedInto<u64> +
// UniqueSaturatedInto<u128> +
// UniqueSaturatedFrom<u64> +
// UniqueSaturatedFrom<u128> +
// Shl<u32, Output = Self> +
// Shr<u32, Output = Self> +
// CheckedAdd +
// CheckedSub +
// CheckedDiv +
// CheckedShl +
// CheckedShr +
// IntegerSquareRoot +
{
}

impl<T> BaseArithmetic for T where
    T: Sized
        + From<u8>
        + Bounded
        + Ord
        + PartialOrd<Self>
        + Zero
        + One
        + Add<Self, Output = Self>
        + AddAssign<Self>
        + Sub<Self, Output = Self>
        + SubAssign<Self>
        + Mul<Self, Output = Self>
        + MulAssign<Self>
        + Div<Self, Output = Self>
        + DivAssign<Self>
        + CheckedMul
        + Saturating
        + TryFrom<u16>
        + TryFrom<u32>
        + TryFrom<u64>
        + TryFrom<u128>
        + TryFrom<usize>
        + TryInto<u16>
        + TryInto<u32>
        + TryInto<u64>
        + TryInto<u128>
        + TryInto<usize>
{
}

/// A meta trait for arithmetic (copied from substrate).
///
/// Arithmetic types do all the usual stuff you'd expect numbers to do. They are guaranteed to
/// be able to represent at least `u32` values without loss, hence the trait implies `From<u32>`
/// and smaller integers. All other conversions are fallible.
pub trait AtLeast32Bit: BaseArithmetic + From<u16> + From<u32> {}

impl<T> AtLeast32Bit for T where T: BaseArithmetic + From<u16> + From<u32> {}

/// A meta trait for arithmetic.  Same as [`AtLeast32Bit `], but also bounded to be unsigned.
pub trait AtLeast32BitUnsigned: AtLeast32Bit + Unsigned {}

impl<T> AtLeast32BitUnsigned for T where T: AtLeast32Bit + Unsigned {}

/// Saturating arithmetic operations, returning maximum or minimum values instead of overflowing.
pub trait Saturating {
    /// Saturating addition. Compute `self + rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    fn saturating_add(self, rhs: Self) -> Self;

    /// Saturating subtraction. Compute `self - rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    fn saturating_sub(self, rhs: Self) -> Self;

    /// Saturating multiply. Compute `self * rhs`, saturating at the numeric bounds instead of
    /// overflowing.
    fn saturating_mul(self, rhs: Self) -> Self;

    /// Saturating exponentiation. Compute `self.pow(exp)`, saturating at the numeric bounds
    /// instead of overflowing.
    fn saturating_pow(self, exp: usize) -> Self;
}

impl<T> Saturating for T
where
    T: Clone + Zero + One + PartialOrd + CheckedMul + Bounded + num_traits::Saturating,
{
    fn saturating_add(self, o: Self) -> Self {
        <Self as num_traits::Saturating>::saturating_add(self, o)
    }

    fn saturating_sub(self, o: Self) -> Self {
        <Self as num_traits::Saturating>::saturating_sub(self, o)
    }

    fn saturating_mul(self, o: Self) -> Self {
        self.checked_mul(&o).unwrap_or_else(|| {
            if (self < T::zero()) != (o < T::zero()) {
                Bounded::min_value()
            } else {
                Bounded::max_value()
            }
        })
    }

    fn saturating_pow(self, exp: usize) -> Self {
        let neg = self < T::zero() && exp % 2 != 0;
        checked_pow(self, exp).unwrap_or_else(|| {
            if neg {
                Bounded::min_value()
            } else {
                Bounded::max_value()
            }
        })
    }
}

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

use ::ink_lang::{
    EnvAccess
};

use ::ink_env::Environment;


pub trait Contract{
    type Env: 'static + Environment;
}

pub trait StaticEnvHolder<E: 'static + Environment> : Contract{
    fn env() -> EnvAccess<'static, E>;
}

// impl<'a, E: Environment, C: ContractEnv + StaticEnv> StaticEnvHolder<'a, E> for C {
// fn env() -> EnvAccess<'a, E> {
// <Self as StaticEnv>::env()
// }
// }

pub trait Erc20EnvAccess<E: 'static + Environment>: StaticEnvHolder<E> {
    // type Contract: ContractEnv + StaticEnv;
    type StaticEnv: StaticEnvHolder<E>;
    type Balance: 'static
        + scale::Codec
        + Copy
        + Clone
        + PartialEq
        + Eq
        + AtLeast32BitUnsigned;

    fn caller() -> E::AccountId {
        Self::StaticEnv::env().caller()
    }

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

pub trait Erc20Storage<E: 'static + Environment>: Erc20EnvAccess<E> {
    // get
    fn get_balance(&self, owner: E::AccountId) -> Self::Balance;
    fn get_total_supply(&self) -> Self::Balance;
    fn get_allowance(&self, owner: E::AccountId, spender: E::AccountId) -> Self::Balance;

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

    fn transfer_impl(&mut self, to: E::AccountId, value: Self::Balance) -> Result<()> {
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
