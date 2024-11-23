use std::fmt::Debug;
use std::ops::Shr;
use crate::{Number, NumberErrorKind, NumberType, ParseNumberError, TryNewError};
use crate::util::CompileTimeAssert;

mod impl_core;

#[cfg(feature = "num-traits")]
mod impl_num_traits;

#[cfg(feature = "serde")]
mod impl_serde;

#[cfg(feature = "borsh")]
mod impl_borsh;

#[cfg(feature = "step_trait")]
mod impl_step_trait;

#[cfg(feature = "schemars")]
mod impl_schemars;

#[cfg(feature = "funty")]
mod impl_funty;

#[cfg(feature = "defmt")]
mod impl_defmt;


#[cfg(test)]
mod tests;

pub trait UnsignedNumberType:
    NumberType + From<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128>
{
}

impl<T> UnsignedNumberType for T where
    T: NumberType + From<u8> + TryFrom<u16> + TryFrom<u32> + TryFrom<u64> + TryFrom<u128>
{
}

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd, Debug)]
pub struct UInt<T, const BITS: usize>
where
    T: UnsignedNumberType,
{
    value: T,
}

impl<T, const BITS: usize> UInt<T, BITS>
where
    T: UnsignedNumberType,
{
    #[inline]
    pub const fn value(self) -> T {
        self.value
    }

    /// Initializes a new value without checking the bounds
    ///
    /// # Safety
    /// Must only be called with a value less than or equal to [Self::MAX](Self::MAX) value.
    #[inline]
    pub const unsafe fn new_unchecked(value: T) -> Self {
        Self { value }
    }
}

macro_rules! uint_impl_number {
    ($( $type:ty),+) => {
        $(

            impl<const BITS: usize> Number for UInt<$type, BITS>
            {
                type UnderlyingType = $type;
                type TryNewError = TryNewError;

                const BITS: usize = BITS;

                const BYTES: usize = (BITS + 7usize) / 8usize;

                const MIN: Self = <Self>::MIN;

                const MAX: Self = <Self>::MAX;

                const ZERO: Self = <Self>::ZERO;

                const ONE: Self = <Self>::ONE;


                #[inline]
                fn try_new(value: Self::UnderlyingType) -> Result<Self, Self::TryNewError> {
                    if value <= Self::MAX.value {
                        Ok(Self { value })
                    } else {
                        Err(TryNewError { kind: NumberErrorKind::PosOverflow })
                    }
                }

                #[inline]
                fn new(value: $type) -> Self {
                    assert!(value <= Self::MAX.value);

                    Self { value }
                }

                #[inline]
                unsafe fn new_unchecked(value: $type) -> Self {
                    Self { value }
                }

                #[inline]
                fn value(self) -> $type {
                    self.value
                }

                // /// Extracts bits from a given value. The extract is equivalent to: `new((value >> start_bit) & MASK)`
                // /// Unlike new, extract doesn't perform range-checking so it is slightly more efficient.
                // /// panics if start_bit+<number of bits> doesn't fit within an u8, e.g. u5::extract_u8(8, 4);
                // #[inline]
                // fn extract_from<F>(value: F, start_bit: usize) -> Result<Self, TryNewError>
                // where
                //     F: ReNum + Shr<usize, Output = <UInt<$type, BITS> as ReNum>::Container>,
                // {
                //     // TODO: better error
                //     assert!(start_bit + BITS <= F::BITS);

                //     Self::try_new(value >> start_bit)
                // }
            }

        )+
    };
}
uint_impl_number!(u8, u16, u32, u64, u128);

macro_rules! uint_impl {
    ($(($type:ident, $max_bytes:expr)),+) => {
        $(
            impl<const BITS: usize> UInt<$type, BITS> {

                pub const BITS: usize = BITS;
                pub const BYTES: usize = (BITS + 7usize) / 8usize;

                pub const ZERO: Self = Self { value: 0 };
                pub const ONE: Self = Self {value: 1 };

                pub const MIN: Self = Self::ZERO;

                // The existence of MAX also serves as a bounds check: If NUM_BITS is > available bits,
                // we will get a compiler error right here
                pub const MAX: Self = Self { value: (<$type>::MAX >> (<$type>::BITS as usize - Self::BITS)) };

                // pub const MASK: $type = Self::MAX.value;

                /// Creates an instance. Panics if the given value is outside of the valid range
                #[inline]
                pub const fn new(value: $type) -> Self {
                    assert!(value <= Self::MAX.value);

                    Self { value }
                }

                /// Creates an instance or an error if the given value is outside of the valid range
                #[inline]
                pub const fn try_new(value: $type) -> Result<Self, TryNewError> {
                    if value <= Self::MAX.value {
                        Ok(Self { value })
                    } else {
                        Err(TryNewError { kind: NumberErrorKind::PosOverflow})
                    }
                }

                #[inline]
                pub const fn bits() -> usize {
                    Self::BITS
                }

                #[inline]
                pub const fn bytes() -> usize {
                    Self::BYTES
                }

                #[inline]
                pub const fn zero() -> Self {
                    Self::ZERO
                }

                #[inline]
                pub const fn is_zero(self) -> bool {
                    self.value == Self::ZERO.value
                }

                #[inline]
                pub const fn one() -> Self {
                    Self::ONE
                }

                #[inline]
                pub const fn min_value() -> Self {
                    Self::MIN
                }

                #[inline]
                pub const fn max_value() -> Self {
                    Self::MAX
                }

                pub fn try_extract<E>(value: E, start_bit: usize) -> Result<Self, TryNewError>
                where
                    // From<$type> makes sure that any value of Self will fit in E
                    E: Number + Shr<usize, Output=E> + From<$type> + TryInto<$type>,
                    <E as TryInto<$type>>::Error: Debug,
                {
                    // TODO: get rid of assert and use errorable
                    assert!(start_bit + <$type>::BITS as usize <= E::BITS);

                    // Unwrap should be safe here since we did a check before
                    // TODO: handle unwrap
                    Self::try_new((value >> start_bit).try_into().unwrap())
                }

                pub fn extract<E>(value: E, start_bit: usize) -> Self
                where
                    // From<$type> makes sure that any value of Self will fit in E
                    E: Number + Shr<usize, Output=E> + From<$type> + TryInto<$type>,
                    <E as TryInto<$type>>::Error: Debug,
                {
                    assert!(start_bit + <$type>::BITS as usize <= E::BITS);

                    // Unwrap should be safe here since we did a check before
                    Self::new((value >> start_bit).try_into().unwrap())
                }

                /// Returns a UInt with a wider bit depth but with the same base data type
                pub const fn widen<const WIDE_BITS: usize>(
                    self,
                ) -> UInt<$type, WIDE_BITS> {
                    let _ = CompileTimeAssert::<BITS, WIDE_BITS>::LESSER_OR_EQUAL;

                    // Query MAX of the result to ensure we get a compiler error if the current definition is bogus (e.g. <u8, 9>)
                    let _ = UInt::<$type, WIDE_BITS>::MAX;
                    UInt::<$type, WIDE_BITS> { value: self.value }
                }

                #[inline]
                pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                    if core::mem::size_of::<$type>() << 3 == BITS {
                        // We are something like a UInt::<u8; 8>. We can fallback to the base implementation
                        match self.value.checked_add(rhs.value) {
                            Some(value) => Some(Self { value }),
                            None => None,
                        }
                    } else {
                        // We're dealing with fewer bits than the underlying type (e.g. u7).
                        // That means the addition can never overflow the underlying type
                        let sum = self.value.wrapping_add(rhs.value);
                        if sum > Self::MAX.value {
                            None
                        } else {
                            Some(Self { value: sum })
                        }
                    }
                }

                // #[inline]
                // pub const fn checked_add_signed(self, rhs: &Int<$type, BITS>) -> Option<Self> {

                // }

                #[inline]
                pub const fn checked_div(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_div(rhs.value) {
                        Some(value) => Some(Self { value }),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_div_euclid(rhs.value) {
                        Some(value) => Some(Self { value }),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_mul(self, rhs: Self) -> Option<Self> {
                    let product = if BITS << 1 <= (core::mem::size_of::<$type>() << 3) {
                        // We have half the bits (e.g. u4 * u4) of the base type, so we can't overflow the base type
                        // wrapping_mul likely provides the best performance on all cpus
                        Some(self.value.wrapping_mul(rhs.value))
                    } else {
                        // We have more than half the bits (e.g. u6 * u6)
                        self.value.checked_mul(rhs.value)
                    };

                    match product {
                        Some(value) => {
                            if value > Self::MAX.value {
                                None
                            } else {
                                Some(Self { value })
                            }
                        }
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_neg(self) -> Option<Self> {
                    match self.value.checked_neg() {
                        Some(value) => Some(Self { value }),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_pow(self, exp: u32) -> Option<Self> {
                    match self.value.checked_pow(exp) {
                        Some(value) => {
                            if value > Self::MAX.value {
                                None
                            } else {
                                Some(Self { value })
                            }
                        }
                        None => None,
                    }
                }


                #[inline]
                pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_rem(rhs.value) {
                        Some(value) => Some(Self { value }),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_rem_euclid(rhs.value) {
                        Some(value) => Some(Self { value }),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
                    if rhs >= (BITS as u32) {
                        None
                    } else {
                        Some(Self {
                            value: (self.value << rhs) & Self::MASK,
                        })
                    }
                }

                #[inline]
                pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
                    if rhs >= (BITS as u32) {
                        None
                    } else {
                        Some(Self {
                            value: (self.value >> rhs),
                        })
                    }
                }

                #[inline]
                pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_sub(rhs.value) {
                        Some(value) => Some(Self { value }),
                        None => None,
                    }
                }

                #[inline]
                pub const fn count_ones(self) -> u32 {
                    // The upper bits are zero, so we can ignore them
                    self.value.count_ones()
                }

                #[inline]
                pub const fn count_zeros(self) -> u32 {
                    // The upper bits are zero, so we can have to subtract them from the result
                    let filler_bits = ((core::mem::size_of::<$type>() << 3) - BITS) as u32;
                    self.value.count_zeros() - filler_bits
                }

                #[inline]
                pub const fn div_euclid(self, rhs: Self) -> Self {
                    Self {
                        value: self.value.div_euclid(rhs.value),
                    }
                }

                // TODO: div_ceil

                #[inline]
                pub const fn from_be(value: Self) -> Self {
                    value.to_be()
                }

                #[cfg(not(feature="generic_const_exprs"))]
                #[inline]
                pub const fn from_be_bytes<const BYTES: usize>(from: [u8; BYTES] ) -> Self {
                    const { assert!(BYTES == Self::BYTES); }

                    let mut value: $type = 0;

                    let mut bx = 0;

                    while bx < Self::BYTES {
                        value |= if BITS > (8 * (bx + 1)) {
                            (from[bx] as $type) << (BITS - 8 * (bx + 1))
                        } else {
                            // For the last partial byte, shift just enough to align the remaining bits
                            (from[bx] as $type) << (8 * bx)
                        };
                        bx += 1;
                    }

                    Self { value }
                }

                #[cfg(feature="generic_const_exprs")]
                #[inline]
                pub const fn from_be_bytes(from: [u8; UInt::<$type, BITS>::BYTES] ) -> Self {
                    let mut value: $type = 0;

                    let mut bx = 0;
                    while bx < Self::BYTES {
                        value |= if BITS > (8 * (bx + 1)) {
                            (from[bx] as $type) << (BITS - 8 * (bx + 1))
                        } else {
                            // For the last partial byte, shift just enough to align the remaining bits
                            (from[bx] as $type) << (8 * bx)
                        };
                        bx += 1;
                    }

                    Self { value }
                }

                #[inline]
                pub const fn from_le(value: Self) -> Self {
                    value.to_le()
                }

                #[cfg(not(feature="generic_const_exprs"))]
                #[inline]
                pub const fn from_le_bytes<const BYTES: usize>(from: [u8; BYTES] ) -> Self {
                    const { assert!(BYTES == Self::BYTES); }

                    let mut value: $type = 0;
                    let mut bx = 0;

                    while bx < Self::BYTES {
                        value |= ((from[bx] as $type) * 8);
                        bx += 1;
                    }

                    Self { value }
                }

                #[cfg(feature="generic_const_exprs")]
                #[inline]
                pub const fn from_le_bytes(from: [u8; UInt::<$type, BITS>::BYTES] ) -> Self {
                    let mut value: $type = 0;

                    let mut bx = 0;

                    while bx < Self::BYTES {
                        value |= ((from[bx] as $type) * 8);
                        bx += 1;
                    }

                    Self { value }
                }

                #[cfg(not(feature="generic_const_exprs"))]
                #[inline]
                pub const fn from_ne_bytes<const BYTES: usize>(from: [u8; BYTES] ) -> Self {
                    #[cfg(target_endian = "little")]
                    {
                        Self::from_le_bytes(from)
                    }
                    #[cfg(target_endian = "big")]
                    {
                        Self::from_be_bytes(from)
                    }
                }

                #[cfg(feature="generic_const_exprs")]
                #[inline]
                pub const fn from_ne_bytes(from: [u8; UInt::<$type, BITS>::BYTES] ) -> Self {
                    #[cfg(target_endian = "little")]
                    {
                        Self::from_le_bytes(from)
                    }
                    #[cfg(target_endian = "big")]
                    {
                        Self::from_be_bytes(from)
                    }
                }


                #[inline]
                pub const fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseNumberError> {
                    let value = match $type::from_str_radix(s, radix) {
                        Ok(v) => v,
                        Err(err) => return Err(ParseNumberError::from_native(err)),
                    };

                    match value {
                        v if v < Self::MIN.value => Err(ParseNumberError{ kind: NumberErrorKind::NegOverflow }),
                        v if v > Self::MAX.value => Err(ParseNumberError{ kind: NumberErrorKind::PosOverflow }),
                        v => Ok(Self { value: v })
                    }
                }

                #[inline]
                pub const fn leading_ones(self) -> u32 {
                    let shift = ((core::mem::size_of::<$type>() << 3) - BITS) as u32;
                    (self.value << shift).leading_ones()
                }

                #[inline]
                pub const fn leading_zeros(self) -> u32 {
                    let shift = ((core::mem::size_of::<$type>() << 3) - BITS) as u32;
                    (self.value << shift).leading_zeros()
                }

                #[inline]
                pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                    let (value, overflow) = if core::mem::size_of::<$type>() << 3 == BITS {
                        // We are something like a UInt::<u8; 8>. We can fallback to the base implementation
                        self.value.overflowing_add(rhs.value)
                    } else {
                        // We're dealing with fewer bits than the underlying type (e.g. u7).
                        // That means the addition can never overflow the underlying type
                        let sum = self.value.wrapping_add(rhs.value);
                        let masked = sum & Self::MASK;
                        (masked, masked != sum)
                    };
                    (Self { value }, overflow)
                }

                // pub const fn overflowing_add_signed(self, rhs: Int<$type, ) -> (Self, bool) {

                // }


                #[inline]
                pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                    let value = self.value.wrapping_div(rhs.value);
                    (Self { value }, false)
                }

                #[inline]
                pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                    let value = self.value.wrapping_div_euclid(rhs.value);
                    (Self { value }, false)
                }

                #[inline]
                pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                    let (wrapping_product, overflow) = if BITS << 1 <= (core::mem::size_of::<$type>() << 3) {
                        // We have half the bits (e.g. u4 * u4) of the base type, so we can't overflow the base type
                        // wrapping_mul likely provides the best performance on all cpus
                        self.value.overflowing_mul(rhs.value)
                    } else {
                        // We have more than half the bits (e.g. u6 * u6)
                        self.value.overflowing_mul(rhs.value)
                    };

                    let masked = wrapping_product & Self::MASK;
                    let overflow2 = masked != wrapping_product;
                    (Self { value: masked }, overflow || overflow2)
                }

                #[inline]
                pub const fn overflowing_neg(self) -> (Self, bool) {
                    let (negated, overflow) = self.value.overflowing_neg();
                    let value = negated & Self::MASK;

                    let min = 1 << (BITS - 1);
                    let is_overflow = self.value == min || overflow;

                    (Self {value }, is_overflow)
                }

                #[inline]
                pub const fn overflowing_pow(self, exp: u32) -> (Self, bool) {
                    let (powed, overflow) = self.value.overflowing_pow(exp);

                    let overflowed = overflow || powed > Self::MAX.value;
                    let value = powed & Self::MASK;

                    (Self {value}, overflowed)
                }

                #[inline]
                pub const fn overflowing_rem(self, rhs: Self) -> (Self, bool) {
                    let (value, overflow) = self.value.overflowing_rem(rhs.value);
                    (Self {value}, overflow)
                }

                #[inline]
                pub const fn overflowing_rem_euclid(self, rhs: Self) -> (Self, bool) {
                    let (value, overflow) = self.value.overflowing_rem_euclid(rhs.value);
                    (Self {value}, overflow)
                }

                #[inline]
                pub const fn overflowing_shl(self, rhs: u32) -> (Self, bool) {
                    if rhs >= (BITS as u32) {
                        (
                            Self {
                                value: self.value << (rhs % (BITS as u32)),
                            },
                            true,
                        )
                    } else {
                        (
                            Self {
                                value: self.value << rhs,
                            },
                            false,
                        )
                    }
                }

                #[inline]
                pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                    if rhs >= (BITS as u32) {
                        (
                            Self {
                                value: self.value >> (rhs % (BITS as u32)),
                            },
                            true,
                        )
                    } else {
                        (
                            Self {
                                value: self.value >> rhs,
                            },
                            false,
                        )
                    }
                }

                #[inline]
                pub const fn overflowing_sub(self, rhs: Self) -> (Self, bool) {
                    // For unsigned numbers, the only difference is when we reach 0 - which is the same
                    // no matter the data size. In the case of overflow we do have the mask the result though
                    let (value, overflow) = self.value.overflowing_sub(rhs.value);
                    (
                        Self {
                            value: value & Self::MASK,
                        },
                        overflow,
                    )
                }

                #[inline]
                pub const fn pow(self, exp: u32) -> Self {
                    let powed = self.value.pow(exp);

                    if powed > Self::MAX.value {
                        panic!("attempt to pow with overflow");
                    }

                    Self {
                        value: powed,
                    }
                }

                #[inline]
                pub const fn rem_euclid(self, rhs: Self) -> Self {
                    Self {
                        value: self.value.rem_euclid(rhs.value),
                    }
                }

                #[inline]
                pub const fn reverse_bits(self) -> Self {
                    let shift_right = (core::mem::size_of::<$type>() << 3) - BITS;
                    Self {
                        value: self.value.reverse_bits() >> shift_right,
                    }
                }

                #[inline]
                pub const fn rotate_left(self, n: u32) -> Self {
                    let b = BITS as u32;
                    let n = if n >= b { n % b } else { n };

                    let moved_bits = (self.value << n) & Self::MASK;
                    let truncated_bits = self.value >> (b - n);
                    Self {
                        value: moved_bits | truncated_bits,
                    }

                }

                #[inline]
                pub const fn rotate_right(self, n: u32) -> Self {
                    let b = BITS as u32;
                    let n = if n >= b { n % b } else { n };

                    let moved_bits = self.value >> n;
                    let truncated_bits = (self.value << (b - n)) & Self::MASK;
                    Self {
                        value: moved_bits | truncated_bits,
                    }
                }

                #[inline]
                pub const fn saturating_add(self, rhs: Self) -> Self {
                    let saturated = if core::mem::size_of::<$type>() << 3 == BITS {
                        // We are something like a UInt::<u8; 8>. We can fallback to the base implementation
                        self.value.saturating_add(rhs.value)
                    } else {
                        // We're dealing with fewer bits than the underlying type (e.g. u7).
                        // That means the addition can never overflow the underlying type
                        let sum = self.value.wrapping_add(rhs.value);
                        let max = Self::MAX.value;
                        if sum > max {
                            max
                        } else {
                            sum
                        }
                    };
                    Self { value: saturated }
                }

                #[inline]
                pub const fn saturating_div(self, rhs: Self) -> Self {
                    // When dividing unsigned numbers, we never need to saturate.
                    // Divison by zero in saturating_div throws an exception (in debug and release mode),
                    // so no need to do anything special there either
                    Self {
                        value: self.value.saturating_div(rhs.value),
                    }
                }

                #[inline]
                pub const fn saturating_mul(self, rhs: Self) -> Self {
                    let product = if BITS << 1 <= (core::mem::size_of::<$type>() << 3) {
                        // We have half the bits (e.g. u4 * u4) of the base type, so we can't overflow the base type
                        // wrapping_mul likely provides the best performance on all cpus
                        self.value.wrapping_mul(rhs.value)
                    } else {
                        // We have more than half the bits (e.g. u6 * u6)
                        self.value.saturating_mul(rhs.value)
                    };

                    let max = Self::MAX.value;
                    let saturated = if product > max { max } else { product };
                    Self { value: saturated }
                }

                #[inline]
                pub const fn saturating_pow(self, exp: u32) -> Self {
                    // It might be possible to handwrite this to be slightly faster as both
                    // saturating_pow has to do a bounds-check and then we do second one
                    let powed = self.value.saturating_pow(exp);
                    let max = Self::MAX.value;
                    let saturated = if powed > max { max } else { powed };
                    Self { value: saturated }
                }

                #[inline]
                pub const fn saturating_sub(self, rhs: Self) -> Self {
                    // For unsigned numbers, the only difference is when we reach 0 - which is the same
                    // no matter the data size
                    Self {
                        value: self.value.saturating_sub(rhs.value),
                    }
                }

                #[inline]
                pub const fn swap_bytes(self) -> Self {
                    // swap_bytes() of the underlying type does most of the work. Then, we just need to shift
                    let amount: usize = (core::mem::size_of::<$type>() << 3) - Self::BITS;
                    Self {
                        value: self.value.swap_bytes() >> amount,
                    }
                }

                #[inline]
                pub const fn to_be(self) -> Self {
                    #[cfg(target_endian = "little")]
                    {
                        self.swap_bytes()
                    }
                    #[cfg(target_endian = "big")]
                    {
                        self
                    }
                }

                // Combining this new const generic with a compile time assert is a bit
                // weird, but since we can't have associated types based on generics
                // this is the best I could come up with
                // The alternative is to implement this for every single bit size which would be extremely slow
                #[cfg(not(feature="generic_const_exprs"))]
                #[inline]
                pub const fn to_be_bytes<const BYTES: usize>(self) -> [u8; BYTES] {
                    const { assert!(BYTES == Self::BYTES); }

                    let mut ret = [0; BYTES];

                    let mut bx = 0;

                    while bx < Self::BYTES {
                        ret[bx] =
                            if Self::BITS - ((bx + 1) << 3) > 0 {
                                (self.value >> (Self::BITS - (bx + 1) * 8)) as u8
                            } else {
                                // Only mask the relevant part for the last few bits
                                (self.value << ((bx + 1) * 8 - Self::BITS)) as u8
                            };

                        bx += 1;
                    }
                    ret
                }

                #[cfg(feature="generic_const_exprs")]
                #[inline]
                pub const fn to_be_bytes(self) -> [u8; UInt::<$type, BITS>::BYTES] {
                    let mut ret = [0; Self::BYTES];

                    let mut bx = 0;

                    while bx < Self::BYTES {
                        ret[bx] =
                            if Self::BITS - ((bx + 1) << 3) > 0 {
                                (self.value >> (Self::BITS - (bx + 1) * 8)) as u8
                            } else {
                                // Only mask the relevant part for the last few bits
                                (self.value << ((bx + 1) * 8 - Self::BITS)) as u8
                            };

                        bx += 1;
                    }
                    ret
                }


                #[inline]
                pub const fn to_le(self) -> Self {
                    #[cfg(target_endian = "little")]
                    {
                        self
                    }
                    #[cfg(target_endian = "big")]
                    {
                        self.swap_bytes()
                    }
                }

                #[cfg(not(feature="generic_const_exprs"))]
                #[inline]
                pub const fn to_le_bytes<const BYTES: usize>(self) -> [u8; BYTES] {
                    const { assert!(BYTES == Self::BYTES); }

                    let mut ret = [0; BYTES];

                    let mut bx = 0;
                    while bx < Self::BYTES {
                        ret[bx] = (self.value >> (bx * 8)) as u8;
                        bx += 1;
                    }
                    ret
                }

                #[cfg(feature="generic_const_exprs")]
                #[inline]
                pub const fn to_le_bytes(self) -> [u8; UInt::<$type, BITS>::BYTES] {
                    let mut ret = [0; Self::BYTES];

                    let mut bx = 0;
                    while bx < Self::BYTES {
                        ret[bx] = (self.value >> (bx * 8)) as u8;
                        bx += 1;
                    }
                    ret
                }

                #[cfg(not(feature="generic_const_exprs"))]
                #[inline]
                pub const fn to_ne_bytes<const BYTES: usize>(self) -> [u8; BYTES] {
                    #[cfg(target_endian = "little")]
                    {
                        self.to_le_bytes()
                    }
                    #[cfg(target_endian = "big")]
                    {
                        self.to_be_bytes()
                    }
                }

                #[cfg(feature="generic_const_exprs")]
                #[inline]
                pub const fn to_ne_bytes(self) -> [u8; UInt::<$type, BITS>::BYTES] {
                    #[cfg(target_endian = "little")]
                    {
                        self.to_le_bytes()
                    }
                    #[cfg(target_endian = "big")]
                    {
                        self.to_be_bytes()
                    }
                }

                #[inline]
                pub const fn trailing_ones(self) -> u32 {
                    self.value.trailing_ones()
                }

                #[inline]
                pub const fn trailing_zeros(self) -> u32 {
                    self.value.trailing_zeros()
                }

                #[inline]
                pub const fn wrapping_add(self, rhs: Self) -> Self {
                    let sum = self.value.wrapping_add(rhs.value);
                    Self {
                        value: sum & Self::MASK,
                    }
                }

                #[inline]
                pub const fn wrapping_div(self, rhs: Self) -> Self {
                    Self {
                        // No need to mask here - divisions always produce a result that is <= self
                        value: self.value.wrapping_div(rhs.value),
                    }
                }

                #[inline]
                pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
                    Self {
                        // No need to mask here - divisions always produce a result that is <= self
                        value: self.value.wrapping_div_euclid(rhs.value),
                    }
                }

                #[inline]
                pub const fn wrapping_mul(self, rhs: Self) -> Self {
                    let value = self.value.wrapping_mul(rhs.value);
                    Self {
                        value: value & Self::MASK,
                    }
                }

                #[inline]
                pub const fn wrapping_neg(self) -> Self {
                    // TODO: verify this!
                    let max = Self::MAX.value;
                    Self {
                        value: max + 1 - (self.value - max - 1)
                    }
                }

                #[inline]
                pub const fn wrapping_pow(self, exp: u32) -> Self {
                    // TODO: verify this!
                    let value = self.value.wrapping_pow(exp);
                    Self {
                        value: value & Self::MASK,
                    }
                }

                #[inline]
                pub const fn wrapping_rem(self, rhs: Self) -> Self {
                    Self {
                        value: self.value.wrapping_rem(rhs.value),
                    }
                }

                #[inline]
                pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                    Self {
                        value: self.value.wrapping_rem_euclid(rhs.value),
                    }
                }

                #[inline]
                pub const fn wrapping_shl(self, rhs: u32) -> Self {
                    // modulo is expensive on some platforms, so only do it when necessary
                    let shift_amount = if rhs >= (BITS as u32) {
                        rhs % (BITS as u32)
                    } else {
                        rhs
                    };

                    Self {
                        // We could use wrapping_shl here to make Debug builds slightly smaller;
                        // the downside would be that on weird CPUs that don't do wrapping_shl by
                        // default release builds would get slightly worse. Using << should give
                        // good release performance everywere
                        value: (self.value << (shift_amount as usize)) & Self::MASK,
                    }
                }

                #[inline]
                pub const fn wrapping_shr(self, rhs: u32) -> Self {
                    // modulo is expensive on some platforms, so only do it when necessary
                    let shift_amount = if rhs >= (BITS as u32) {
                        rhs % (BITS as u32)
                    } else {
                        rhs
                    };

                    Self {
                        value: (self.value >> (shift_amount as usize)),
                    }
                }

                #[inline]
                pub const fn wrapping_sub(self, rhs: Self) -> Self {
                    let sum = self.value.wrapping_sub(rhs.value);
                    Self {
                        value: sum & Self::MASK,
                    }
                }



            // impl<T, const BITS: usize> Sub for UInt<T, BITS>
            // where
            //     Self: ConstUpperBounded,
            //     T: PrimInt + ConstUpperBounded + Unsigned,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn sub(self, rhs: Self) -> Self::Output {
            //         // No need for extra overflow checking as the regular minus operator already handles it for us
            //         Self {
            //             value: (self.value - rhs.value) & Self::MASK,
            //         }
            //     }
            // }

            // impl<T, const BITS: usize> SubAssign for UInt<T, BITS>
            // where
            //     Self: ConstUpperBounded,
            //     T: PrimInt + SubAssign + BitAndAssign + Unsigned,
            // {
            //     fn sub_assign(&mut self, rhs: Self) {
            //         // No need for extra overflow checking as the regular minus operator already handles it for us
            //         self.value -= rhs.value;
            //         self.value &= Self::MASK;
            //     }
            // }

            // impl<T, const BITS: usize> Mul for UInt<T, BITS>
            // where
            //     Self: ConstZero + ConstUpperBounded,
            //     T: PrimInt + Unsigned,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn mul(self, rhs: Self) -> Self::Output {
            //         // In debug builds, this will perform two bounds checks: Initial multiplication, followed by
            //         // our bounds check. As wrapping_mul isn't available as a trait bound (in regular Rust), this
            //         // is unavoidable
            //         let product = self.value * rhs.value;
            //         #[cfg(debug_assertions)]
            //         if (product & !Self::MASK) != Self::ZERO.value {
            //             panic!("attempt to multiply with overflow");
            //         }
            //         Self {
            //             value: product & Self::MASK,
            //         }
            //     }
            // }

            // impl<T, const BITS: usize> MulAssign for UInt<T, BITS>
            // where
            //     Self: ConstUpperBounded + ConstZero,
            //     T: PrimInt + MulAssign + BitAndAssign + Unsigned,
            // {
            //     fn mul_assign(&mut self, rhs: Self) {
            //         self.value *= rhs.value;
            //         #[cfg(debug_assertions)]
            //         if (self.value & !Self::MASK) != Self::ZERO.value {
            //             panic!("attempt to multiply with overflow");
            //         }
            //         self.value &= Self::MASK;
            //     }
            // }

            // impl<T, const BITS: usize> Div for UInt<T, BITS>
            // where
            //     T: PrimInt + Unsigned,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn div(self, rhs: Self) -> Self::Output {
            //         // Integer division can only make the value smaller. And as the result is same type as
            //         // Self, there's no need to range-check or mask
            //         Self {
            //             value: self.value / rhs.value,
            //         }
            //     }
            // }

            // impl<T, const BITS: usize> DivAssign for UInt<T, BITS>
            // where
            //     T: PrimInt + DivAssign + Unsigned,
            // {
            //     fn div_assign(&mut self, rhs: Self) {
            //         self.value /= rhs.value;
            //     }
            // }

            // impl<T, const BITS: usize> Rem for UInt<T, BITS>
            // where
            //     T: PrimInt + Unsigned,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn rem(self, rhs: Self) -> Self::Output {
            //         // Integer division can only make the value smaller. And as the result is same type as
            //         // Self, there's no need to range-check or mask
            //         Self {
            //             value: self.value % rhs.value,
            //         }
            //     }
            // }

            // impl<T, const BITS: usize> RemAssign for UInt<T, BITS>
            // where
            //     T: PrimInt + RemAssign + Unsigned,
            // {
            //     fn rem_assign(&mut self, rhs: Self) {
            //         self.value %= rhs.value
            //     }
            // }

            // impl<T, const BITS: usize> Sum for UInt<T, BITS>
            // where
            //     Self: ConstZero + Add,
            //     T: PrimInt + Unsigned,
            // {
            //     fn sum<I>(iter: I) -> Self
            //     where
            //         I: Iterator<Item = Self>,
            //     {
            //         iter.fold(Self::ZERO, |a, b| a + b)
            //     }
            // }

            // impl<'a, T, const BITS: usize> Sum<&'a UInt<T, BITS>> for UInt<T, BITS>
            // where
            //     Self: ConstZero + Add,
            //     T: PrimInt + Unsigned,
            // {
            //     fn sum<I>(iter: I) -> Self
            //     where
            //         I: Iterator<Item = &'a Self>,
            //     {
            //         iter.fold(Self::ZERO, |a, b| a + *b)
            //     }
            // }

            // impl<T, const BITS: usize> Product for UInt<T, BITS>
            // where
            //     Self: ConstOne + Mul,
            //     T: PrimInt + Unsigned,
            // {
            //     fn product<I>(iter: I) -> Self
            //     where
            //         I: Iterator<Item = Self>,
            //     {
            //         iter.fold(Self::ONE, |a, b| a * b)
            //     }
            // }

            // impl<'a, T, const BITS: usize> Product<&'a UInt<T, BITS>> for UInt<T, BITS>
            // where
            //     Self: ConstOne + Mul,
            //     T: PrimInt + Unsigned,
            // {
            //     fn product<I>(iter: I) -> Self
            //     where
            //         I: Iterator<Item = &'a Self>,
            //     {
            //         iter.fold(Self::ONE, |a, b| a * *b)
            //     }
            // }

            // impl<T, const BITS: usize> BitAnd for UInt<T, BITS>
            // where
            //     T: PrimInt + Unsigned,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn bitand(self, rhs: Self) -> Self::Output {
            //         Self {
            //             value: self.value & rhs.value,
            //         }
            //     }
            // }

            // impl<T, const BITS: usize> BitAndAssign for UInt<T, BITS>
            // where
            //     T: PrimInt + BitAndAssign + Unsigned,
            // {
            //     fn bitand_assign(&mut self, rhs: Self) {
            //         self.value &= rhs.value;
            //     }
            // }

            // impl<T, const BITS: usize> BitOr for UInt<T, BITS>
            // where
            //     T: PrimInt + Unsigned + Unsigned,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn bitor(self, rhs: Self) -> Self::Output {
            //         Self {
            //             value: self.value | rhs.value,
            //         }
            //     }
            // }

            // impl<T, const BITS: usize> BitOrAssign for UInt<T, BITS>
            // where
            //     T: PrimInt + BitOrAssign + Unsigned,
            // {
            //     fn bitor_assign(&mut self, rhs: Self) {
            //         self.value |= rhs.value;
            //     }
            // }

            // impl<T, const BITS: usize> BitXor for UInt<T, BITS>
            // where
            //     T: PrimInt + Unsigned,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn bitxor(self, rhs: Self) -> Self::Output {
            //         Self {
            //             value: self.value ^ rhs.value,
            //         }
            //     }
            // }

            // impl<T, const BITS: usize> BitXorAssign for UInt<T, BITS>
            // where
            //     T: PrimInt + BitXorAssign + Unsigned,
            // {
            //     fn bitxor_assign(&mut self, rhs: Self) {
            //         self.value ^= rhs.value;
            //     }
            // }

            // impl<T, const BITS: usize> Not for UInt<T, BITS>
            // where
            //     Self: ConstUpperBounded,
            //     T: PrimInt + BitXor + Unsigned,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn not(self) -> Self::Output {
            //         Self {
            //             value: self.value ^ Self::MASK,
            //         }
            //     }
            // }

            // impl<T, TSHIFTBITS, const BITS: usize> Shl<TSHIFTBITS> for UInt<T, BITS>
            // where
            //     Self: ConstUpperBounded,
            //     T: PrimInt + Shl<TSHIFTBITS, Output = T> + BitAnd + Unsigned,
            //     TSHIFTBITS: TryInto<usize> + Copy,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn shl(self, rhs: TSHIFTBITS) -> Self::Output {
            //         // With debug assertions, the << and >> operators throw an exception if the shift amount
            //         // is larger than the number of bits (in which case the result would always be 0)
            //         #[cfg(debug_assertions)]
            //         if rhs.try_into().unwrap_or(usize::MAX) >= BITS {
            //             panic!("attempt to shift left with overflow")
            //         }

            //         Self {
            //             value: (self.value << rhs) & Self::MASK,
            //         }
            //     }
            // }

            // impl<T, TSHIFTBITS, const BITS: usize> ShlAssign<TSHIFTBITS> for UInt<T, BITS>
            // where
            //     Self: ConstUpperBounded,
            //     T: PrimInt + ShlAssign<TSHIFTBITS> + BitAndAssign + Unsigned,
            //     TSHIFTBITS: TryInto<usize> + Copy,
            // {
            //     fn shl_assign(&mut self, rhs: TSHIFTBITS) {
            //         // With debug assertions, the << and >> operators throw an exception if the shift amount
            //         // is larger than the number of bits (in which case the result would always be 0)
            //         #[cfg(debug_assertions)]
            //         if rhs.try_into().unwrap_or(usize::MAX) >= BITS {
            //             panic!("attempt to shift left with overflow")
            //         }
            //         self.value <<= rhs;
            //         self.value &= Self::MASK;
            //     }
            // }

            // impl<T, TSHIFTBITS, const BITS: usize> Shr<TSHIFTBITS> for UInt<T, BITS>
            // where
            //     T: PrimInt + Shr<TSHIFTBITS, Output = T> + Unsigned,
            //     TSHIFTBITS: TryInto<usize> + Copy,
            // {
            //     type Output = UInt<T, BITS>;

            //     fn shr(self, rhs: TSHIFTBITS) -> Self::Output {
            //         // With debug assertions, the << and >> operators throw an exception if the shift amount
            //         // is larger than the number of bits (in which case the result would always be 0)
            //         #[cfg(debug_assertions)]
            //         if rhs.try_into().unwrap_or(usize::MAX) >= BITS {
            //             panic!("attempt to shift left with overflow")
            //         }
            //         Self {
            //             value: self.value >> rhs,
            //         }
            //     }
            // }

            // impl<T, TSHIFTBITS, const BITS: usize> ShrAssign<TSHIFTBITS> for UInt<T, BITS>
            // where
            //     T: PrimInt + ShrAssign<TSHIFTBITS> + Unsigned,
            //     TSHIFTBITS: TryInto<usize> + Copy,
            // {
            //     fn shr_assign(&mut self, rhs: TSHIFTBITS) {
            //         // With debug assertions, the << and >> operators throw an exception if the shift amount
            //         // is larger than the number of bits (in which case the result would always be 0)
            //         #[cfg(debug_assertions)]
            //         if rhs.try_into().unwrap_or(usize::MAX) >= BITS {
            //             panic!("attempt to shift left with overflow")
            //         }
            //         self.value >>= rhs;
            //     }
            // }










            //     #[inline]
            //     pub const fn mul_add(self, a: Self, b: Self) -> Self {
            //         Self:add(Self::mul(self, a), b)
            //     }




            //     // pub use num_traits::CheckedNeg;





            //     impl<T, const BITS: usize> Saturating for UInt<T, BITS>
            //     where
            //         T: PrimInt + WrappingAdd + SaturatingAdd + ConstUpperBounded + SaturatingSub + Unsigned,
            //         Self: ConstZero + ConstUpperBounded,
            //     {
            //         #[inline]
            //         fn saturating_add(self, rhs: Self) -> Self {
            //             <Self as SaturatingAdd>::saturating_add(self, &rhs)
            //         }

            //         #[inline]
            //         fn saturating_sub(self, rhs: Self) -> Self {
            //             <Self as SaturatingSub>::saturating_sub(self, &rhs)
            //         }
            //     }



















            }
        )+
    }
}

uint_impl!((u8, 8), (u16, 2), (u32, 4), (u64, 8), (u128, 16));


// Conversions
macro_rules! from_uint_impl {
    ($from:ty, [$($into:ty),+]) => {
        $(
            impl<const BITS: usize, const BITS_FROM: usize> From<UInt<$from, BITS_FROM>>
                for UInt<$into, BITS>
            {
                #[inline]
                fn from(item: UInt<$from, BITS_FROM>) -> Self {
                    let _ = CompileTimeAssert::<BITS_FROM, BITS>::LESSER_OR_EQUAL;
                    Self { value: item.value as $into }
                }
            }
        )+
    };
}

macro_rules! from_native_impl {
    ($from:ty, [$($into:ty),+]) => {
        $(
            impl<const BITS: usize> From<$from> for UInt<$into, BITS> {
                #[inline]
                fn from(from: $from) -> Self {
                    let _ = CompileTimeAssert::<{<$from>::BITS as usize}, BITS>::LESSER_OR_EQUAL;
                    Self { value: from as $into }
                }
            }

            impl<const BITS: usize> From<UInt<$from, BITS>> for $into {
                #[inline]
                fn from(from: UInt<$from, BITS>) -> Self {
                    let _ = CompileTimeAssert::<{<$from>::BITS as usize}, BITS>::LESSER_OR_EQUAL;
                    from.value as $into
                }
            }
        )+
    };
}

from_uint_impl!(u8, [u16, u32, u64, u128]);
from_uint_impl!(u16, [u8, u32, u64, u128]);
from_uint_impl!(u32, [u8, u16, u64, u128]);
from_uint_impl!(u64, [u8, u16, u32, u128]);
from_uint_impl!(u128, [u8, u32, u64, u16]);

from_native_impl!(u8, [u8, u16, u32, u64, u128]);
from_native_impl!(u16, [u8, u16, u32, u64, u128]);
from_native_impl!(u32, [u8, u16, u32, u64, u128]);
from_native_impl!(u64, [u8, u16, u32, u64, u128]);
from_native_impl!(u128, [u8, u16, u32, u64, u128]);

// Define type aliases like u1, u63 and u80 using the smallest possible underlying data type.
// These are for convenience only - UInt<u32, 15> is still legal
macro_rules! type_alias {
    ($storage:ty, $(($name:ident, $bits:expr)),+) => {
        $( pub type $name = crate::UInt<$storage, $bits>; )+
    }
}

pub use aliases::*;

#[rustfmt::skip]
#[allow(non_camel_case_types)]
pub mod aliases {
    type_alias!(u8, (u1, 1), (u2, 2), (u3, 3), (u4, 4), (u5, 5), (u6, 6), (u7, 7));
    type_alias!(u16, (u9, 9), (u10, 10), (u11, 11), (u12, 12), (u13, 13), (u14, 14), (u15, 15));
    type_alias!(u32, (u17, 17), (u18, 18), (u19, 19), (u20, 20), (u21, 21), (u22, 22), (u23, 23), (u24, 24), (u25, 25), (u26, 26), (u27, 27), (u28, 28), (u29, 29), (u30, 30), (u31, 31));
    type_alias!(u64, (u33, 33), (u34, 34), (u35, 35), (u36, 36), (u37, 37), (u38, 38), (u39, 39), (u40, 40), (u41, 41), (u42, 42), (u43, 43), (u44, 44), (u45, 45), (u46, 46), (u47, 47), (u48, 48), (u49, 49), (u50, 50), (u51, 51), (u52, 52), (u53, 53), (u54, 54), (u55, 55), (u56, 56), (u57, 57), (u58, 58), (u59, 59), (u60, 60), (u61, 61), (u62, 62), (u63, 63));
    type_alias!(u128, (u65, 65), (u66, 66), (u67, 67), (u68, 68), (u69, 69), (u70, 70), (u71, 71), (u72, 72), (u73, 73), (u74, 74), (u75, 75), (u76, 76), (u77, 77), (u78, 78), (u79, 79), (u80, 80), (u81, 81), (u82, 82), (u83, 83), (u84, 84), (u85, 85), (u86, 86), (u87, 87), (u88, 88), (u89, 89), (u90, 90), (u91, 91), (u92, 92), (u93, 93), (u94, 94), (u95, 95), (u96, 96), (u97, 97), (u98, 98), (u99, 99), (u100, 100), (u101, 101), (u102, 102), (u103, 103), (u104, 104), (u105, 105), (u106, 106), (u107, 107), (u108, 108), (u109, 109), (u110, 110), (u111, 111), (u112, 112), (u113, 113), (u114, 114), (u115, 115), (u116, 116), (u117, 117), (u118, 118), (u119, 119), (u120, 120), (u121, 121), (u122, 122), (u123, 123), (u124, 124), (u125, 125), (u126, 126), (u127, 127));
}

// We need to wrap this in a macro, currently: https://github.com/rust-lang/rust/issues/67792#issuecomment-1130369066

macro_rules! boolu1 {
    () => {
        impl From<bool> for u1 {
            #[inline]
            fn from(value: bool) -> Self {
                u1::new(value as u8)
            }
        }
        impl From<u1> for bool {
            #[inline]
            fn from(value: u1) -> Self {
                match value.value {
                    0 => false,
                    1 => true,
                    _ => panic!("ReNum already validates that this is unreachable"), //TODO: unreachable!() is not const yet
                }
            }
        }
    };
}

boolu1!();
