
use core::ops::{Add, Shr};
use crate::util::{Assert, CompileTimeAssert};
use crate::{AInt, AIntContainer, Number, TryNewError, AIntErrorKind};
use core::fmt::Debug;
use crate::bits::ConstBounded;

// Conversions

macro_rules! aint_impl_from_bool {
    ($($type:ident),+) => {
        $(
            impl From<bool> for AInt<$type, 1>  {
                #[inline]
                fn from(value: bool) -> Self {
                    unsafe {
                        AInt::<$type, 1>::new_unchecked(value as $type)
                    }
                }
            }

            impl From<AInt<$type, 1>> for bool {
                #[inline]
                fn from(value: AInt<$type, 1>) -> Self {
                    match value.value {
                        0 => false,
                        1 => true,
                        _ => panic!("arbitrary-int already validates that this is unreachable"), //TODO: unreachable!() is not const yet
                    }
                }
            }
        )+
    };
}


aint_impl_from_bool!(u8, u16, u32, u64, u128);



// Implement From for any type thas has the same amount or more bits available
macro_rules! aint_impl_from_aint {
    ($from:ty, [$($into:ty),+]) => {
        $(
            impl<const BITS: usize, const FROM_BITS: usize> From<AInt<$from, FROM_BITS>> for AInt<$into, BITS>
            where
                $from: AIntContainer + Debug,
                $into: AIntContainer + Debug,
                AInt<$from, FROM_BITS>: Number<Container=$from>,
                AInt<$into, FROM_BITS>: Number<Container=$into>,
                ConstBounded<BITS, FROM_BITS, 128>: Sized
            {
                #[inline]
                fn from(item: AInt<$from, FROM_BITS>) -> Self {
                    unsafe {
                        Self::new_unchecked(item.value as $into)
                    }
                }
            }
        )+
    };
}

// Pity: we can;t use from/into for AInt with the same container type.
// Conflicts with the From<T> for T blanket impl
aint_impl_from_aint!(u8, [u16, u32, u64, u128]);
aint_impl_from_aint!(u16, [u8, u32, u64, u128]);
aint_impl_from_aint!(u32, [u8, u16, u64, u128]);
aint_impl_from_aint!(u64, [u8, u16, u32, u128]);
aint_impl_from_aint!(u128, [u8, u16, u32, u64]);

aint_impl_from_aint!(i8, [i16, i32, i64, i128]);
aint_impl_from_aint!(i16, [i8, i32, i64, i128]);
aint_impl_from_aint!(i32, [i8, i16, i64, i128]);
aint_impl_from_aint!(i64, [i8, i16, i32, i128]);
aint_impl_from_aint!(i128, [i8, i16, i32, i64]);


// Implement From for unsigned into signed if the signed type has at least 1 bit more room
macro_rules! aint_impl_from_unsigned {
    ($from:ty, [$($into:ty),+]) => {
        $(
            impl<const BITS: usize, const FROM_BITS: usize> From<AInt<$from, FROM_BITS>> for AInt<$into, BITS>
            where
                $from: AIntContainer + Debug,
                $into: AIntContainer + Debug,
                AInt<$from, FROM_BITS>: Number<Container=$from>,
                AInt<$into, FROM_BITS>: Number<Container=$into>,
                ConstBounded<BITS, FROM_BITS, 128>: Sized,
                ConstBounded<FROM_BITS, 1, BITS>: Sized
            {
                #[inline]
                fn from(item: AInt<$from, FROM_BITS>) -> Self {
                    unsafe {
                        Self::new_unchecked(item.value as $into)
                    }
                }
            }

            impl<const BITS: usize> From<$from> for AInt<$into, BITS>
            where
                $into: AIntContainer + Debug,
                AInt<$into, BITS>: Number<Container=$into>,
                ConstBounded<BITS, { <$from>::BITS as usize}, 128>: Sized,
                ConstBounded< { <$from>::BITS as usize}, 1, BITS>: Sized
            {
                #[inline]
                fn from(item: $from) -> Self {
                    unsafe {
                        Self::new_unchecked(item as $into)
                    }
                }
            }
        )+
    };
}

aint_impl_from_unsigned!(u8, [i16, i32, i64, i128]);
aint_impl_from_unsigned!(u16, [i8, i32, i64, i128]);
aint_impl_from_unsigned!(u32, [i8, i16, i64, i128]);
aint_impl_from_unsigned!(u64, [i8, i16, i32, i128]);
aint_impl_from_unsigned!(u128, [i8, i16, i32, i64]);


// Implement From for any type thas has the same amount or more bits available
macro_rules! aint_impl_from_native {
    ($from:ty, [$($into:ty),+]) => {
        $(
            impl<const BITS: usize> From<$from> for AInt<$into, BITS>
            where
                $into: AIntContainer + Debug,
                AInt<$into, BITS>: Number<Container=$into>,
                ConstBounded<BITS, { <$from>::BITS as usize}, 128>: Sized,
                ConstBounded< { <$from>::BITS as usize}, 1, BITS>: Sized
            {
                #[inline]
                fn from(item: $from) -> Self {
                    Self::new_wrapping(item as $into)
                }
            }

            impl<const BITS: usize> From<AInt<$from, BITS>> for $into
            where
                $from: AIntContainer + Debug,
                AInt<$from, BITS>: Number<Container=$from>,
                ConstBounded<{ <$into>::BITS as usize}, BITS, 128>: Sized,
                ConstBounded<BITS, 1, { <$into>::BITS as usize}>: Sized
            {
                #[inline]
                fn from(item: AInt<$from, BITS>) -> Self {
                    item.value() as $into
                }
            }
        )+
    };
}


aint_impl_from_native!(u8, [u8, u16, u32, u64, u128]);
aint_impl_from_native!(u16, [u8, u16, u32, u64, u128]);
aint_impl_from_native!(u32, [u8, u16, u32, u64, u128]);
aint_impl_from_native!(u64, [u8, u16, u32, u64, u128]);
aint_impl_from_native!(u128, [u8, u16, u32, u64, u128]);

aint_impl_from_native!(i8, [i8, i16, i32, i64, i128]);
aint_impl_from_native!(i16, [i8, i16, i32, i64, i128]);
aint_impl_from_native!(i32, [i8, i16, i32, i64, i128]);
aint_impl_from_native!(i64, [i8, i16, i32, i64, i128]);
aint_impl_from_native!(i128, [i8, i16, i32, i64, i128]);



// Implement From for unsigned into signed if the signed type has at least 1 bit more room
macro_rules! aint_impl_try_from_signed {
    ($from:ty, [$($into:ty),+]) => {
        $(
            impl<const FROM_BITS: usize, const BITS: usize> TryFrom<AInt<$from, FROM_BITS>> for AInt<$into, BITS>
            where
                $into: TryFrom<$from>
            {
                type Error = TryNewError;

                #[inline]
                fn try_from(item: AInt<$from, FROM_BITS>) -> Result<Self, Self::Error> {
                    if item.value() >= 0 {
                        Self::try_new(item.value as $into)
                    } else {
                        Err(TryNewError{kind: AIntErrorKind::NegOverflow})
                    }

                }
            }

            impl<const BITS: usize> TryFrom<$from> for AInt<$into, BITS>
            where
                $into: TryFrom<$from>
            {
                type Error = TryNewError;

                #[inline]
                fn try_from(item: $from) -> Result<Self, Self::Error> {
                    if item >= 0 {
                        Self::try_new(item as $into)
                    } else {
                        Err(TryNewError{kind: AIntErrorKind::NegOverflow})
                    }
                }
            }
        )+
    };
}

aint_impl_try_from_signed!(i8, [u16, u32, u64, u128]);
aint_impl_try_from_signed!(i16, [u8, u32, u64, u128]);
aint_impl_try_from_signed!(i32, [u8, u16, u64, u128]);
aint_impl_try_from_signed!(i64, [u8, u16, u32, u128]);
aint_impl_try_from_signed!(i128, [u8, u16, u32, u64]);


#[cfg(test)]
mod tests {
    use super::*;
    use crate::aliases::*;

    #[test]
    fn from_same_bit_widths() {
        assert_eq!(u5::from(AInt::<u8, 5>::new(0b10101)), u5::new(0b10101));
        assert_eq!(u5::from(AInt::<u16, 5>::new(0b10101)), u5::new(0b10101));
        assert_eq!(u5::from(AInt::<u32, 5>::new(0b10101)), u5::new(0b10101));
        assert_eq!(u5::from(AInt::<u64, 5>::new(0b10101)), u5::new(0b10101));

        assert_eq!(u5::from(AInt::<u128, 5>::new(0b10101)), u5::new(0b10101));

        assert_eq!(
            AInt::<u8, 8>::from(AInt::<u128, 8>::new(0b1110_0101)),
            AInt::<u8, 8>::new(0b1110_0101)
        );

        assert_eq!(
            AInt::<u16, 6>::from(AInt::<u8, 5>::new(0b10101)),
            AInt::<u16, 6>::new(0b10101)
        );
        assert_eq!(u15::from(AInt::<u16, 15>::new(0b10101)), u15::new(0b10101));
        assert_eq!(u15::from(AInt::<u32, 15>::new(0b10101)), u15::new(0b10101));
        assert_eq!(u15::from(AInt::<u64, 15>::new(0b10101)), u15::new(0b10101));

        assert_eq!(u15::from(AInt::<u128, 15>::new(0b10101)), u15::new(0b10101));

        assert_eq!(
            AInt::<u32, 6>::from(u6::new(0b10101)),
            AInt::<u32, 6>::new(0b10101)
        );
        assert_eq!(
            AInt::<u32, 14>::from(u14::new(0b10101)),
            AInt::<u32, 14>::new(0b10101)
        );
        assert_eq!(u30::from(AInt::<u32, 30>::new(0b10101)), u30::new(0b10101));
        assert_eq!(u30::from(AInt::<u64, 30>::new(0b10101)), u30::new(0b10101));

        assert_eq!(u30::from(AInt::<u128, 30>::new(0b10101)), u30::new(0b10101));

        assert_eq!(
            AInt::<u64, 7>::from(AInt::<u8, 7>::new(0b10101)),
            AInt::<u64, 7>::new(0b10101)
        );
        assert_eq!(
            AInt::<u64, 12>::from(AInt::<u16, 12>::new(0b10101)),
            AInt::<u64, 12>::new(0b10101)
        );
        assert_eq!(
            AInt::<u64, 28>::from(AInt::<u32, 28>::new(0b10101)),
            AInt::<u64, 28>::new(0b10101)
        );
        assert_eq!(u60::from(u60::new(0b10101)), u60::new(0b10101));

        assert_eq!(u60::from(AInt::<u128, 60>::new(0b10101)), u60::new(0b10101));

        assert_eq!(
            AInt::<u128, 5>::from(AInt::<u8, 5>::new(0b10101)),
            AInt::<u128, 5>::new(0b10101)
        );
        assert_eq!(
            AInt::<u128, 12>::from(AInt::<u16, 12>::new(0b10101)),
            AInt::<u128, 12>::new(0b10101)
        );
        assert_eq!(
            AInt::<u128, 26>::from(AInt::<u32, 26>::new(0b10101)),
            AInt::<u128, 26>::new(0b10101)
        );
        assert_eq!(
            AInt::<u128, 60>::from(AInt::<u64, 60>::new(0b10101)),
            AInt::<u128, 60>::new(0b10101)
        );

        assert_eq!(
            u120::from(AInt::<u128, 120>::new(0b10101)),
            u120::new(0b10101)
        );
    }

    #[test]
    fn from_smaller_bit_widths() {
        // The code to get more bits from fewer bits (through From) is the same as the code above
        // for identical bitwidths. Therefore just do a few point checks to ensure things compile

        // There are compile-breakers for the opposite direction (e.g. tryint to do u5 = From(u17),
        // but we can't test compile failures here

        // from is not yet supported if the bitcounts are different but the base data types are the same (need
        // fancier Rust features to support that)
        assert_eq!(u6::from(AInt::<u16, 5>::new(0b10101)), u6::new(0b10101));
        assert_eq!(u6::from(AInt::<u32, 5>::new(0b10101)), u6::new(0b10101));
        assert_eq!(u6::from(AInt::<u64, 5>::new(0b10101)), u6::new(0b10101));

        assert_eq!(u6::from(AInt::<u128, 5>::new(0b10101)), u6::new(0b10101));

        assert_eq!(u15::from(AInt::<u8, 7>::new(0b10101)), u15::new(0b10101));
        //assert_eq!(u15::from(AInt::<u16, 15>::new(0b10101)), u15::new(0b10101));
        assert_eq!(u15::from(AInt::<u32, 14>::new(0b10101)), u15::new(0b10101));
        assert_eq!(u15::from(AInt::<u64, 14>::new(0b10101)), u15::new(0b10101));

        assert_eq!(u15::from(AInt::<u128, 14>::new(0b10101)), u15::new(0b10101));
    }

    #[allow(non_camel_case_types)]
    #[test]
    fn from_native_ints_same_bits() {
        use std::primitive;

        type u8 = AInt<primitive::u8, 8>;
        type u16 = AInt<primitive::u16, 16>;
        type u32 = AInt<primitive::u32, 32>;
        type u64 = AInt<primitive::u64, 64>;
        type u128 = AInt<primitive::u128, 128>;

        assert_eq!(u8::from(0x80_u8), u8::new(0x80));
        assert_eq!(u16::from(0x8000_u16), u16::new(0x8000));
        assert_eq!(u32::from(0x8000_0000_u32), u32::new(0x8000_0000));
        assert_eq!(
            u64::from(0x8000_0000_0000_0000_u64),
            u64::new(0x8000_0000_0000_0000)
        );

        assert_eq!(
            u128::from(0x8000_0000_0000_0000_0000_0000_0000_0000_u128),
            u128::new(0x8000_0000_0000_0000_0000_0000_0000_0000)
        );
    }

    #[test]
    fn from_native_ints_fewer_bits() {
        assert_eq!(u9::from(0x80_u8), u9::new(0x80));

        assert_eq!(u17::from(0x80_u8), u17::new(0x80));
        assert_eq!(u17::from(0x8000_u16), u17::new(0x8000));

        assert_eq!(u33::from(0x80_u8), u33::new(0x80));
        assert_eq!(u33::from(0x8000_u16), u33::new(0x8000));
        assert_eq!(u33::from(0x8000_0000_u32), u33::new(0x8000_0000));

        assert_eq!(u65::from(0x80_u8), u65::new(0x80));

        assert_eq!(u65::from(0x8000_u16), u65::new(0x8000));

        assert_eq!(u65::from(0x8000_0000_u32), u65::new(0x8000_0000));

        assert_eq!(
            u65::from(0x8000_0000_0000_0000_u64),
            u65::new(0x8000_0000_0000_0000)
        );
    }

    #[allow(non_camel_case_types)]
    #[test]
    fn into_native_ints_same_bits() {
        assert_eq!(u8::from(AInt::<u8, 8>::new(0x80)), 0x80);
        assert_eq!(u16::from(AInt::<u16, 16>::new(0x8000)), 0x8000);
        assert_eq!(u32::from(AInt::<u32, 32>::new(0x8000_0000)), 0x8000_0000);
        assert_eq!(
            u64::from(AInt::<u64, 64>::new(0x8000_0000_0000_0000)),
            0x8000_0000_0000_0000
        );

        assert_eq!(
            u128::from(AInt::<u128, 128>::new(
                0x8000_0000_0000_0000_0000_0000_0000_0000
            )),
            0x8000_0000_0000_0000_0000_0000_0000_0000
        );
    }

    #[test]
    fn into_native_ints_fewer_bits() {
        assert_eq!(u8::from(u7::new(0x40)), 0x40);
        assert_eq!(u16::from(u15::new(0x4000)), 0x4000);
        assert_eq!(u32::from(u31::new(0x4000_0000)), 0x4000_0000);
        assert_eq!(
            u64::from(u63::new(0x4000_0000_0000_0000)),
            0x4000_0000_0000_0000
        );

        assert_eq!(
            u128::from(u127::new(0x4000_0000_0000_0000_0000_0000_0000_0000)),
            0x4000_0000_0000_0000_0000_0000_0000_0000
        );
    }

    #[test]
    fn from_into_bool() {
        assert_eq!(u1::from(true), u1::new(1));
        assert_eq!(u1::from(false), u1::new(0));
        assert_eq!(bool::from(u1::new(1)), true);
        assert_eq!(bool::from(u1::new(0)), false);
    }
}
