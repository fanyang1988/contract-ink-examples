#![cfg_attr(not(feature = "std"), no_std)]

pub mod contract {
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

    use ::ink_env::Environment;
    use ::ink_lang::EnvAccess;

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
        T: Clone
            + Zero
            + One
            + PartialOrd
            + CheckedMul
            + Bounded
            + num_traits::Saturating,
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

    pub trait ContractWithEnv<E: 'static + Environment> {
        type Balance: 'static
            + scale::Codec
            + Copy
            + Clone
            + PartialEq
            + Eq
            + AtLeast32BitUnsigned;
        fn env() -> EnvAccess<'static, E>;
    }

    pub trait ContractEnvAccess<E: 'static + Environment>: ContractWithEnv<E> {
        #[inline]
        fn caller() -> E::AccountId {
            Self::env().caller()
        }
    }

    impl<E: 'static + Environment, C: ContractWithEnv<E>> ContractEnvAccess<E> for C {}
}

pub use contract::{
    ContractWithEnv,
    ContractEnvAccess,
};