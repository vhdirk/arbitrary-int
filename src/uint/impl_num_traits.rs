use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
// use std::str::FromStr;

// use crate::{FromStrError, ParseIntXError, ValueOutOfRange};
// use crate::{IntX, UInt};
// use seq_macro::seq;

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use num_traits::{
    bounds::UpperBounded,
    ops::overflowing::{OverflowingAdd, OverflowingMul, OverflowingSub},
    AsPrimitive, Bounded, CheckedAdd, CheckedDiv, CheckedEuclid, CheckedMul, CheckedNeg,
    CheckedRem, CheckedShl, CheckedShr, CheckedSub, ConstOne, ConstZero, Euclid, Float, FloatConst,
    FloatErrorKind, FromBytes, FromPrimitive, Inv, MulAdd, MulAddAssign, Num, NumAssign,
    NumAssignOps, NumAssignRef, NumCast, NumOps, NumRef, One, ParseFloatError, Pow, PrimInt,
    RefNum, Saturating, SaturatingAdd, SaturatingMul, SaturatingSub, Signed, ToBytes, ToPrimitive,
    Unsigned, WrappingAdd, WrappingMul, WrappingNeg, WrappingShl, WrappingShr, WrappingSub, Zero,
};

use crate::error::ParseNumberError;
use crate::uint::UInt;
use crate::Number;

macro_rules! uint_impl_num_traits {
    ($( $type:ty),+) => {
        $(
            impl<const BITS: usize> Unsigned for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
            }

            impl<const BITS: usize> Zero for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn zero() -> Self {
                    Self::ZERO
                }

                #[inline]
                fn is_zero(&self) -> bool {
                    *self == Self::ZERO
                }

                fn set_zero(&mut self) {
                    *self = Zero::zero();
                }
            }

            impl<const BITS: usize> One for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn one() -> Self {
                    Self::ONE
                }
            }

            impl<const BITS: usize> Bounded for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn min_value() -> Self {
                    Self::MIN
                }

                #[inline]
                fn max_value() -> Self {
                    Self::MAX
                }
            }

            impl<const BITS: usize> Euclid for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn div_euclid(&self, rhs: &Self) -> Self {
                    Self::div_euclid(*self, *rhs)
                }

                #[inline]
                fn rem_euclid(&self, rhs: &Self) -> Self {
                    Self::rem_euclid(*self, *rhs)
                }
            }

            impl<const BITS: usize> MulAdd for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                type Output = Self;

                fn mul_add(self, a: Self, b: Self) -> Self::Output {
                    <Self as Add>::add(<Self as Mul>::mul(self, a), b)
                }
            }

            impl<const BITS: usize> CheckedAdd for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_add(*self, *rhs)
                }
            }

            impl<const BITS: usize> CheckedSub for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_sub(*self, *rhs)
                }
            }

            impl<const BITS: usize> CheckedDiv for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn checked_div(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_div(*self, *rhs)
                }
            }

            impl<const BITS: usize> CheckedMul for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_mul(*self, *rhs)
                }
            }

// // pub use num_traits::CheckedNeg;


            impl<const BITS: usize> CheckedRem for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn checked_rem(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_rem(*self, *rhs)
                }
            }

            impl<const BITS: usize> CheckedShl for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn checked_shl(&self, n: u32) -> Option<Self> {
                    Self::checked_shl(*self, n)
                }
            }

            impl<const BITS: usize> CheckedShr for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn checked_shr(&self, n: u32) -> Option<Self> {
                    Self::checked_shr(*self, n)
                }
            }

            impl<const BITS: usize> CheckedEuclid for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn checked_div_euclid(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_div_euclid(*self, *rhs)
                }

                #[inline]
                fn checked_rem_euclid(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_rem_euclid(*self, *rhs)
                }
            }

            impl<const BITS: usize> SaturatingAdd for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn saturating_add(&self, rhs: &Self) -> Self {
                    Self::saturating_add(*self, *rhs)
                }
            }

            impl<const BITS: usize> SaturatingSub for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn saturating_sub(&self, rhs: &Self) -> Self {
                    Self::saturating_sub(*self, *rhs)
                }
            }

            impl<const BITS: usize> Saturating  for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn saturating_add(self, rhs: Self) -> Self {
                    <Self as SaturatingAdd>::saturating_add(&self, &rhs)
                }

                #[inline]
                fn saturating_sub(self, rhs: Self) -> Self {
                    <Self as SaturatingSub>::saturating_sub(&self, &rhs)
                }
            }

            impl<const BITS: usize> SaturatingMul for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn saturating_mul(&self, rhs: &Self) -> Self {
                    Self::saturating_mul(*self, *rhs)
                }
            }

            impl<const BITS: usize> WrappingAdd for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn wrapping_add(&self, rhs: &Self) -> Self {
                    Self::wrapping_add(*self, *rhs)
                }
            }

            impl<const BITS: usize> WrappingSub for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn wrapping_sub(&self, rhs: &Self) -> Self {
                    Self::wrapping_sub(*self, *rhs)
                }
            }

            impl<const BITS: usize> WrappingMul for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn wrapping_mul(&self, rhs: &Self) -> Self {
                    Self::wrapping_mul(*self, *rhs)
                }
            }


            impl<const BITS: usize> WrappingShl for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn wrapping_shl(&self, n: u32) -> Self {
                    Self::wrapping_shl(*self, n)
                }
            }

            impl<const BITS: usize> WrappingShr for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn wrapping_shr(&self, n: u32) -> Self {
                    Self::wrapping_shr(*self, n)
                }
            }

            impl<const BITS: usize> Num for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                type FromStrRadixErr = ParseNumberError;

                #[inline]
                fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                    <Self>::from_str_radix(s, radix)
                }
            }

            impl<const BITS: usize> ToPrimitive for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn to_u64(&self) -> Option<u64> {
                    self.value.to_u64()
                }

                #[inline]
                fn to_i64(&self) -> Option<i64> {
                    self.value.to_i64()
                }
            }

            impl<const BITS: usize> NumCast for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>,
            {
                #[inline]
                fn from<F: ToPrimitive>(n: F) -> Option<Self> {
                    n.to_u64()
                        .and_then(|v| v.try_into().ok())
                        .and_then(|v| Self::try_new(v).ok())
                }
            }


            impl<const BITS: usize> PrimInt for UInt<$type, BITS>
            where
                Self: Number<UnderlyingType = $type>
                    + Shl<u32, Output = Self>
                    + Shr<u32, Output = Self>
                    + Pow<u32, Output = Self>,
            {
                fn count_ones(self) -> u32 {
                    <Self>::count_ones(self)
                }

                fn count_zeros(self) -> u32 {
                    <Self>::count_zeros(self)
                }

                fn leading_zeros(self) -> u32 {
                    <Self>::leading_zeros(self)
                }

                fn trailing_zeros(self) -> u32 {
                    <Self>::trailing_zeros(self)
                }

                fn rotate_left(self, n: u32) -> Self {
                    <Self>::rotate_left(self, n)
                }

                fn rotate_right(self, n: u32) -> Self {
                    <Self>::rotate_right(self, n)
                }

                fn signed_shl(self, n: u32) -> Self {
                    todo!()
                }

                fn signed_shr(self, n: u32) -> Self {
                    todo!()
                }

                fn unsigned_shl(self, n: u32) -> Self {
                    <Self as Shl<u32>>::shl(self, n)
                }

                fn unsigned_shr(self, n: u32) -> Self {
                    <Self as Shr<u32>>::shr(self, n)
                }

                fn swap_bytes(self) -> Self {
                    <Self>::swap_bytes(self)
                }

                fn from_be(x: Self) -> Self {
                    <Self>::from_be(x)
                }

                fn from_le(x: Self) -> Self {
                    <Self>::from_le(x)
                }

                fn to_be(self) -> Self {
                    <Self>::to_be(self)
                }

                fn to_le(self) -> Self {
                    <Self>::to_le(self)
                }

                fn pow(self, exp: u32) -> Self {
                    <Self as Pow<u32>>::pow(self, exp)
                }
            }






        )+
    };
}

uint_impl_num_traits!(u8, u16, u32, u64, u128);

// // TODO: leaving this here as a reminder that we need https://github.com/rust-lang/rust/issues/76560
// // macro_rules! bytes_impl {
// //     ($bytes: expr ) => {
// // impl<T, const BITS: usize> ToBytes for UInt<T, BITS>
// // where
// //     T: PrimInt + Into<u8>,
// //     Self: ConstBytes,
// //     Assert<{<UInt<T, BITS> as ConstBytes>::BYTES == 1 }>: IsTrue

// // {
// //     type Bytes = [u8; $bytes];

// //     fn to_be_bytes(&self) -> Self::Bytes {
// //         let _ = CompileTimeAssert::<{BITS}, {$bytes*8}>::SMALLER_OR_EQUAL;

// //         seq!(INDEX in 0..$bytes {
// //             [
// //                 #(
// //                     if BITS - ((INDEX + 1) << 3) > 0 {
// //                         (self.value >> (BITS - (INDEX + 1) * 8)).into()
// //                     } else {
// //                         // Only mask the relevant part for the last few bits
// //                         (self.value << ((INDEX + 1) * 8 - BITS)).into()
// //                     } ,
// //                 )*
// //             ]
// //         })
// //     }

// //     fn to_le_bytes(&self) -> Self::Bytes {
// //         seq!(INDEX in 0..$bytes {
// //             [
// //                 #( (self.value >> (INDEX * 8)).into(), )*
// //             ]
// //         })
// //     }
// // }

// // };
// // }

// // seq!(BYTES in 1..=2 {
// //     #(
// //         bytes_impl!(BYTES);
// //     )*
// // });

// // Need to explicitely implement this since generic `Self` types are currently not permitted in anonymous constants

// macro_rules! bytes_operation_impl {
//     ($type:ident, $bytes: expr, $bits:expr, $min_bits:expr, $max_bits:expr ) => {

//         impl ToBytes for UInt<$type, $bits>
//         where
//             Self: ConstBytes
//         {
//             type Bytes = [u8; $bytes];

//             fn to_be_bytes(&self) -> Self::Bytes {
//                 seq!(INDEX in 0..$bytes {
//                     [
//                         #(
//                             if $bits - ((INDEX + 1) << 3) > 0 {
//                                 (self.value >> ($bits - (INDEX + 1) * 8)) as u8
//                             } else {
//                                 // Only mask the relevant part for the last few bits
//                                 (self.value << ((INDEX + 1) * 8 - $bits)) as u8
//                             } ,
//                         )*
//                     ]
//                 })
//             }

//             fn to_le_bytes(&self) -> Self::Bytes {
//                 seq!(INDEX in 0..$bytes {
//                     [
//                         #( (self.value >> (INDEX * 8)) as u8, )*
//                     ]
//                 })
//             }
//         }

//         impl FromBytes for UInt<$type, $bits>
//         where
//             Self: ConstBytes
//         {
//             type Bytes = [u8; $bytes];

//             fn from_le_bytes(from: &Self::Bytes) -> Self {
//                 let value = seq!(INDEX in 0usize..$bytes {
//                     0 #( | ((from[INDEX] as $type) * 8) )*
//                 });
//                 Self { value }
//             }

//             fn from_be_bytes(from: &Self::Bytes) -> Self {
//                 let value = seq!(INDEX in 0..$bytes {

//                     0 #( | if $bits > (8 * (INDEX + 1)) {
//                         (from[INDEX] as $type) << ($bits - 8 * (INDEX + 1))
//                     } else {
//                         // For the last partial byte, shift just enough to align the remaining bits
//                         (from[INDEX] as $type) << (8 * INDEX)
//                     } )*
//                 });

//                 Self { value }
//             }

//         }
//     };

//     ($type:ident, $bytes: expr, $min_bits:expr, $max_bits:expr) => {
//         seq!(BITS in $min_bits..=$max_bits {
//             #(
//                 bytes_operation_impl!($type, $bytes, BITS, $min_bits, $max_bits);
//             )*
//         });
//     };
// }

// // bytes_operation_impl!(u128, 16, 121, 128);

// // // pub use crate::ops::bytes::FromBytes;
// // // pub use crate::ops::bytes::ToBytes;
// //         // pub use Inv;

// //         // pub use num_traits::MulAdd;
// //         // pub use num_traits::MulAddAssign;

// // AsPrimitive, CheckedNeg,  Float, FloatConst, FloatErrorKind, FromPrimitive, Inv, MulAdd, MulAddAssign, NumAssign,
// // NumAssignOps, NumAssignRef, NumOps, NumRef, ParseFloatError, Pow, RefNum, Signed, Unsigned, WrappingNeg
// //     };
// // }


#[cfg(test)]
mod tests {
    use super::*;
    use crate::uint::aliases::*;

    #[test]
    fn calculation_with_number_trait() {
        fn increment_by_1<T>(foo: T) -> T
        where
            T: Number + num_traits::WrappingAdd,
            <T as Number>::UnderlyingType: From<u8>,
        {
            foo.wrapping_add(&T::new(1.into()))
        }

        fn increment_by_512<T>(
            foo: T,
        ) -> Result<T, <<T as Number>::UnderlyingType as TryFrom<u32>>::Error>
        where
            T: Number + num_traits::WrappingAdd,
            <T as Number>::UnderlyingType: TryFrom<u32>,
            <<T as Number>::UnderlyingType as TryFrom<u32>>::Error: core::fmt::Debug,

        {
            Ok(foo.wrapping_add(&T::new(512u32.try_into()?)))
        }

        assert_eq!(increment_by_1(0u16), 1u16);
        assert_eq!(increment_by_1(u7::new(3)), u7::new(4));
        assert_eq!(increment_by_1(u15::new(3)), u15::new(4));

        assert_eq!(increment_by_512(0u16), Ok(512u16));
        assert!(increment_by_512(u7::new(3)).is_err());
        assert_eq!(increment_by_512(u15::new(3)), Ok(u15::new(515)));
    }
}
