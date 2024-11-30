use crate::error::TryNewError;
use core::fmt;
use std::error::Error;
use std::ops::{Add, Div};

pub trait BitsSpec:
    typenum::Unsigned
    + typenum::Cmp<Self>
    + typenum::IsGreaterOrEqual<typenum::U1, Output = typenum::True>
    + typenum::IsLessOrEqual<typenum::U128, Output = typenum::True>
    + PartialEq<Self>
    + PartialOrd<Self>
{
}

impl<T> BitsSpec for T where
    T: typenum::Unsigned
        + typenum::Cmp<Self>
        + typenum::IsGreaterOrEqual<typenum::U1, Output = typenum::True>
        + typenum::IsLessOrEqual<typenum::U128, Output = typenum::True>
        + PartialEq<T>
        + PartialOrd<T>
{
}

pub trait BytesSpec:
    typenum::Unsigned
    + typenum::Cmp<Self>
    + typenum::IsGreaterOrEqual<typenum::U1, Output = typenum::True>
    + typenum::IsLessOrEqual<typenum::U16, Output = typenum::True>
    + PartialEq<Self>
    + PartialOrd<Self>
{
}

impl<T> BytesSpec for T where
    T: typenum::Unsigned
        + typenum::Cmp<Self>
        + typenum::IsGreaterOrEqual<typenum::U1, Output = typenum::True>
        + typenum::IsLessOrEqual<typenum::U16, Output = typenum::True>
        + PartialEq<T>
        + PartialOrd<T>
{
}

pub trait AIntContainer:
    Sized + fmt::Debug + Copy + Clone + PartialOrd + Ord + Eq + PartialEq + Default
{
    type Bits: BitsSpec;
}

macro_rules! impl_container {
    ($( $type:ty, $bits:ty ),+) => {
        $(
            impl AIntContainer for $type {
                type Bits = $bits;
            }
        )+
    };
}

impl_container!(u8, typenum::U8);
impl_container!(u16, typenum::U16);
impl_container!(u32, typenum::U32);
impl_container!(u64, typenum::U64);
impl_container!(u128, typenum::U128);

impl_container!(i8, typenum::U8);
impl_container!(i16, typenum::U16);
impl_container!(i32, typenum::U32);
impl_container!(i64, typenum::U64);
impl_container!(i128, typenum::U128);

pub trait UnsignedNumberType:
    AIntContainer + From<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128>
{
}

impl<T> UnsignedNumberType for T where
    T: AIntContainer + From<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128>
{
}

pub trait SignedNumberType:
    AIntContainer + From<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64> + TryFrom<i128>
{
}

impl<T> SignedNumberType for T where
    T: AIntContainer + From<i8> + TryFrom<i16> + TryFrom<i32> + TryFrom<i64> + TryFrom<i128>
{
}

pub trait Number
where
    Self: Sized,
{
    type Container: AIntContainer;
    type Bits: BitsSpec;
    type Bytes: BytesSpec;

    type SignedEquivalent: Number + Signed;
    type UnsignedEquivalent: Number + Unsigned;

    type TryNewError: Error;

    const BITS: u32 = <Self::Bits as typenum::Unsigned>::U32;

    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;
    const ONE: Self;

    const MASK: Self::Container;
    const SIGNED: bool;

    fn new(value: Self::Container) -> Self;

    fn try_new(value: Self::Container) -> Result<Self, TryNewError>;

    fn new_wrapping(value: Self::Container) -> Self;

    fn new_saturating(value: Self::Container) -> Self;

    fn new_overflowing(value: Self::Container) -> (Self, bool);

    /// Returns the type as a fundamental data type
    fn value(self) -> Self::Container;

    /// Initializes a new value without checking the bounds
    ///
    /// # Safety
    /// Must only be called with a value less than or equal to [Self::MAX](Self::MAX) value.
    unsafe fn new_unchecked(value: Self::Container) -> Self;

    fn as_signed(self) -> Self::SignedEquivalent;
    fn as_unsigned(self) -> Self::UnsignedEquivalent;
}

macro_rules! impl_native {
    ($( $type:ty, ( $unsigned:ty, $signed:ty ) ),+) => {
        $(
            impl Number for $type {
                type Container = $type;
                type Bits = <$type as AIntContainer>::Bits;
                type Bytes = <<$type as AIntContainer>::Bits as Div<typenum::U8>>::Output;
                type TryNewError = TryNewError;

                type SignedEquivalent = $signed;
                type UnsignedEquivalent = $unsigned;

                const MIN: Self = <$type>::MIN;
                const MAX: Self = <$type>::MAX;

                const ZERO: $type = 0;
                const ONE: $type = 1;

                #[allow(unused_comparisons)]
                const SIGNED: bool = <$type>::MIN  < 0;

                #[allow(unused_comparisons)]
                const MASK: $type = if <$type>::MIN  < 0 { Self::ZERO - 1 } else { Self::MAX };


                #[inline]
                fn new(value: Self::Container) -> Self {
                    value
                }

                #[inline]
                fn try_new(value: Self::Container) -> Result<Self, Self::TryNewError> {
                    Ok(value)
                }

                #[inline]
                fn new_wrapping(value: Self::Container) -> Self {
                    value
                }

                #[inline]
                fn new_saturating(value: Self::Container) -> Self {
                    value
                }

                fn new_overflowing(value: Self::Container) -> (Self, bool) {
                    (value, false)
                }

                #[inline]
                unsafe fn new_unchecked(value: Self::Container) -> Self {
                    value
                }

                #[inline]
                fn value(self) -> Self::Container {
                    self
                }

                fn as_signed(self) -> Self::SignedEquivalent {
                    self as $signed
                }
                fn as_unsigned(self) -> Self::UnsignedEquivalent {
                    self as $unsigned
                }
            }
        )+
    };
}

impl_native!(u8, (u8, i8));
impl_native!(u16, (u16, i16));
impl_native!(u32, (u32, i32));
impl_native!(u64, (u64, i64));
impl_native!(u128, (u128, i128));

impl_native!(i8, (u8, i8));
impl_native!(i16, (u16, i16));
impl_native!(i32, (u32, i32));
impl_native!(i64, (u64, i64));
impl_native!(i128, (u128, i128));

#[allow(unused)]
pub trait Unsigned: Number {}

#[allow(unused)]
pub trait Signed: Number {}
macro_rules! impl_native_signedness {
    ($sign:ty, ( $( $type:ty ),+) ) => {
        $(
            impl $sign for $type { }
        )+
    };
}

impl_native_signedness!(Unsigned, (u8, u16, u32, u64, u128));
impl_native_signedness!(Signed, (i8, i16, i32, i64, i128));
