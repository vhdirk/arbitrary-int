use crate::error::TryNewError;
use core::fmt;
use std::error::Error;

pub trait NumberType:
    Sized + fmt::Debug + Copy + Clone + PartialOrd + Ord + Eq + PartialEq + Default
{
}

impl<T> NumberType for T where
    T: Sized + fmt::Debug + Copy + PartialOrd + Ord + Eq + PartialEq + Default
{
}

pub trait UnsignedNumberType:
    NumberType + From<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128>
{
}

impl<T> UnsignedNumberType for T where
    T: NumberType + From<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128>
{
}

pub trait SignedNumberType:
    NumberType + From<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64> + TryFrom<i128>
{
}

impl<T> SignedNumberType for T where
    T: NumberType + From<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64> + TryFrom<i128>
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

    const MASK: Self::UnderlyingType;
    const SIGNED: bool;

    fn new(value: Self::UnderlyingType) -> Self;

    fn try_new(value: Self::UnderlyingType) -> Result<Self, TryNewError>;

    fn new_wrapping(value: Self::UnderlyingType) -> Self;

    fn new_saturating(value: Self::UnderlyingType) -> (Self, bool);

    fn new_overflowing(value: Self::UnderlyingType) -> (Self, bool);

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
                const ONE: $type = 1;

                #[allow(unused_comparisons)]
                const SIGNED: bool = <$type>::MIN  < 0;

                #[allow(unused_comparisons)]
                const MASK: $type = if <$type>::MIN  < 0 { Self::ZERO - 1 } else { Self::MAX };


                #[inline]
                fn new(value: Self::UnderlyingType) -> Self {
                    value
                }

                #[inline]
                fn try_new(value: Self::UnderlyingType) -> Result<Self, Self::TryNewError> {
                    Ok(value)
                }

                #[inline]
                fn new_wrapping(value: Self::UnderlyingType) -> Self {
                    value
                }

                #[inline]
                fn new_saturating(value: Self::UnderlyingType) -> (Self, bool) {
                    (value, false)
                }

                fn new_overflowing(value: Self::UnderlyingType) -> (Self, bool) {
                    (value, false)
                }

                #[inline]
                unsafe fn new_unchecked(value: Self::UnderlyingType) -> Self {
                    value
                }

                #[inline]
                fn value(self) -> Self::UnderlyingType {
                    self
                }
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
