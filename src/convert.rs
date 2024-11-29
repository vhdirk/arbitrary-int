
use crate::traits::BitsSpec;
use crate::util::{Assert, CompileTimeAssert, NotSame, NotSame2};
use crate::{AInt, AIntContainer, Number, TryNewError};

// Conversions

macro_rules! aint_impl_from_bool {
    ($($type:ident),+) => {
        $(
            impl From<bool> for AInt<$type, typenum::U1>  {
                #[inline]
                fn from(value: bool) -> Self {
                    unsafe {
                        AInt::<$type, typenum::U1>::new_unchecked(value as $type)
                    }
                }
            }

            impl From<AInt<$type, typenum::U1>> for bool {
                #[inline]
                fn from(value: AInt<$type, typenum::U1>) -> Self {
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
    ($into:ty, [$($from:ty),+]) => {
        $(
            impl<Bits, FromBits> From<AInt<$from, FromBits>> for AInt<$into, Bits>
            where
                Bits: BitsSpec,
                <$from as AIntContainer>::Bits: typenum::IsGreaterOrEqual<FromBits, Output = typenum::True>,
                <$into as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
                FromBits: BitsSpec + typenum::IsLessOrEqual<Bits, Output = typenum::True>,
            {
                #[inline]
                fn from(item: AInt<$from, FromBits>) -> Self {
                    unsafe {
                        Self::new_unchecked(item.value as $into)
                    }
                }
            }
        )+
    };
}

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
macro_rules! aint_impl_from_unsigned_aint {
    ($into:ty, [$($from:ty),+]) => {
        $(
            impl<Bits, FromBits> From<AInt<$from, FromBits>> for AInt<$into, Bits>
            where
                Bits: BitsSpec,
                <$from as AIntContainer>::Bits: typenum::IsGreaterOrEqual<FromBits, Output = typenum::True>,
                <$into as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
                FromBits: BitsSpec + typenum::IsLess<Bits, Output = typenum::True>,
            {
                #[inline]
                fn from(item: AInt<$from, FromBits>) -> Self {
                    unsafe {
                        Self::new_unchecked(item.value as $into)
                    }
                }
            }
        )+
    };
}

aint_impl_from_unsigned_aint!(i8, [u16, u32, u64, u128]);
aint_impl_from_unsigned_aint!(i16, [u8, u32, u64, u128]);
aint_impl_from_unsigned_aint!(i32, [u8, u16, u64, u128]);
aint_impl_from_unsigned_aint!(i64, [u8, u16, u32, u128]);
aint_impl_from_unsigned_aint!(i128, [u8, u16, u32, u64]);


// Implement From for any type thas has the same amount or more bits available
macro_rules! aint_impl_from_native {
    ($into:ty, [$($from:ty),+]) => {
        $(
            impl<Bits> From<$from> for AInt<$into, Bits>
            where
                Bits: BitsSpec,
                $from: Number,
                <$from as Number>::Bits: typenum::IsLessOrEqual<Bits, Output = typenum::True>,
                Self: Number<Container=$into, Bits=Bits>,
                <$into as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>
            {
                #[inline]
                fn from(item: $from) -> Self {
                    Self::new_wrapping(item as $into)
                }
            }

            impl<Bits> From<AInt<$from, Bits>> for $into
            where
                Bits: BitsSpec,
                $from: AIntContainer,
                <$from as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
                $into: Number,
                <$into as Number>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>
            {
                #[inline]
                fn from(item: AInt<$from, Bits>) -> Self {
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

// macro_rules! from_native_impl {
//     ($into:ty, [$($from:ty),+]) => {
//         $(
//             impl<Bits> From<$from> for AInt<$into, Bits> {
//                 #[inline]
//                 fn from(from: $from) -> Self {
//                     let _ = CompileTimeAssert::<{<$from>::BITS as usize}, BITS>::LESSER_OR_EQUAL;
//                     Self { value: from as $into }
//                 }
//             }

//             impl<Bits> From<AInt<$from, Bits>> for $into {
//                 #[inline]
//                 fn from(from: AInt<$from, BITS>) -> Self {
//                     let _ = CompileTimeAssert::<BITS, {<$into>::BITS as usize}>::LESSER_OR_EQUAL;
//                     from.value as $into
//                 }
//             }
//         )+
//     };
// }

// from_native_impl!(u8, [u8]);
// from_native_impl!(u16, [u8, u16]);
// from_native_impl!(u32, [u8, u16, u32]);
// from_native_impl!(u64, [u8, u16, u32, u64]);
// from_native_impl!(u128, [u8, u16, u32, u64, u128]);

// from_native_impl!(i8, [i8]);
// from_native_impl!(i16, [i8, i16]);
// from_native_impl!(i32, [i8, i16, i32]);
// from_native_impl!(i64, [i8, i16, i32, i64]);
// from_native_impl!(i128, [i8, i16, i32, i64, i128]);


// macro_rules! try_from_native_impl {
//     ($into:ty, [$($from:ty),+]) => {
//         $(
//             impl<const BITS: usize> TryFrom<$from> for AInt<$into, BITS> {
//                 type Error = TryNewError;

//                 #[inline]
//                 fn try_from(from: $from) -> Result<Self, Self::Error> {
//                     Self::try_new(from as $into)
//                 }
//             }
//         )+
//     };
// }

// try_from_native_impl!(u8, [i16, i32, i64, i128]);
// try_from_native_impl!(u16, [u32, u64, u128, i8, i16, i32, i64, i128]);
// try_from_native_impl!(u32, [u64, u128, i8, i16, i32, i64, i128]);
// try_from_native_impl!(u64, [u128, i8, i16, i32, i64, i128]);
// try_from_native_impl!(u128, [i8, i16, i32, i64, i128]);

// try_from_native_impl!(i8, [u8, u16, u32, u64, u128, i16, i32, i64, i128]);
// try_from_native_impl!(i16, [u8, u16, u32, u64, u128, i32, i64, i128]);
// try_from_native_impl!(i32, [u8, u16, u32, u64, u128, i64, i128]);
// try_from_native_impl!(i64, [u8, u16, u32, u64, u128, i128]);
// try_from_native_impl!(i128, [u8, u16, u32, u64, u128]);



#[cfg(test)]
mod tests {
    use super::*;
    use crate::aliases::*;

    #[test]
    fn from_same_bit_widths() {
        assert_eq!(u5::from(AInt::<u8, typenum::U5>::new(0b10101)), u5::new(0b10101));
        assert_eq!(u5::from(AInt::<u16, typenum::U5>::new(0b10101)), u5::new(0b10101));
        assert_eq!(u5::from(AInt::<u32, typenum::U5>::new(0b10101)), u5::new(0b10101));
        assert_eq!(u5::from(AInt::<u64, typenum::U5>::new(0b10101)), u5::new(0b10101));

        assert_eq!(u5::from(AInt::<u128, typenum::U5>::new(0b10101)), u5::new(0b10101));

        assert_eq!(
            AInt::<u8, typenum::U8>::from(AInt::<u128, typenum::U8>::new(0b1110_0101)),
            AInt::<u8, typenum::U8>::new(0b1110_0101)
        );

        assert_eq!(
            AInt::<u16, typenum::U6>::from(AInt::<u8, typenum::U5>::new(0b10101)),
            AInt::<u16, typenum::U6>::new(0b10101)
        );
        assert_eq!(u15::from(AInt::<u16, typenum::U15>::new(0b10101)), u15::new(0b10101));
        assert_eq!(u15::from(AInt::<u32, typenum::U15>::new(0b10101)), u15::new(0b10101));
        assert_eq!(u15::from(AInt::<u64, typenum::U15>::new(0b10101)), u15::new(0b10101));

        assert_eq!(u15::from(AInt::<u128, typenum::U15>::new(0b10101)), u15::new(0b10101));

        assert_eq!(
            AInt::<u32, typenum::U6>::from(u6::new(0b10101)),
            AInt::<u32, typenum::U6>::new(0b10101)
        );
        assert_eq!(
            AInt::<u32, typenum::U14>::from(u14::new(0b10101)),
            AInt::<u32, typenum::U14>::new(0b10101)
        );
        assert_eq!(u30::from(AInt::<u32, typenum::U30>::new(0b10101)), u30::new(0b10101));
        assert_eq!(u30::from(AInt::<u64, typenum::U30>::new(0b10101)), u30::new(0b10101));

        assert_eq!(u30::from(AInt::<u128, typenum::U30>::new(0b10101)), u30::new(0b10101));

        assert_eq!(
            AInt::<u64, typenum::U7>::from(AInt::<u8, typenum::U7>::new(0b10101)),
            AInt::<u64, typenum::U7>::new(0b10101)
        );
        assert_eq!(
            AInt::<u64, typenum::U12>::from(AInt::<u16, typenum::U12>::new(0b10101)),
            AInt::<u64, typenum::U12>::new(0b10101)
        );
        assert_eq!(
            AInt::<u64, typenum::U28>::from(AInt::<u32, typenum::U28>::new(0b10101)),
            AInt::<u64, typenum::U28>::new(0b10101)
        );
        assert_eq!(u60::from(u60::new(0b10101)), u60::new(0b10101));

        assert_eq!(u60::from(AInt::<u128, typenum::U60>::new(0b10101)), u60::new(0b10101));

        assert_eq!(
            AInt::<u128, typenum::U5>::from(AInt::<u8, typenum::U5>::new(0b10101)),
            AInt::<u128, typenum::U5>::new(0b10101)
        );
        assert_eq!(
            AInt::<u128, typenum::U12>::from(AInt::<u16, typenum::U12>::new(0b10101)),
            AInt::<u128, typenum::U12>::new(0b10101)
        );
        assert_eq!(
            AInt::<u128, typenum::U26>::from(AInt::<u32, typenum::U26>::new(0b10101)),
            AInt::<u128, typenum::U26>::new(0b10101)
        );
        assert_eq!(
            AInt::<u128, typenum::U60>::from(AInt::<u64, typenum::U60>::new(0b10101)),
            AInt::<u128, typenum::U60>::new(0b10101)
        );

        assert_eq!(
            u120::from(AInt::<u128, typenum::U120>::new(0b10101)),
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
        assert_eq!(u6::from(AInt::<u16, typenum::U5>::new(0b10101)), u6::new(0b10101));
        assert_eq!(u6::from(AInt::<u32, typenum::U5>::new(0b10101)), u6::new(0b10101));
        assert_eq!(u6::from(AInt::<u64, typenum::U5>::new(0b10101)), u6::new(0b10101));

        assert_eq!(u6::from(AInt::<u128, typenum::U5>::new(0b10101)), u6::new(0b10101));

        assert_eq!(u15::from(AInt::<u8, typenum::U7>::new(0b10101)), u15::new(0b10101));
        //assert_eq!(u15::from(AInt::<u16, typenum::U15>::new(0b10101)), u15::new(0b10101));
        assert_eq!(u15::from(AInt::<u32, typenum::U14>::new(0b10101)), u15::new(0b10101));
        assert_eq!(u15::from(AInt::<u64, typenum::U14>::new(0b10101)), u15::new(0b10101));

        assert_eq!(u15::from(AInt::<u128, typenum::U14>::new(0b10101)), u15::new(0b10101));
    }

    #[allow(non_camel_case_types)]
    #[test]
    fn from_native_ints_same_bits() {
        use std::primitive;

        type u8 = AInt<primitive::u8, typenum::U8>;
        type u16 = AInt<primitive::u16, typenum::U16>;
        type u32 = AInt<primitive::u32, typenum::U32>;
        type u64 = AInt<primitive::u64, typenum::U64>;
        type u128 = AInt<primitive::u128, typenum::U128>;

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
        assert_eq!(u8::from(AInt::<u8, typenum::U8>::new(0x80)), 0x80);
        assert_eq!(u16::from(AInt::<u16, typenum::U16>::new(0x8000)), 0x8000);
        assert_eq!(u32::from(AInt::<u32, typenum::U32>::new(0x8000_0000)), 0x8000_0000);
        assert_eq!(
            u64::from(AInt::<u64, typenum::U64>::new(0x8000_0000_0000_0000)),
            0x8000_0000_0000_0000
        );

        assert_eq!(
            u128::from(AInt::<u128, typenum::U128>::new(
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
