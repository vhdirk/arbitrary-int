use core::fmt;
use std::error::Error;
use crate::error::TryNewError;

pub trait NumberType:
    Sized + fmt::Debug + Copy + Clone + PartialOrd + Ord + Eq + PartialEq + Default
{
}

impl<T> NumberType for T where
    T: Sized + fmt::Debug + Copy + PartialOrd + Ord + Eq + PartialEq + Default
{
}

pub trait Number
where
    Self: Sized,
{
    type UnderlyingType: NumberType;

    type TryNewError: Error;

    const BITS: usize;
    const BYTES: usize;
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;

    fn new(value: Self::UnderlyingType) -> Self;

    fn try_new(value: Self::UnderlyingType) -> Result<Self, TryNewError>;

    /// Returns the type as a fundamental data type
    fn value(self) -> Self::UnderlyingType;

    /// Initializes a new value without checking the bounds
    ///
    /// # Safety
    /// Must only be called with a value less than or equal to [Self::MAX](Self::MAX) value.
    unsafe fn new_unchecked(value: Self::UnderlyingType) -> Self;
}

macro_rules! impl_native {
    ($( $type:ty),+) => {
        $(
            impl Number for $type {
                type UnderlyingType = $type;
                type TryNewError = TryNewError;

                const BITS: usize = <$type>::BITS as usize;
                const BYTES: usize = <$type>::BITS as usize / 8usize;

                const MIN: Self = <$type>::MIN;
                const MAX: Self = <$type>::MAX;

                const ZERO: $type = 0;
                const ONE: $type = 0;

                #[inline]
                fn new(value: Self::UnderlyingType) -> Self { value }

                #[inline]
                fn try_new(value: Self::UnderlyingType) -> Result<Self, Self::TryNewError> { Ok(value) }

                #[inline]
                unsafe fn new_unchecked(value: Self::UnderlyingType) -> Self { value }

                #[inline]
                fn value(self) -> Self::UnderlyingType { self }
            }
        )+
    };
}

impl_native!(u8, u16, u32, u64, u128);
impl_native!(i8, i16, i32, i64, i128);

pub trait Unsigned: Number {}

pub trait Signed: Number {}


macro_rules! impl_native_signed {
    ($sign:ty, ( $( $type:ty ),+) ) => {
        $(
            impl $sign for $type { }
        )+
    };
}

impl_native_signed!(Unsigned, (u8, u16, u32, u64, u128));
impl_native_signed!(Signed, (i8, i16, i32, i64, i128));
