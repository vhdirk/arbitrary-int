use core::fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex};
use core::hash::{Hash, Hasher};

use core::iter::{Product, Sum};
use core::str::FromStr;

use core::num::ParseIntError;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign, Neg,
};

use crate::{AIntErrorKind, Number, ParseAIntError};

use crate::{AInt, AIntContainer};

impl<T, const BITS: usize> Hash for AInt<T, BITS>
where
    T: AIntContainer + Hash,
    Self: Number<Container = T>,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state)
    }
}

impl<T, const BITS: usize> Add for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Add<Output = T> + Not<Output = T> + BitAnd<Output = T>,
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

impl<T, const BITS: usize> AddAssign for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer
        + Add<Output = T>
        + Not<Output = T>
        + BitAnd<Output = T>
        + AddAssign
        + BitAndAssign,
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

impl<T, const BITS: usize> Sub for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitAnd<Output = T> + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // No need for extra overflow checking as the regular minus operator already handles it for us
        Self::new_wrapping(self.value - rhs.value)
    }
}

impl<T, const BITS: usize> SubAssign for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + SubAssign + BitAndAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        // No need for extra overflow checking as the regular minus operator already handles it for us
        self.value -= rhs.value;
        self.value &= Self::MASK;
    }
}

impl<T, const BITS: usize> Mul for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Mul<Output = T> + BitAnd<Output = T> + Not<Output = T>,
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

impl<T, const BITS: usize> MulAssign for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + MulAssign + BitAndAssign + BitAnd<Output = T> + Not<Output = T>,
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

impl<T, const BITS: usize> Div for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Div<Output = T>,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        // Integer division can only make the value smaller. And as the result is same type as
        // Self, there's no need to range-check or mask

        unsafe { Self::new_unchecked(self.value / rhs.value) }
    }
}

impl<T, const BITS: usize> DivAssign for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.value /= rhs.value;
    }
}

impl<T, const BITS: usize> Rem for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Rem<Output = T>,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        // Integer division can only make the value smaller. And as the result is same type as
        // Self, there's no need to range-check or mask
        unsafe { Self::new_unchecked(self.value % rhs.value) }
    }
}

impl<T, const BITS: usize> RemAssign for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.value %= rhs.value
    }
}

impl<T, const BITS: usize> Sum for AInt<T, BITS>
where
    Self: Number<Container = T> + Add<Output = Self>,
    T: AIntContainer,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ZERO, |a, b| a + b)
    }
}

impl<'a, T, const BITS: usize> Sum<&'a AInt<T, BITS>> for AInt<T, BITS>
where
    Self: Number<Container = T> + Add<Output = Self>,
    T: AIntContainer,
{
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ZERO, |a, b| a + *b)
    }
}

impl<T, const BITS: usize> Product for AInt<T, BITS>
where
    Self: Number<Container = T> + Mul<Output = Self>,
    T: AIntContainer,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::ONE, |a, b| a * b)
    }
}

impl<'a, T, const BITS: usize> Product<&'a AInt<T, BITS>> for AInt<T, BITS>
where
    Self: Number<Container = T> + Mul<Output = Self>,
    T: AIntContainer,
{
    fn product<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self::ONE, |a, b| a * *b)
    }
}

impl<T, const BITS: usize> BitAnd for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitAnd<Output = T>,
{
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.value & rhs.value) }
    }
}

impl<T, const BITS: usize> BitAndAssign for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitAndAssign,
{
    fn bitand_assign(&mut self, rhs: Self) {
        self.value &= rhs.value;
    }
}

impl<T, const BITS: usize> BitOr for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitOr<Output = T>,
{
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.value | rhs.value) }
    }
}

impl<T, const BITS: usize> BitOrAssign for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitOrAssign,
{
    fn bitor_assign(&mut self, rhs: Self) {
        self.value |= rhs.value;
    }
}

impl<T, const BITS: usize> BitXor for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitXor<Output = T>,
{
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        unsafe { Self::new_unchecked(self.value ^ rhs.value) }
    }
}

impl<T, const BITS: usize> BitXorAssign for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitXorAssign,
{
    fn bitxor_assign(&mut self, rhs: Self) {
        self.value ^= rhs.value;
    }
}

impl<T, const BITS: usize> Not for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + BitXor<Output = T>,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        unsafe { Self::new_unchecked(self.value ^ Self::MASK) }
    }
}

impl<T, TSHIFTBITS, const BITS: usize> Shl<TSHIFTBITS> for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Shl<TSHIFTBITS, Output = T> + BitAnd<Output = T>,
    TSHIFTBITS: TryInto<usize> + Copy,
{
    type Output = Self;

    fn shl(self, rhs: TSHIFTBITS) -> Self::Output {
        // With debug assertions, the << and >> operators throw an exception if the shift amount
        // is larger than the number of bits (in which case the result would always be 0)
        #[cfg(debug_assertions)]
        if rhs.try_into().unwrap_or(usize::MAX) >= BITS {
            panic!("attempt to shift left with overflow")
        }

        Self::new_wrapping(self.value << rhs)
    }
}

impl<T, TSHIFTBITS, const BITS: usize> ShlAssign<TSHIFTBITS> for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + ShlAssign<TSHIFTBITS> + BitAndAssign,
    TSHIFTBITS: TryInto<usize> + Copy,
{
    fn shl_assign(&mut self, rhs: TSHIFTBITS) {
        // With debug assertions, the << and >> operators throw an exception if the shift amount
        // is larger than the number of bits (in which case the result would always be 0)
        #[cfg(debug_assertions)]
        if rhs.try_into().unwrap_or(usize::MAX) >= BITS {
            panic!("attempt to shift left with overflow")
        }
        self.value <<= rhs;
        self.value &= Self::MASK;
    }
}

impl<T, TSHIFTBITS, const BITS: usize> Shr<TSHIFTBITS> for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Shr<TSHIFTBITS, Output = T>,
    TSHIFTBITS: TryInto<usize> + Copy,
{
    type Output = AInt<T, BITS>;

    fn shr(self, rhs: TSHIFTBITS) -> Self::Output {
        // With debug assertions, the << and >> operators throw an exception if the shift amount
        // is larger than the number of bits (in which case the result would always be 0)
        #[cfg(debug_assertions)]
        if rhs.try_into().unwrap_or(usize::MAX) >= BITS {
            panic!("attempt to shift left with overflow")
        }

        unsafe { Self::new_unchecked(self.value >> rhs) }
    }
}

impl<T, TSHIFTBITS, const BITS: usize> ShrAssign<TSHIFTBITS> for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + ShrAssign<TSHIFTBITS>,
    TSHIFTBITS: TryInto<usize> + Copy,
{
    fn shr_assign(&mut self, rhs: TSHIFTBITS) {
        // With debug assertions, the << and >> operators throw an exception if the shift amount
        // is larger than the number of bits (in which case the result would always be 0)
        #[cfg(debug_assertions)]
        if rhs.try_into().unwrap_or(usize::MAX) >= BITS {
            panic!("attempt to shift left with overflow")
        }
        self.value >>= rhs;
    }
}

impl<T, const BITS: usize> Display for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.value, f)
    }
}

impl<T, const BITS: usize> LowerHex for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + LowerHex,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        LowerHex::fmt(&self.value, f)
    }
}

impl<T, const BITS: usize> UpperHex for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + UpperHex,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        UpperHex::fmt(&self.value, f)
    }
}

impl<T, const BITS: usize> Octal for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Octal,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Octal::fmt(&self.value, f)
    }
}

impl<T, const BITS: usize> Binary for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Binary,
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Binary::fmt(&self.value, f)
    }
}

impl<T, const BITS: usize> FromStr for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + FromStr<Err = ParseIntError>,
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

impl<T, const BITS: usize> Neg for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(self.value.neg())
    }
}



