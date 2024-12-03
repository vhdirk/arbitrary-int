use core::ops::{Add, Mul, Shl, Shr, Neg};

use crate::error::ParseAIntError;
use crate::traits::{Unsigned, Signed};
use crate::{AInt, Number};

macro_rules! aint_impl_num_traits {
    ($( $type:ty),+) => {
        $(
            impl<const BITS: usize> num_traits::Zero for AInt<$type, BITS>
            where

                Self: Number<Container=$type> + PartialEq<Self>,

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
                    *self = Self::ZERO;
                }
            }

            impl<const BITS: usize> num_traits::One for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn one() -> Self {
                    Self::ONE
                }
            }

            impl<const BITS: usize> num_traits::Bounded for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

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

            impl<const BITS: usize> num_traits::Euclid for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

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

            impl<const BITS: usize> num_traits::MulAdd for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                type Output = Self;

                fn mul_add(self, a: Self, b: Self) -> Self::Output {
                    <Self as Add>::add(<Self as Mul>::mul(self, a), b)
                }
            }

            impl<const BITS: usize> num_traits::CheckedAdd for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn checked_add(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_add(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::CheckedSub for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn checked_sub(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_sub(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::CheckedDiv for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn checked_div(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_div(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::CheckedMul for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn checked_mul(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_mul(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::CheckedNeg for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn checked_neg(&self) -> Option<Self> {
                    Self::checked_neg(*self)
                }
            }


            impl<const BITS: usize> num_traits::CheckedRem for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn checked_rem(&self, rhs: &Self) -> Option<Self> {
                    Self::checked_rem(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::CheckedShl for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn checked_shl(&self, n: u32) -> Option<Self> {
                    Self::checked_shl(*self, n)
                }
            }

            impl<const BITS: usize> num_traits::CheckedShr for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn checked_shr(&self, n: u32) -> Option<Self> {
                    Self::checked_shr(*self, n)
                }
            }

            impl<const BITS: usize> num_traits::CheckedEuclid for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

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

            impl<const BITS: usize> num_traits::SaturatingAdd for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn saturating_add(&self, rhs: &Self) -> Self {
                    Self::saturating_add(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::SaturatingSub for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn saturating_sub(&self, rhs: &Self) -> Self {
                    Self::saturating_sub(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::Saturating  for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn saturating_add(self, rhs: Self) -> Self {
                    <Self as num_traits::SaturatingAdd>::saturating_add(&self, &rhs)
                }

                #[inline]
                fn saturating_sub(self, rhs: Self) -> Self {
                    <Self as num_traits::SaturatingSub>::saturating_sub(&self, &rhs)
                }
            }

            impl<const BITS: usize> num_traits::SaturatingMul for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn saturating_mul(&self, rhs: &Self) -> Self {
                    Self::saturating_mul(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::WrappingAdd for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn wrapping_add(&self, rhs: &Self) -> Self {
                    Self::wrapping_add(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::WrappingSub for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn wrapping_sub(&self, rhs: &Self) -> Self {
                    Self::wrapping_sub(*self, *rhs)
                }
            }

            impl<const BITS: usize> num_traits::WrappingMul for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn wrapping_mul(&self, rhs: &Self) -> Self {
                    Self::wrapping_mul(*self, *rhs)
                }
            }


            impl<const BITS: usize> num_traits::WrappingShl for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn wrapping_shl(&self, n: u32) -> Self {
                    Self::wrapping_shl(*self, n)
                }
            }

            impl<const BITS: usize> num_traits::WrappingShr for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn wrapping_shr(&self, n: u32) -> Self {
                    Self::wrapping_shr(*self, n)
                }
            }

            impl<const BITS: usize> num_traits::Num for AInt<$type, BITS>
            where

                Self: Number<Container=$type> + PartialEq<Self>,

            {
                type FromStrRadixErr = ParseAIntError;

                #[inline]
                fn from_str_radix(s: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
                    <Self>::from_str_radix(s, radix)
                }
            }

            impl<const BITS: usize> num_traits::ToPrimitive for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

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

            impl<const BITS: usize> num_traits::NumCast for AInt<$type, BITS>
            where

                Self: Number<Container=$type>,

            {
                #[inline]
                fn from<F: num_traits::ToPrimitive>(n: F) -> Option<Self> {
                    n.to_u64()
                        .and_then(|v| v.try_into().ok())
                        .and_then(|v| Self::try_new(v).ok())
                }
            }

        )+
    };
}


aint_impl_num_traits!(u8, u16, u32, u64, u128);

macro_rules! aint_impl_num_traits_unsigned {
    ($( $type:ty),+) => {
        $(

            impl<const BITS: usize> num_traits::Unsigned for AInt<$type, BITS>
            where
                Self: Number<Container=$type> + PartialEq<Self> + Unsigned,


            {
            }
        )+
    };
}

aint_impl_num_traits_unsigned!(u8, u16, u32, u64, u128);


macro_rules! aint_impl_num_traits_signed {
    ($( $type:ty),+) => {
        $(

            impl<const BITS: usize> num_traits::Signed for AInt<$type, BITS>
            where
                Self: Number<Container=$type> + PartialEq<Self> + Neg<Output=Self> + Signed + num_traits::Num,


            {

                fn abs(&self) -> Self {
                    unsafe {
                        Self::new_unchecked(self.value().abs())
                    }
                }
                fn abs_sub(&self, other: &Self) -> Self {
                    unsafe {
                        Self::new_unchecked(self.value().abs_sub(&other.value()))
                    }
                }

                fn signum(&self) -> Self{
                    Self::signum(*self)
                }

                fn is_positive(&self) -> bool{
                    self.value().is_positive()
                }
                fn is_negative(&self) -> bool{
                    self.value().is_negative()
                }
            }
        )+
    };
}

aint_impl_num_traits_signed!(i8, i16, i32, i64, i128);


// impl<const BITS: usize> num_traits::Signed for AInt<$type, BITS>
// where
//     Self: Number<Container=$type> + PartialEq<Self> + PartialOrd<Self> + Neg + Signed,
//
//
// {
// }

// impl<const BITS: usize> num_traits::PrimInt for AInt<$type, BITS>
// where
//
//     Self: Number<Container = $type>
//         + Shl<u32, Output = Self>
//         + Shr<u32, Output = Self>
//         + num_traits::Pow<u32, Output = Self>,
//     <Self as Number>::SignedEquivalent: Shl<u32, Output=<Self as Number>::SignedEquivalent> + Shr<u32, Output=<Self as Number>::SignedEquivalent>,
//     <Self as Number>::UnsignedEquivalent: Shl<u32, Output=<Self as Number>::UnsignedEquivalent> + Shr<u32, Output=<Self as Number>::UnsignedEquivalent>,
// {
//     fn count_ones(self) -> u32 {
//         <Self>::count_ones(self)
//     }

//     fn count_zeros(self) -> u32 {
//         <Self>::count_zeros(self)
//     }

//     fn leading_zeros(self) -> u32 {
//         <Self>::leading_zeros(self)
//     }

//     fn trailing_zeros(self) -> u32 {
//         <Self>::trailing_zeros(self)
//     }

//     fn rotate_left(self, n: u32) -> Self {
//         <Self>::rotate_left(self, n)
//     }

//     fn rotate_right(self, n: u32) -> Self {
//         <Self>::rotate_right(self, n)
//     }

//     fn signed_shl(self, n: u32) -> Self {
//         let v = self.as_signed().shl(n);
//         if Self::SIGNED {
//             v.as_signed().into()
//         } else {
//             v.as_unsigned()
//         }
//     }

//     fn signed_shr(self, n: u32) -> Self {
//         self.as_signed().shr(n);
//         if Self::SIGNED {
//             v.as_signed()
//         } else {
//             v.as_unsigned()
//         }
//     }

//     fn unsigned_shl(self, n: u32) -> Self {
//         self.as_unsigned().shl(n);
//         if Self::SIGNED {
//             v.as_signed()
//         } else {
//             v.as_unsigned()
//         }
//     }

//     fn unsigned_shr(self, n: u32) -> Self {
//         self.as_unsigned().shr(n);
//         if Self::SIGNED {
//             v.as_signed()
//         } else {
//             v.as_unsigned()
//         }
//     }

//     fn swap_bytes(self) -> Self {
//         <Self>::swap_bytes(self)
//     }

//     fn from_be(x: Self) -> Self {
//         <Self>::from_be(x)
//     }

//     fn from_le(x: Self) -> Self {
//         <Self>::from_le(x)
//     }

//     fn to_be(self) -> Self {
//         <Self>::to_be(self)
//     }

//     fn to_le(self) -> Self {
//         <Self>::to_le(self)
//     }

//     fn pow(self, exp: u32) -> Self {
//         <Self as num_traits::Pow<u32>>::pow(self, exp)
//     }
// }






// // TODO: leaving this here as a reminder that we need https://github.com/rust-lang/rust/issues/76560
// // macro_rules! bytes_impl {
// //     ($bytes: expr ) => {
// // impl<T, const BITS: usize> ToBytes for AInt<T, BITS>
// // where
// //     T: PrimInt + Into<u8>,
// //     Self: ConstBytes,
// //     Assert<{<AInt<T, BITS> as ConstBytes>::BYTES == 1 }>: IsTrue

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

//         impl ToBytes for AInt<$type, $bits>
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

//         impl FromBytes for AInt<$type, $bits>
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
    use crate::aliases::*;

    #[test]
    fn calculation_with_number_trait() {
        fn increment_by_1<T>(foo: T) -> T
        where
            T: Number + num_traits::WrappingAdd,
            <T as Number>::Container: From<u8>,
        {
            foo.wrapping_add(&T::new(1.into()))
        }

        fn increment_by_512<T>(
            foo: T,
        ) -> Result<T, <<T as Number>::Container as TryFrom<u32>>::Error>
        where
            T: Number + num_traits::WrappingAdd,
            <T as Number>::Container: TryFrom<u32>,
            <<T as Number>::Container as TryFrom<u32>>::Error: core::fmt::Debug,

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

    #[test]
    fn add_wrapping() {
        let v1 = u7::new(120);
        let v2 = u7::new(10);
        let v3 = num_traits::WrappingAdd::wrapping_add(&v1, &v2);
        assert_eq!(v3, u7::new(2));
    }

    #[cfg(feature = "num-traits")]
    #[test]
    fn sub_wrapping() {
        let v1 = u7::new(15);
        let v2 = u7::new(20);
        let v3 = num_traits::WrappingSub::wrapping_sub(&v1, &v2);
        assert_eq!(v3, u7::new(123));
    }

    #[cfg(feature = "num-traits")]
    #[test]
    fn bounded() {
        assert_eq!(u7::MAX, u7::max_value());

        assert_eq!(u119::MAX, u119::max_value());
        assert_eq!(u7::new(0), u7::min_value());

        assert_eq!(u119::new(0), u119::min_value());
    }

}
