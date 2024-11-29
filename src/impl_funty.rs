use core::{
    convert::{TryFrom, TryInto},
    fmt::{Binary, Debug, Display, LowerHex, Octal, UpperHex},
    hash::Hash,
    iter::{Product, Sum},
    num::{ParseIntError, Wrapping},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
    str::FromStr,
};
use seq_macro::seq;

use crate::{AInt, Number};
use crate::traits::{BitsSpec, AIntContainer};

macro_rules! aint_impl_funty {
    ($($type:ident),+) => {
        $(
            impl<Bits> funty::Fundamental for AInt<$type, Bits>
            where
                $type: AIntContainer + Debug,
                Bits: BitsSpec,
                <$type as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
                Self: Number<Container = $type, Bits=Bits>
                    + PartialEq<Self>
                    + BitXor
                    + BitXorAssign
                    + Display
                    + Debug
                    + FromStr
                    + Default
                    + Unpin
                    + Sync
                    + Send
                    + 'static,
                Wrapping<$type>: Add<Wrapping<$type>, Output = Wrapping<$type>>,
            {
                fn as_bool(self) -> bool {
                    self.value() > 0
                }

                #[inline]
                fn as_char(self) -> Option<char> {
                    core::char::from_u32(self.value() as u32)
                }

                #[inline]
                fn as_i8(self) -> i8 {
                    self.value() as i8
                }

                #[inline]
                fn as_i16(self) -> i16 {
                    self.value() as i16
                }

                #[inline]
                fn as_i32(self) -> i32 {
                    self.value() as i32
                }

                #[inline]
                fn as_i64(self) -> i64 {
                    self.value() as i64
                }

                #[inline]
                fn as_i128(self) -> i128 {
                    self.value() as i128
                }

                #[inline]
                fn as_isize(self) -> isize {
                    self.value() as isize
                }

                #[inline]
                fn as_u8(self) -> u8 {
                    self.value() as u8
                }

                #[inline]
                fn as_u16(self) -> u16 {
                    self.value() as u16
                }

                #[inline]
                fn as_u32(self) -> u32 {
                    self.value() as u32
                }

                #[inline]
                fn as_u64(self) -> u64 {
                    self.value() as u64
                }

                #[inline]
                fn as_u128(self) ->u128 {
                    self.value() as u128
                }

                #[inline]
                fn as_usize(self) -> usize {
                    self.value() as usize
                }

                #[inline]
                fn as_f32(self) -> f32 {
                    self.value() as u32 as f32
                }

                #[inline]
                fn as_f64(self) -> f64 {
                    self.value() as u64 as f64
                }
            }


            impl<Bits> funty::Integral for AInt<$type, Bits>
            where
                $type: AIntContainer + Debug,
                Bits: BitsSpec,
                <$type as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
                Self: Number<Container = $type, Bits=Bits>
                    + funty::Numeric
                    + Hash
                    + Eq
                    + Ord
                    + Binary
                    + LowerHex
                    + UpperHex
                    + Octal
                    + BitAnd<Self, Output = Self>
                    + for<'a> BitAnd<&'a Self, Output = Self>
                    + BitAndAssign<Self>
                    + for<'a> BitAndAssign<&'a Self>
                    + BitOr<Self, Output = Self>
                    + for<'a> BitOr<&'a Self, Output = Self>
                    + BitOrAssign<Self>
                    + for<'a> BitOrAssign<&'a Self>
                    + BitXor<Self, Output = Self>
                    + for<'a> BitXor<&'a Self, Output = Self>
                    + BitXorAssign<Self>
                    + for<'a> BitXorAssign<&'a Self>
                    + Not<Output = Self>
                    + TryFrom<i8>
                    + TryFrom<u8>
                    + TryFrom<i16>
                    + TryFrom<u16>
                    + TryFrom<i32>
                    + TryFrom<u32>
                    + TryFrom<i64>
                    + TryFrom<u64>
                    + TryFrom<i128>
                    + TryFrom<u128>
                    + TryFrom<isize>
                    + TryFrom<usize>
                    + TryInto<i8>
                    + TryInto<u8>
                    + TryInto<i16>
                    + TryInto<u16>
                    + TryInto<i32>
                    + TryInto<u32>
                    + TryInto<i64>
                    + TryInto<u64>
                    + TryInto<i128>
                    + TryInto<u128>
                    + TryInto<isize>
                    + TryInto<usize>
                    + Shl<Self, Output = Self>
                    + for<'a> Shl<&'a Self, Output = Self>
                    + ShlAssign<Self>
                    + for<'a> ShlAssign<&'a Self>
                    + Shr<Self, Output = Self>
                    + for<'a> Shr<&'a Self, Output = Self>
                    + ShrAssign<Self>
                    + for<'a> ShrAssign<&'a Self>
                    + Shl<i8, Output = Self>
                    + for<'a> Shl<&'a i8, Output = Self>
                    + ShlAssign<i8>
                    + for<'a> ShlAssign<&'a i8>
                    + Shr<i8, Output = Self>
                    + for<'a> Shr<&'a i8, Output = Self>
                    + ShrAssign<i8>
                    + for<'a> ShrAssign<&'a i8>
                    + Shl<u8, Output = Self>
                    + for<'a> Shl<&'a u8, Output = Self>
                    + ShlAssign<u8>
                    + for<'a> ShlAssign<&'a u8>
                    + Shr<u8, Output = Self>
                    + for<'a> Shr<&'a u8, Output = Self>
                    + ShrAssign<u8>
                    + for<'a> ShrAssign<&'a u8>
                    + Shl<i16, Output = Self>
                    + for<'a> Shl<&'a i16, Output = Self>
                    + ShlAssign<i16>
                    + for<'a> ShlAssign<&'a i16>
                    + Shr<i16, Output = Self>
                    + for<'a> Shr<&'a i16, Output = Self>
                    + ShrAssign<i16>
                    + for<'a> ShrAssign<&'a i16>
                    + Shl<u16, Output = Self>
                    + for<'a> Shl<&'a u16, Output = Self>
                    + ShlAssign<u16>
                    + for<'a> ShlAssign<&'a u16>
                    + Shr<u16, Output = Self>
                    + for<'a> Shr<&'a u16, Output = Self>
                    + ShrAssign<u16>
                    + for<'a> ShrAssign<&'a u16>
                    + Shl<i32, Output = Self>
                    + for<'a> Shl<&'a i32, Output = Self>
                    + ShlAssign<i32>
                    + for<'a> ShlAssign<&'a i32>
                    + Shr<i32, Output = Self>
                    + for<'a> Shr<&'a i32, Output = Self>
                    + ShrAssign<i32>
                    + for<'a> ShrAssign<&'a i32>
                    + Shl<u32, Output = Self>
                    + for<'a> Shl<&'a u32, Output = Self>
                    + ShlAssign<u32>
                    + for<'a> ShlAssign<&'a u32>
                    + Shr<u32, Output = Self>
                    + for<'a> Shr<&'a u32, Output = Self>
                    + ShrAssign<u32>
                    + for<'a> ShrAssign<&'a u32>
                    + Shl<i64, Output = Self>
                    + for<'a> Shl<&'a i64, Output = Self>
                    + ShlAssign<i64>
                    + for<'a> ShlAssign<&'a i64>
                    + Shr<i64, Output = Self>
                    + for<'a> Shr<&'a i64, Output = Self>
                    + ShrAssign<i64>
                    + for<'a> ShrAssign<&'a i64>
                    + Shl<u64, Output = Self>
                    + for<'a> Shl<&'a u64, Output = Self>
                    + ShlAssign<u64>
                    + for<'a> ShlAssign<&'a u64>
                    + Shr<u64, Output = Self>
                    + for<'a> Shr<&'a u64, Output = Self>
                    + ShrAssign<u64>
                    + for<'a> ShrAssign<&'a u64>
                    + Shl<i128, Output = Self>
                    + for<'a> Shl<&'a i128, Output = Self>
                    + ShlAssign<i128>
                    + for<'a> ShlAssign<&'a i128>
                    + Shr<i128, Output = Self>
                    + for<'a> Shr<&'a i128, Output = Self>
                    + ShrAssign<i128>
                    + for<'a> ShrAssign<&'a i128>
                    + Shl<u128, Output = Self>
                    + for<'a> Shl<&'a u128, Output = Self>
                    + ShlAssign<u128>
                    + for<'a> ShlAssign<&'a u128>
                    + Shr<u128, Output = Self>
                    + for<'a> Shr<&'a u128, Output = Self>
                    + ShrAssign<u128>
                    + for<'a> ShrAssign<&'a u128>
                    + Shl<isize, Output = Self>
                    + for<'a> Shl<&'a isize, Output = Self>
                    + ShlAssign<isize>
                    + for<'a> ShlAssign<&'a isize>
                    + Shr<isize, Output = Self>
                    + for<'a> Shr<&'a isize, Output = Self>
                    + ShrAssign<isize>
                    + for<'a> ShrAssign<&'a isize>
                    + Shl<usize, Output = Self>
                    + for<'a> Shl<&'a usize, Output = Self>
                    + ShlAssign<usize>
                    + for<'a> ShlAssign<&'a usize>
                    + Shr<usize, Output = Self>
                    + for<'a> Shr<&'a usize, Output = Self>
                    + ShrAssign<usize>
                    + for<'a> ShrAssign<&'a usize>,
            {
                const ZERO: Self = <Self>::ZERO;

                const ONE: Self = <Self>::ONE;

                const MIN: Self = <Self>::MIN;

                const MAX: Self = <Self>::MAX;

                const BITS: u32 = <Bits as typenum::Unsigned>::U32;

                #[inline]
                fn min_value() -> Self {
                    <Self>::MIN
                }

                #[inline]
                fn max_value() -> Self {
                    <Self>::MAX
                }

                #[inline]
                fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                    <Self>::from_str_radix(src, radix).map_err(|e| e.into())
                }

                #[inline]
                fn count_ones(self) -> u32 {
                    <Self>::count_ones(self)
                }

                #[inline]
                fn count_zeros(self) -> u32 {
                    <Self>::count_zeros(self)
                }

                #[inline]
                fn leading_zeros(self) -> u32 {
                    <Self>::leading_zeros(self)
                }

                #[inline]
                fn trailing_zeros(self) -> u32 {
                    <Self>::trailing_zeros(self)
                }

                #[inline]
                fn leading_ones(self) -> u32 {
                    <Self>::leading_ones(self)
                }

                #[inline]
                fn trailing_ones(self) -> u32 {
                    <Self>::trailing_ones(self)
                }

                #[inline]
                fn rotate_left(self, n: u32) -> Self {
                    <Self>::rotate_left(self, n)
                }

                #[inline]
                fn rotate_right(self, n: u32) -> Self {
                    <Self>::rotate_right(self, n)
                }

                #[inline]
                fn swap_bytes(self) -> Self {
                    <Self>::swap_bytes(self)
                }

                #[inline]
                fn reverse_bits(self) -> Self {
                    <Self>::reverse_bits(self)
                }

                #[inline]
                fn from_be(self) -> Self {
                    <Self>::from_be(self)
                }

                #[inline]
                fn from_le(self) -> Self {
                    <Self>::from_le(self)
                }

                #[inline]
                fn to_be(self) -> Self {
                    <Self>::to_be(self)
                }

                #[inline]
                fn to_le(self) -> Self {
                    <Self>::to_le(self)
                }

                #[inline]
                fn checked_add(self, rhs: Self) -> Option<Self> {
                    <Self>::checked_add(self, rhs)
                }

                #[inline]
                fn checked_sub(self, rhs: Self) -> Option<Self> {
                    <Self>::checked_sub(self, rhs)
                }

                #[inline]
                fn checked_mul(self, rhs: Self) -> Option<Self> {
                    <Self>::checked_mul(self, rhs)
                }

                #[inline]
                fn checked_div(self, rhs: Self) -> Option<Self> {
                    <Self>::checked_div(self, rhs)
                }

                #[inline]
                fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                    <Self>::checked_div_euclid(self, rhs)
                }

                #[inline]
                fn checked_rem(self, rhs: Self) -> Option<Self> {
                    <Self>::checked_rem(self, rhs)
                }

                #[inline]
                fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                    <Self>::checked_rem_euclid(self, rhs)
                }

                #[inline]
                fn checked_neg(self) -> Option<Self> {
                    <Self>::checked_neg(self)
                }

                #[inline]
                fn checked_shl(self, n: u32) -> Option<Self> {
                    <Self>::checked_shl(self, n)
                }

                #[inline]
                fn checked_shr(self, n: u32) -> Option<Self> {
                    <Self>::checked_shr(self, n)
                }

                #[inline]
                fn checked_pow(self, exp: u32) -> Option<Self> {
                    <Self>::checked_pow(self, exp)
                }

                #[inline]
                fn saturating_add(self, rhs: Self) -> Self {
                    <Self>::saturating_add(self, rhs)
                }

                #[inline]
                fn saturating_sub(self, rhs: Self) -> Self {
                    <Self>::saturating_sub(self, rhs)
                }

                #[inline]
                fn saturating_mul(self, rhs: Self) -> Self {
                    <Self>::saturating_mul(self, rhs)
                }

                #[inline]
                fn saturating_pow(self, exp: u32) -> Self {
                    <Self>::saturating_pow(self, exp)
                }

                #[inline]
                fn wrapping_add(self, rhs: Self) -> Self {
                    <Self>::wrapping_add(self, rhs)
                }

                #[inline]
                fn wrapping_sub(self, rhs: Self) -> Self {
                    <Self>::wrapping_sub(self, rhs)
                }

                #[inline]
                fn wrapping_mul(self, rhs: Self) -> Self {
                    <Self>::wrapping_mul(self, rhs)
                }

                #[inline]
                fn wrapping_div(self, rhs: Self) -> Self {
                    <Self>::wrapping_div(self, rhs)
                }

                fn wrapping_div_euclid(self, rhs: Self) -> Self {
                    <Self>::wrapping_div_euclid(self, rhs)
                }

                fn wrapping_rem(self, rhs: Self) -> Self {
                    <Self>::wrapping_rem(self, rhs)
                }

                fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                    <Self>::wrapping_rem_euclid(self, rhs)
                }

                fn wrapping_neg(self) -> Self {
                    <Self>::wrapping_neg(self)
                }

                fn wrapping_shl(self, n: u32) -> Self {
                    <Self>::wrapping_shl(self, n)
                }

                fn wrapping_shr(self, n: u32) -> Self {
                    <Self>::wrapping_shr(self, n)
                }

                fn wrapping_pow(self, exp: u32) -> Self {
                    <Self>::wrapping_pow(self, exp)
                }

                fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                    <Self>::overflowing_add(self, rhs)
                }

                fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                    <Self>::overflowing_sub(self, rhs)
                }

                fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                    <Self>::overflowing_mul(self, rhs)
                }

                fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                    <Self>::overflowing_div(self, rhs)
                }

                fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                    <Self>::overflowing_div_euclid(self, rhs)
                }

                fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                    <Self>::overflowing_rem(self, rhs)
                }

                fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                    <Self>::overflowing_rem_euclid(self, rhs)
                }

                fn overflowing_neg(self) -> (Self, bool) {
                    <Self>::overflowing_neg(self)
                }

                fn overflowing_shl(self, n: u32) -> (Self, bool) {
                    <Self>::overflowing_shl(self, n)
                }

                fn overflowing_shr(self, n: u32) -> (Self, bool) {
                    <Self>::overflowing_shl(self, n)
                }

                fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                    <Self>::overflowing_pow(self, exp)
                }

                fn pow(self, exp: u32) -> Self {
                    <Self>::pow(self, exp)
                }

                fn div_euclid(self, rhs: Self) -> Self {
                    <Self>::div_euclid(self, rhs)
                }

                fn rem_euclid(self, rhs: Self) -> Self {
                    <Self>::rem_euclid(self, rhs)
                }
            }
        )+
    };
}

aint_impl_funty!(u8, u16, u32, u64, u128);
aint_impl_funty!(i8, i16, i32, i64, i128);


macro_rules! bytes_operation_impl {
    ($type:ident, $bytes: expr, $bytes_type: ident) => {

        impl<Bits> funty::Numeric for AInt<$type, Bits>
        where
            Bits: BitsSpec,
            <$type as Number>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
            Self: Number<Container = $type, Bits=Bits, Bytes=typenum::$bytes_type>
                + funty::Fundamental
                + Product<Self>
                + for<'a> Product<&'a Self>
                + Sum<Self>
                + for<'a> Sum<&'a Self>
                //  numeric ops
                + Add<Self, Output = Self>
                + for<'a> Add<&'a Self, Output = Self>
                + AddAssign<Self>
                + for<'a> AddAssign<&'a Self>
                + Sub<Self, Output = Self>
                + for<'a> Sub<&'a Self, Output = Self>
                + SubAssign<Self>
                + for<'a> SubAssign<&'a Self>
                + Mul<Self, Output = Self>
                + for<'a> Mul<&'a Self, Output = Self>
                + MulAssign<Self>
                + for<'a> MulAssign<&'a Self>
                + Div<Self, Output = Self>
                + for<'a> Div<&'a Self, Output = Self>
                + DivAssign<Self>
                + for<'a> DivAssign<&'a Self>
                + Rem<Self, Output = Self>
                + for<'a> Rem<&'a Self, Output = Self>
                + RemAssign<Self>
                + for<'a> RemAssign<&'a Self>,
            Wrapping<$type>: Add<Wrapping<$type>, Output = Wrapping<$type>>,
        {
            type Bytes = [u8; $bytes];

            fn to_be_bytes(self) -> Self::Bytes {
                <Self>::to_be_bytes(self)
            }

            fn to_le_bytes(self) -> Self::Bytes {
                <Self>::to_le_bytes(self)
            }

            #[inline]
            fn to_ne_bytes(self) -> Self::Bytes {
                <Self>::to_ne_bytes(self)
            }

            fn from_le_bytes(from: Self::Bytes) -> Self {
                <Self>::from_le_bytes(from)
            }

            fn from_be_bytes(from: Self::Bytes) -> Self {
                <Self>::from_be_bytes(from)
            }

            #[inline]
            fn from_ne_bytes(from: Self::Bytes) -> Self {
                <Self>::from_ne_bytes(from)
            }

        }
    };
}


bytes_operation_impl!(u8, 1, U1);
bytes_operation_impl!(i8, 1, U1);

seq!(BYTES in 1..=2 {
    #(
        bytes_operation_impl!(u16, BYTES, U~BYTES);
        bytes_operation_impl!(i16, BYTES, U~BYTES);
    )*
});

seq!(BYTES in 1..=4 {
    #(
        bytes_operation_impl!(u32, BYTES, U~BYTES);
        bytes_operation_impl!(i32, BYTES, U~BYTES);
    )*
});

seq!(BYTES in 1..=8 {
    #(
        bytes_operation_impl!(u64, BYTES, U~BYTES);
        bytes_operation_impl!(i64, BYTES, U~BYTES);
    )*
});

seq!(BYTES in 1..=16 {
    #(
        bytes_operation_impl!(u128, BYTES, U~BYTES);
        bytes_operation_impl!(i128, BYTES, U~BYTES);
    )*
});
