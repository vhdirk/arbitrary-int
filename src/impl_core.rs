use core::fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex};
use core::hash::{Hash, Hasher};

use core::iter::{Product, Sum};
use core::str::FromStr;

use core::num::ParseIntError;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign, Neg,
};

use crate::traits::BitsSpec;
use crate::{AIntErrorKind, Number, ParseAIntError};

use crate::{AInt, AIntContainer};

impl<T, Bits> Hash for AInt<T, Bits>
where
    T: AIntContainer + Hash,
    Self: Number<Container = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T, Bits> Add for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Add<Output = T> + Not<Output = T> + BitAnd<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let sum = self.value + rhs.value;

        #[cfg(debug_assertions)]
        if (sum & !Self::MASK) != Self::ZERO.value {
            panic!("attempt to add with overflow");
        }
        Self::new_wrapping(sum)
    }
}

impl<T, Bits> AddAssign for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer
        + Add<Output = T>
        + Not<Output = T>
        + BitAnd<Output = T>
        + AddAssign
        + BitAndAssign,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.value += rhs.value;
        #[cfg(debug_assertions)]
        if (self.value & !Self::MASK) != Self::ZERO.value {
            panic!("attempt to add with overflow");
        }
        self.value &= Self::MASK;
    }
}

impl<T, Bits> Sub for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitAnd<Output = T> + Sub<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // No need for extra overflow checking as the regular minus operator already handles it for us
        Self::new_wrapping(self.value - rhs.value)
    }
}

impl<T, Bits> SubAssign for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + SubAssign + BitAndAssign,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn sub_assign(&mut self, rhs: Self) {
        // No need for extra overflow checking as the regular minus operator already handles it for us
        self.value -= rhs.value;
        self.value &= Self::MASK;
    }
}

impl<T, Bits> Mul for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Mul<Output = T> + BitAnd<Output = T> + Not<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // In debug builds, this will perform two bounds checks: Initial multiplication, followed by
        // our bounds check. As wrapping_mul isn't available as a trait bound (in regular Rust), this
        // is unavoidable
        let product = self.value * rhs.value;
        #[cfg(debug_assertions)]
        if (product & !Self::MASK) != Self::ZERO.value {
            panic!("attempt to multiply with overflow");
        }
        Self::new_wrapping(product)
    }
}

impl<T, Bits> MulAssign for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + MulAssign + BitAndAssign + BitAnd<Output = T> + Not<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.value *= rhs.value;
        #[cfg(debug_assertions)]
        if (self.value & !Self::MASK) != Self::ZERO.value {
            panic!("attempt to multiply with overflow");
        }
        self.value &= Self::MASK;
    }
}

impl<T, Bits> Div for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Div<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // Integer division can only make the value smaller. And as the result is same type as
        // Self, there's no need to range-check or mask

        unsafe { Self::new_unchecked(self.value / rhs.value) }
    }
}

impl<T, Bits> DivAssign for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + DivAssign,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl<T, Bits> Rem for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Rem<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        // Integer division can only make the value smaller. And as the result is same type as
        // Self, there's no need to range-check or mask
        unsafe { Self::new_unchecked(self.value % rhs.value) }
    }
}

impl<T, Bits> RemAssign for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + RemAssign,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value
    }
}

impl<T, Bits> Sum for AInt<T, Bits>
where
    Self: Number<Container = T> + Add<Output = Self>,
    T: AIntContainer,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, T, Bits> Sum<&'a AInt<T, Bits>> for AInt<T, Bits>
where
    Self: Number<Container = T> + Add<Output = Self>,
    T: AIntContainer,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, b| a + *b)
    }
}

impl<T, Bits> Product for AInt<T, Bits>
where
    Self: Number<Container = T> + Mul<Output = Self>,
    T: AIntContainer,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, T, Bits> Product<&'a AInt<T, Bits>> for AInt<T, Bits>
where
    Self: Number<Container = T> + Mul<Output = Self>,
    T: AIntContainer,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}

impl<T, Bits> BitAnd for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitAnd<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.value & rhs.value) }
    }
}

impl<T, Bits> BitAndAssign for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitAndAssign,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl<T, Bits> BitOr for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitOr<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.value | rhs.value) }
    }
}

impl<T, Bits> BitOrAssign for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitOrAssign,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl<T, Bits> BitXor for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitXor<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.value ^ rhs.value) }
    }
}

impl<T, Bits> BitXorAssign for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitXorAssign,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}

impl<T, Bits> Not for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitXor<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        unsafe { Self::new_unchecked(self.value ^ Self::MASK) }
    }
}

impl<T, TSHIFTBITS, Bits> Shl<TSHIFTBITS> for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Shl<TSHIFTBITS, Output = T> + BitAnd<Output = T>,
    TSHIFTBITS: TryInto<usize> + Copy,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn shl(self, rhs: TSHIFTBITS) -> Self::Output {
        // With debug assertions, the << and >> operators throw an exception if the shift amount
        // is larger than the number of bits (in which case the result would always be 0)
        #[cfg(debug_assertions)]
        if rhs.try_into().unwrap_or(usize::MAX) >= (Bits::U32 as usize) {
            panic!("attempt to shift left with overflow")
        }

        Self::new_wrapping(self.value << rhs)
    }
}

impl<T, TSHIFTBITS, Bits> ShlAssign<TSHIFTBITS> for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + ShlAssign<TSHIFTBITS> + BitAndAssign,
    TSHIFTBITS: TryInto<usize> + Copy,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn shl_assign(&mut self, rhs: TSHIFTBITS) {
        // With debug assertions, the << and >> operators throw an exception if the shift amount
        // is larger than the number of bits (in which case the result would always be 0)
        #[cfg(debug_assertions)]
        if rhs.try_into().unwrap_or(usize::MAX) >= (Bits::U32 as usize) {
            panic!("attempt to shift left with overflow")
        }
        self.value <<= rhs;
        self.value &= Self::MASK;
    }
}

impl<T, TSHIFTBITS, Bits> Shr<TSHIFTBITS> for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Shr<TSHIFTBITS, Output = T>,
    TSHIFTBITS: TryInto<usize> + Copy,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = AInt<T, Bits>;

    fn shr(self, rhs: TSHIFTBITS) -> Self::Output {
        // With debug assertions, the << and >> operators throw an exception if the shift amount
        // is larger than the number of bits (in which case the result would always be 0)
        #[cfg(debug_assertions)]
        if rhs.try_into().unwrap_or(usize::MAX) >= (Bits::U32 as usize) {
            panic!("attempt to shift left with overflow")
        }

        unsafe { Self::new_unchecked(self.value >> rhs) }
    }
}

impl<T, TSHIFTBITS, Bits> ShrAssign<TSHIFTBITS> for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + ShrAssign<TSHIFTBITS>,
    TSHIFTBITS: TryInto<usize> + Copy,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    fn shr_assign(&mut self, rhs: TSHIFTBITS) {
        // With debug assertions, the << and >> operators throw an exception if the shift amount
        // is larger than the number of bits (in which case the result would always be 0)
        #[cfg(debug_assertions)]
        if rhs.try_into().unwrap_or(usize::MAX) >= (Bits::U32 as usize) {
            panic!("attempt to shift left with overflow")
        }
        self.value >>= rhs;
    }
}

impl<T, Bits> Display for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T, Bits> LowerHex for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + LowerHex,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&self.value, f)
    }
}

impl<T, Bits> UpperHex for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + UpperHex,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        UpperHex::fmt(&self.value, f)
    }
}

impl<T, Bits> Octal for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Octal,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Octal::fmt(&self.value, f)
    }
}

impl<T, Bits> Binary for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Binary,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Binary::fmt(&self.value, f)
    }
}

impl<T, Bits> FromStr for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + FromStr<Err = ParseIntError>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Err = ParseAIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = match T::from_str(s) {
            Ok(v) => v,
            Err(err) => return Err(ParseAIntError::from_native(err)),
        };

        match Self::try_new(value) {
            Ok(v) => Ok(v),
            Err(err) => Err(ParseAIntError {
                kind: err.kind().clone(),
            }),
        }
    }
}

impl<T, Bits> Neg for AInt<T, Bits>
where
    Self: Number<Container = T>,
    T: AIntContainer + Neg<Output = T>,
    Bits: BitsSpec,
    T::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.value.neg())
    }
}



