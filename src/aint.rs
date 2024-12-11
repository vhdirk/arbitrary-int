use crate::assert_bounds;
use crate::traits::{Signed, Unsigned};
use crate::util::ConstBounded;
use crate::{AIntContainer, AIntErrorKind, Number, ParseAIntError, TryNewError};
use core::fmt;
use std::fmt::Debug;
use std::ops::{BitAnd, Shr};

#[derive(Copy, Clone, Eq, PartialEq, Default, Ord, PartialOrd)]
#[repr(transparent)]
pub struct AInt<T, const BITS: usize>
where
    T: AIntContainer,
{
    pub(crate) value: T,
}

impl<T, const BITS: usize> Debug for AInt<T, BITS>
where
    T: AIntContainer + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(&self.value, f)
    }
}

impl<T, const BITS: usize> AInt<T, BITS>
where
    T: AIntContainer + Debug,
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

macro_rules! aint_impl_unsigned {
    ($($type:ident),+) => {
        $(
            impl<const BITS: usize> AInt<$type, BITS>
            where
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, {<$type>::BITS as usize}>: Sized
            {
                // we define all consts here and reference them in impl Number; that way, we can use them in const fns
                pub const ZERO: Self = Self { value: 0 };
                pub const ONE: Self = Self { value: 1 };

                #[allow(unused_comparisons)]
                pub const SIGNED: bool = <$type>::MIN < 0;

                pub const MIN: Self = assert_bounds!((BITS, 1, <$type>::BITS as usize) => { Self { value: (<$type>::MIN >> (<$type>::BITS - BITS as u32)) } });

                // The existence of MAX also serves as a bounds check: If NUM_BITS is > available bits,
                // we will get a compiler error right here
                pub const MAX: Self = assert_bounds!((BITS, 1, <$type>::BITS as usize) => { Self { value: (<$type>::MAX >> (<$type>::BITS - BITS as u32)) } });

                // can't shift by BITS in one go. x86 doesn't support that
                pub const MASK: $type = assert_bounds!((BITS, 1, <$type>::BITS as usize) => { !(((!(0 as $type)) << (BITS -1) ) << 1) });

            }

            impl<const BITS: usize> Unsigned for AInt<$type, BITS>
            where
                Self: Number<Container=$type>,
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, {<$type>::BITS as usize}>: Sized
            { }
        )+
    }
}

aint_impl_unsigned!(u8, u16, u32, u64, u128);

macro_rules! aint_impl_signed {
    ($($type:ident),+) => {
        $(

            impl<const BITS: usize> AInt<$type, BITS>
            where
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, {<$type>::BITS as usize}>: Sized
            {  }

            impl<const BITS: usize> Signed for AInt<$type, BITS>
            where
                Self: Number<Container=$type>,
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, {<$type>::BITS as usize}>: Sized
            { }

        )+
    }
}

aint_impl_signed!(i8, i16, i32, i64, i128);

macro_rules! aint_impl_number {
    ($( $type:ty, ( $unsigned:ty, $signed:ty ) ),+) => {
        $(

            impl<const BITS: usize> AInt<$type, BITS>
            where
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, {<$type>::BITS as usize}>: Sized
            {

                #[allow(unused)]
                pub(crate) const fn sign_bit() -> $type {
                    if Self::SIGNED {
                        Self::ONE.value << (BITS - 1)
                    } else {
                        Self::ZERO.value
                    }
                }

                #[inline]
                /// Creates an instance. Panics if the given value is outside of the valid range
                pub const fn new(value: $type) -> Self  {
                    assert!(value <= Self::MAX.value);
                    assert!(value >= Self::MIN.value);

                    Self { value }
                }

            }


            impl<const BITS: usize > Number for AInt<$type, BITS>
            where
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, {<$type>::BITS as usize}>: Sized
            {
                type Container = $type;
                type TryNewError = TryNewError;

                type UnsignedEquivalent = AInt<$unsigned, BITS>;
                type SignedEquivalent =  AInt<$signed, BITS>;

                const BITS: u32 = BITS as u32;
                const BYTES: u32 = (BITS as u32 + 7) >> 3;

                // copy/paste from above since the compiler doesn't like the cyclic stuff referencing Self could generate

                const ZERO: Self = Self { value: 0 };
                const ONE: Self = Self { value: 1 };

                #[allow(unused_comparisons)]
                const SIGNED: bool = <$type>::MIN < 0;

                const MIN: Self = assert_bounds!((BITS, 1, <$type>::BITS as usize ) => { Self { value: (<$type>::MIN >> (<$type>::BITS - BITS as u32)) } });

                // The existence of MAX also serves as a bounds check: If NUM_BITS is > available bits,
                // we will get a compiler error right here
                const MAX: Self = assert_bounds!((BITS, 1, <$type>::BITS as usize ) => { Self { value: (<$type>::MAX >> (<$type>::BITS - BITS as u32)) } });

                // can't shift by BITS in one go. x86 doesn't support that
                const MASK: $type = assert_bounds!((BITS, 1, <$type>::BITS as usize ) => { !(((!(0 as $type)) << (BITS -1) ) << 1) });

                #[inline]
                fn try_new(value: Self::Container) -> Result<Self, Self::TryNewError> {
                    Self::try_new(value)
                }

                #[inline]
                fn new(value: Self::Container) -> Self {
                    Self::new(value)
                }

                #[inline]
                unsafe fn new_unchecked(value: Self::Container) -> Self {
                    Self { value }
                }

                #[inline]
                fn value(self) -> $type {
                    self.value
                }

                fn new_wrapping(value: Self::Container) -> Self {
                    <Self>::new_wrapping(value)
                }

                fn new_saturating(value: Self::Container) -> Self {
                    <Self>::new_saturating(value)
                }

                fn new_overflowing(value: Self::Container) -> (Self, bool) {
                    <Self>::new_overflowing(value)
                }

                fn as_signed(self) -> Self::SignedEquivalent {
                    Self::SignedEquivalent::new(self.value as $signed)
                }

                fn as_unsigned(self) -> Self::UnsignedEquivalent {
                    Self::UnsignedEquivalent::new(self.value as $unsigned)
                }

                // /// Extracts bits from a given value. The extract is equivalent to: `new((value >> start_bit) & MASK)`
                // /// Unlike new, extract doesn't perform range-checking so it is slightly more efficient.
                // /// panics if start_bit+<number of bits> doesn't fit within an u8, e.g. u5::extract_u8(8, 4);
                // #[inline]
                // fn extract_from<F>(value: F, start_bit: usize) -> Result<Self, TryNewError>
                // where
                //     F: ReNum + Shr<usize, Output = <AInt<$type, BITS> as ReNum>::Container>,
                // {
                //     // TODO: better error
                //     assert!(start_bit + BITS <= F::BITS);

                //     Self::try_new(value >> start_bit)
                // }
            }

        )+
    };
}

aint_impl_number!(u8, (u8, i8));
aint_impl_number!(u16, (u16, i16));
aint_impl_number!(u32, (u32, i32));
aint_impl_number!(u64, (u64, i64));
aint_impl_number!(u128, (u128, i128));

aint_impl_number!(i8, (u8, i8));
aint_impl_number!(i16, (u16, i16));
aint_impl_number!(i32, (u32, i32));
aint_impl_number!(i64, (u64, i64));
aint_impl_number!(i128, (u128, i128));

macro_rules! aint_impl {
    ($($type:ident),+) => {
        $(

            impl<const BITS: usize> AInt<$type, BITS>
            where
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, { <$type>::BITS as usize }>: Sized
            {
                // #[inline]
                // const fn new_impl(value: $type) -> Self  {
                //     Self { value }
                // }

                // #[inline]
                // /// Creates an instance. Panics if the given value is outside of the valid range
                // pub const fn new(value: $type) -> Self  {
                //     assert!(value <= Self::MAX.value);
                //     assert!(value >= Self::MIN.value);

                //     Self { value }
                // }

                // #[inline]
                // pub const fn value(self) -> $type {
                //     self.value
                // }

                // /// Initializes a new value without checking the bounds
                // ///
                // /// # Safety
                // /// Must only be called with a value less than or equal to [Self::MAX](Self::MAX) value.
                // #[inline]
                // pub const unsafe fn new_unchecked(value: $type) -> Self {

                //     Self { value }
                // }


                /// Creates an instance or an error if the given value is outside of the valid range
                #[inline]
                pub const fn try_new(value: $type) -> Result<Self, TryNewError> {
                    if value > Self::MAX.value {
                        Err(TryNewError { kind: AIntErrorKind::PosOverflow})
                    } else if value < Self::MIN.value {
                        Err(TryNewError { kind: AIntErrorKind::NegOverflow})
                    } else {
                        Ok(Self {value })
                    }
                }

                #[inline]
                pub const fn new_wrapping(value: $type) -> Self {
                    if Self::SIGNED {
                        if (value & Self::MASK) == 0 {
                            Self {value: value & Self::MAX.value }
                        } else {
                            Self {value: value | !Self::MAX.value }
                        }
                    } else {
                        Self {value: value & Self::MASK }
                    }
                }

                /// Creates a new integer value from the underlying representation type.
                ///
                /// The returned value is saturated to the bounds of this integer's value range. If the
                /// representation value is greater than [`MAX`](Self::MAX), the returned value will be
                /// [`MAX`](Self::MAX). If the representation value is less than [`MIN`](Self::MIN), the
                /// returned value will be [`MIN`](Self::MIN).
                #[inline]
                pub const fn new_saturating(value: $type) -> Self{
                    if value >= Self::MAX.value {
                        Self::MAX
                    } else if value <= Self::MIN.value {
                        Self::MIN
                    } else {
                        Self {value }
                    }
                }

                #[track_caller]
                #[inline]
                pub(crate) const fn new_overflowing_impl((value, overflow): ($type, bool)) -> (Self, bool) {
                    if value > Self::MAX.value {
                        (Self {value: value & Self::MAX.value}, true)
                    } else if value < Self::MIN.value {
                        (Self {value: value | !Self::MAX.value}, true)
                    } else {
                        (Self {value}, overflow)
                    }
                }

                #[inline]
                pub(crate) const fn new_overflowing(value: $type) -> (Self, bool) {
                    Self::new_overflowing_impl((value, false))
                }

                /// Returns the sign of the integer.
                ///
                /// * If `self < 0`, returns `-1`
                /// * If `self > 0`, returns `1`
                /// * If `self == 0`, returns `0`
                #[inline]
                pub const fn signum(self) -> Self {
                    if self.value == Self::ZERO.value {
                        Self::ZERO
                    } else if self.value > Self::ZERO.value {
                        Self::ONE
                    } else {
                        // Can not overflow since we just checked
                        #[allow(arithmetic_overflow)]
                        Self {value: 0 - 1}
                    }
                }

                #[inline]
                pub const fn bits() -> u32 {
                    Self::BITS
                }

                #[inline]
                pub const fn bytes() -> u32 {
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

                #[cfg(not(feature = "generic_const_exprs"))]
                pub fn try_extract_from<F>(value: F, start_bit: u32) -> Result<Self, TryNewError>
                where
                    // From<Self> makes sure that any value of Self will fit in and as such, F has to be equal in size or larger
                    F: Number + From<Self>,
                    <F as Number>::Container: Shr<u32, Output=<F as Number>::Container> + BitAnd<<F as Number>::Container, Output=<F as Number>::Container> + TryInto<$type>,
                    <F as Number>::Container: TryInto<$type> + From<$type>,
                    <F::Container as TryInto<$type>>::Error: Debug,

                    $type: TryFrom<<F as Number>::Container>,
                    <$type as TryFrom<<F as Number>::Container>>::Error: Debug,
                {
                    if (start_bit + Self::BITS) > <F as Number>::BITS {
                        return Err(TryNewError{ kind: AIntErrorKind::PosOverflow})
                    }

                    // Unwrap should be safe here since we did a check before
                    Ok(Self::new_wrapping( TryInto::<$type>::try_into((value.value() >> start_bit) & Self::MASK.into()).unwrap()))
                }

                #[cfg(not(feature = "generic_const_exprs"))]
                pub fn extract_from<F>(value: F, start_bit: u32) -> Self
                where
                    // From<Self> makes sure that any value of Self will fit in and as such, F has to be equal in size or larger
                    F: Number + From<Self>,
                    <F as Number>::Container: Shr<u32, Output=<F as Number>::Container> + BitAnd<<F as Number>::Container, Output=<F as Number>::Container> + TryInto<$type>,
                    <F as Number>::Container: TryInto<$type> + From<$type>,
                    <F::Container as TryInto<$type>>::Error: Debug,

                    $type: TryFrom<<F as Number>::Container>,
                    <$type as TryFrom<<F as Number>::Container>>::Error: Debug,
                {
                    assert!((start_bit + Self::BITS) <= <F as Number>::BITS );

                    // Unwrap should be safe here since we did a check before
                    Self { value: TryInto::<$type>::try_into((value.value() >> start_bit) & Self::MASK.into()).unwrap()}
                }

                #[cfg(feature = "generic_const_exprs")]
                pub fn try_extract_from<F>(value: F, start_bit: u32) -> Result<Self, TryNewError>
                where
                    // From<Self> makes sure that any value of Self will fit in and as such, F has to be equal in size or larger
                    F: Number + From<Self>,
                    ConstBounded<{<F as Number>::BITS as usize}, BITS, 128>: Sized,
                    <F as Number>::Container: Shr<u32, Output=<F as Number>::Container> + BitAnd<<F as Number>::Container, Output=<F as Number>::Container> + TryInto<$type>,
                    <F as Number>::Container: TryInto<$type> + From<$type>,
                    <F as TryInto<$type>>::Error: Debug,

                    $type: TryFrom<<F as Number>::Container>,
                    <$type as TryFrom<<F as Number>::Container>>::Error: Debug,
                {
                    if (start_bit + Self::BITS) > <F as Number>::BITS {
                        return Err(TryNewError{ kind: AIntErrorKind::PosOverflow})
                    }

                    // Unwrap should be safe here since we did a check before
                    Ok(Self::new_wrapping( TryInto::<$type>::try_into((value.value() >> start_bit) & Self::MASK.into()).unwrap()))
                }

                #[cfg(feature = "generic_const_exprs")]
                pub fn extract_from<F>(value: F, start_bit: u32) -> Self
                where
                    // From<Self> makes sure that any value of Self will fit in and as such, F has to be equal in size or larger
                    F: Number + From<Self>,
                    ConstBounded<{<F as Number>::BITS as usize}, BITS, 128>: Sized,
                    <F as Number>::Container: Shr<u32, Output=<F as Number>::Container> + BitAnd<<F as Number>::Container, Output=<F as Number>::Container> + TryInto<$type>,
                    <F as Number>::Container: TryInto<$type> + From<$type>,
                    <F as TryInto<$type>>::Error: Debug,

                    $type: TryFrom<<F as Number>::Container>,
                    <$type as TryFrom<<F as Number>::Container>>::Error: Debug,
                {
                    assert!((start_bit + Self::BITS) <= <F as Number>::BITS );

                    // Unwrap should be safe here since we did a check before
                    Self { value: TryInto::<$type>::try_into((value.value() >> start_bit) & Self::MASK.into()).unwrap()}
                }

                /// Returns a AInt with a wider bit depth but with the same base data type
                pub const fn widen<const WIDEN_BITS: usize>(
                    self,
                ) -> AInt<$type, WIDEN_BITS>
                where
                    ConstBounded<WIDEN_BITS, BITS, { <$type>::BITS as usize }>: Sized
                {
                    AInt::<$type, WIDEN_BITS>::new(self.value)
                }

                #[inline]
                pub const fn checked_add(self, rhs: Self) -> Option<Self> {
                    if core::mem::size_of::<$type>() << 3 == BITS {
                        // We are something like a AInt::<u8; 8>. We can fallback to the base implementation
                        match self.value.checked_add(rhs.value) {
                            Some(value) => Some(Self {value}),
                            None => None,
                        }
                    } else {
                        // We're dealing with fewer bits than the underlying type (e.g. u7).
                        // That means the addition can never overflow the underlying type
                        let sum = self.value.wrapping_add(rhs.value);
                        if sum > Self::MAX.value {
                            None
                        } else {
                            Some(Self {value: sum})
                        }
                    }
                }

                // #[inline]
                // pub const fn checked_add_signed(self, rhs: &Int<$type, BITS>) -> Option<Self> {

                // }

                #[inline]
                pub const fn checked_div(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_div(rhs.value) {
                        Some(value) => Some(Self {value }),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_div_euclid(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_div_euclid(rhs.value) {
                        Some(value) => Some(Self {value }),
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
                            if value < Self::MIN.value || value > Self::MAX.value {
                                None
                            } else {
                                Some(Self {value})
                            }
                        }
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_neg(self) -> Option<Self> {
                    match self.value.checked_neg() {
                        Some(value) => Some(Self {value}),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_pow(self, exp: u32) -> Option<Self> {
                    match self.value.checked_pow(exp) {
                        Some(value) => {
                            if value < Self::MIN.value || value > Self::MAX.value {
                                None
                            } else {
                                Some(Self {value})
                            }
                        }
                        None => None,
                    }
                }


                #[inline]
                pub const fn checked_rem(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_rem(rhs.value) {
                        Some(value) => Some(Self {value}),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_rem_euclid(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_rem_euclid(rhs.value) {
                        Some(value) => Some(Self {value}),
                        None => None,
                    }
                }

                #[inline]
                pub const fn checked_shl(self, rhs: u32) -> Option<Self> {
                    if rhs >= BITS as u32 {
                        None
                    } else {
                        Some(Self::new_wrapping(self.value << rhs))
                    }
                }

                #[inline]
                pub const fn checked_shr(self, rhs: u32) -> Option<Self> {
                    if rhs >= BITS as u32 {
                        None
                    } else {
                        Some(Self {value: self.value >> rhs})
                    }
                }

                #[inline]
                pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
                    match self.value.checked_sub(rhs.value) {
                        Some(value) => Some(Self {value}),
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
                    Self {value: self.value.div_euclid(rhs.value)}
                }

                // TODO: div_ceil

                #[inline]
                pub const fn from_be(value: Self) -> Self {
                    value.to_be()
                }


                #[inline]
                pub const fn from_le(value: Self) -> Self {
                    value.to_le()
                }

                #[inline]
                pub const fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseAIntError> {
                    let value = match $type::from_str_radix(s, radix) {
                        Ok(v) => v,
                        Err(err) => return Err(ParseAIntError::from_native(err)),
                    };

                    match value {
                        v if v < Self::MIN.value => Err(ParseAIntError{ kind: AIntErrorKind::NegOverflow }),
                        v if v > Self::MAX.value => Err(ParseAIntError{ kind: AIntErrorKind::PosOverflow }),
                        v => Ok(Self {value: v})
                    }
                }

                #[inline]
                pub const fn leading_ones(self) -> u32 {
                    let shift = ((core::mem::size_of::<$type>() << 3) - BITS);
                    (self.value << shift).leading_ones()
                }

                #[inline]
                pub const fn leading_zeros(self) -> u32 {
                    let shift = ((core::mem::size_of::<$type>() << 3) - BITS);
                    (self.value << shift).leading_zeros()
                }

                #[inline]
                pub const fn overflowing_add(self, rhs: Self) -> (Self, bool) {
                    let (value, overflow) = if core::mem::size_of::<$type>() << 3 == BITS {
                        // We are something like a AInt::<u8; 8>. We can fallback to the base implementation
                        self.value.overflowing_add(rhs.value)
                    } else {
                        // We're dealing with fewer bits than the underlying type (e.g. u7).
                        // That means the addition can never overflow the underlying type
                        let sum = self.value.wrapping_add(rhs.value);
                        let masked = sum & Self::MASK;
                        (masked, masked != sum)
                    };
                    (Self {value}, overflow)
                }

                // pub const fn overflowing_add_signed(self, rhs: Int<$type, ) -> (Self, bool) {

                // }


                #[inline]
                pub const fn overflowing_div(self, rhs: Self) -> (Self, bool) {
                    let value = self.value.wrapping_div(rhs.value);
                    (Self {value}, false)
                }

                #[inline]
                pub const fn overflowing_div_euclid(self, rhs: Self) -> (Self, bool) {
                    let value = self.value.wrapping_div_euclid(rhs.value);
                    (Self {value}, false)
                }

                #[inline]
                pub const fn overflowing_mul(self, rhs: Self) -> (Self, bool) {
                    let (wrapping_product, overflow) = if (BITS) << 1 <= (core::mem::size_of::<$type>() << 3) {
                        // We have half the bits (e.g. u4 * u4) of the base type, so we can't overflow the base type
                        // wrapping_mul likely provides the best performance on all cpus
                        self.value.overflowing_mul(rhs.value)
                    } else {
                        // We have more than half the bits (e.g. u6 * u6)
                        self.value.overflowing_mul(rhs.value)
                    };

                    let masked = wrapping_product & Self::MASK;
                    let overflow2 = masked != wrapping_product;
                    (Self {value: masked}, overflow || overflow2)
                }

                #[inline]
                pub const fn overflowing_neg(self) -> (Self, bool) {
                    let (negated, overflow) = self.value.overflowing_neg();
                    let value = negated & Self::MASK;

                    let min = 1 << (BITS - 1);
                    let is_overflow = self.value == min || overflow;

                    (Self {value}, is_overflow)
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
                    let bits = BITS as u32;
                    if rhs >= bits {
                        (
                            Self {
                                value: self.value << (rhs % bits),
                            },
                            true,
                        )
                    } else {
                        (
                            Self {value: self.value << rhs},
                            false,
                        )
                    }
                }

                #[inline]
                pub const fn overflowing_shr(self, rhs: u32) -> (Self, bool) {
                    let bits = BITS as u32;
                    if rhs >= bits {
                        (
                            Self {
                                value: self.value >> (rhs % bits),
                            },
                            true,
                        )
                    } else {
                        (
                            Self {value: self.value >> rhs },
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
                        Self::new_wrapping(value),
                        overflow,
                    )
                }

                #[inline]
                pub const fn pow(self, exp: u32) -> Self {
                    let powed = self.value.pow(exp);

                    if powed > Self::MAX.value {
                        panic!("attempt to pow with overflow");
                    }

                    Self {value:powed}
                }

                #[inline]
                pub const fn rem_euclid(self, rhs: Self) -> Self {
                    Self {value: self.value.rem_euclid(rhs.value)}
                }

                #[inline]
                pub const fn reverse_bits(self) -> Self {
                    let shift_right = (core::mem::size_of::<$type>() << 3) - (BITS);
                    Self {value: self.value.reverse_bits() >> shift_right}
                }

                #[inline]
                pub const fn rotate_left(self, n: u32) -> Self {
                    let b = Self::BITS;
                    let n = if n >= b { n % b } else { n };

                    let moved_bits = (self.value << n) & Self::MASK;
                    let truncated_bits = self.value >> (b - n);
                    Self { value: moved_bits | truncated_bits}

                }

                #[inline]
                pub const fn rotate_right(self, n: u32) -> Self {
                    let b = Self::BITS;
                    let n = if n >= b { n % b } else { n };

                    let moved_bits = self.value >> n;
                    let truncated_bits = (self.value << (b - n)) & Self::MASK;
                    Self { value: moved_bits | truncated_bits }
                }

                #[inline]
                pub const fn saturating_add(self, rhs: Self) -> Self {
                    let saturated = if (core::mem::size_of::<$type>() << 3) == BITS {
                        // We are something like a AInt::<u8; 8>. We can fallback to the base implementation
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
                    Self {value: saturated}
                }

                #[inline]
                pub const fn saturating_div(self, rhs: Self) -> Self {
                    // When dividing unsigned numbers, we never need to saturate.
                    // Divison by zero in saturating_div throws an exception (in debug and release mode),
                    // so no need to do anything special there either
                    Self {value: self.value.saturating_div(rhs.value)}
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
                    Self {value: saturated}
                }

                #[inline]
                pub const fn saturating_pow(self, exp: u32) -> Self {
                    // It might be possible to handwrite this to be slightly faster as both
                    // saturating_pow has to do a bounds-check and then we do second one
                    let powed = self.value.saturating_pow(exp);
                    let max = Self::MAX.value;
                    let saturated = if powed > max { max } else { powed };
                    Self {value: saturated}
                }

                #[inline]
                pub const fn saturating_sub(self, rhs: Self) -> Self {
                    // For unsigned numbers, the only difference is when we reach 0 - which is the same
                    // no matter the data size
                    Self {value: self.value.saturating_sub(rhs.value)}
                }

                #[inline]
                pub const fn swap_bytes(self) -> Self {
                    // swap_bytes() of the underlying type does most of the work. Then, we just need to shift
                    let amount = (core::mem::size_of::<$type>() << 3) - BITS;
                    Self {value: self.value.swap_bytes() >> amount}
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
                    Self::new_wrapping(sum)
                }

                #[inline]
                pub const fn wrapping_div(self, rhs: Self) -> Self {
                    Self {
                        // No need to mask here - divisions always produce a result that is <= self
                        value: self.value.wrapping_div(rhs.value)
                    }
                }

                #[inline]
                pub const fn wrapping_div_euclid(self, rhs: Self) -> Self {
                    Self {
                        // No need to mask here - divisions always produce a result that is <= self
                        value: self.value.wrapping_div_euclid(rhs.value)
                    }
                }

                #[inline]
                pub const fn wrapping_mul(self, rhs: Self) -> Self {
                    let value = self.value.wrapping_mul(rhs.value);
                    Self::new_wrapping(value)
                }

                #[inline]
                pub const fn wrapping_neg(self) -> Self {
                    // TODO: verify this!
                    let max = Self::MAX.value;

                    Self {value: max + 1 - (self.value - max - 1)}
                }

                #[inline]
                pub const fn wrapping_pow(self, exp: u32) -> Self {
                    // TODO: verify this!
                    let value = self.value.wrapping_pow(exp);
                    Self::new_wrapping(value)
                }

                #[inline]
                pub const fn wrapping_rem(self, rhs: Self) -> Self {
                    Self {value: self.value.wrapping_rem(rhs.value)}
                }

                #[inline]
                pub const fn wrapping_rem_euclid(self, rhs: Self) -> Self {
                    Self {value: self.value.wrapping_rem_euclid(rhs.value)}
                }

                #[inline]
                pub const fn wrapping_shl(self, rhs: u32) -> Self {
                    // modulo is expensive on some platforms, so only do it when necessary
                    let bits = BITS as u32;
                    let shift_amount = if rhs >= bits {
                        rhs % bits
                    } else {
                        rhs
                    };

                    Self {
                        // We could use wrapping_shl here to make Debug builds slightly smaller;
                        // the downside would be that on weird CPUs that don't do wrapping_shl by
                        // default release builds would get slightly worse. Using << should give
                        // good release performance everywere
                        value: (self.value << shift_amount) & Self::MASK
                    }
                }

                #[inline]
                pub const fn wrapping_shr(self, rhs: u32) -> Self {
                    // modulo is expensive on some platforms, so only do it when necessary
                    let bits = BITS as u32;
                    let shift_amount = if rhs >= bits {
                        rhs % bits
                    } else {
                        rhs
                    };

                    Self {value: self.value >> (shift_amount)}
                }

                #[inline]
                pub const fn wrapping_sub(self, rhs: Self) -> Self {
                    let sum = self.value.wrapping_sub(rhs.value);
                    Self::new_wrapping(sum)
                }


            //     #[inline]
            //     pub const fn mul_add(self, a: Self, b: Self) -> Self {
            //         Self:add(Self::mul(self, a), b)
            //     }

            }

            #[cfg(not(feature="generic_const_exprs"))]
            impl<const BITS: usize> AInt<$type, BITS>
            where
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, { <$type>::BITS as usize }>: Sized
            {
                #[inline]
                pub const fn from_be_bytes<const BYTES: usize>(from: [u8; BYTES] ) -> Self
                {
                    const { assert!(BYTES <= <Self as Number>::BYTES as usize); }

                    let mut value: $type = 0;

                    let mut bi = 0;
                    while bi < BYTES {
                        value |= (from[BYTES - 1 - bi] as $type) << (bi * 8);
                        bi += 1;
                    }
                    Self {value: value & Self::MASK}
                }

                #[inline]
                pub const fn from_le_bytes<const BYTES: usize>(from: [u8; BYTES] ) -> Self {
                    const { assert!(BYTES as u32 <= <Self as Number>::BYTES); }

                    let mut value: $type = 0;
                    let mut bx = 0;

                    while bx < BYTES {
                        value |= (from[bx] as $type) << (bx * 8);
                        bx += 1;
                    }

                    Self::new_wrapping(value)
                }

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

                // Combining this new const generic with a compile time assert is a bit
                // weird, but since we can't have associated types based on generics
                // this is the best I could come up with
                // The alternative is to implement this for every single bit size which would be extremely slow
                #[inline]
                pub const fn to_be_bytes<const BYTES: usize>(self) -> [u8; BYTES] {
                    const { assert!(BYTES <= Self::BYTES as usize) };

                    let mut ret = [0; BYTES];

                    let mut bi = 0;
                    while bi < BYTES {
                        ret[BYTES - 1 - bi] = ((self.value >> (bi * 8)) as u8 & 0xFF) as u8;
                        bi += 1;
                    }
                    ret
                }

                #[inline]
                pub const fn to_le_bytes<const BYTES: usize>(self) -> [u8; BYTES] {
                    const { assert!(BYTES <= <Self as Number>::BYTES as usize); }

                    let mut ret = [0; BYTES];

                    let mut bi = 0;
                    while bi < BYTES {
                        ret[bi] = (self.value >> (bi * 8)) as u8;
                        bi += 1;
                    }
                    ret
                }

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
            }

            #[cfg(feature="generic_const_exprs")]
            impl<const BITS: usize> AInt<$type, BITS>
            where
                $type: AIntContainer + Debug,
                ConstBounded<BITS, 1, { <$type>::BITS as usize }>: Sized
            {

                #[inline]
                pub const fn from_be_bytes(from: [u8; <AInt::<$type, BITS> as Number>::BYTES as usize] ) -> Self {
                    let mut value: $type = 0;
                    let mut bi = 0;
                    let num_bytes = Self::BYTES as usize;

                    while bi < num_bytes {
                        value |= (from[num_bytes - 1 - bi] as $type) << (bi * 8);
                        bi += 1;
                    }
                    Self::new_wrapping(value)
                }


                #[inline]
                pub const fn from_le_bytes(from: [u8; <AInt::<$type, BITS> as Number>::BYTES as usize] ) -> Self {
                    let mut value: $type = 0;

                    let mut bx = 0;
                    let num_bytes = Self::BYTES as usize;

                    while bx < num_bytes {
                        value |= (from[bx] as $type) << (bx * 8);
                        bx += 1;
                    }

                    Self::new_wrapping(value)
                }

                #[inline]
                pub const fn from_ne_bytes(from: [u8; <AInt::<$type, BITS> as Number>::BYTES as usize] ) -> Self {
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
                pub const fn to_be_bytes(self) -> [u8; <Self as Number>::BYTES as usize] {
                    let mut ret = [0; <AInt::<$type, BITS> as Number>::BYTES as usize];

                    let mut bi = 0;
                    let num_bytes = <AInt::<$type, BITS> as Number>::BYTES as usize;
                    while bi < num_bytes {
                        ret[num_bytes - 1 - bi] = ((self.value >> (bi * 8)) as u8 & 0xFF) as u8;
                        bi += 1;
                    }
                    ret
                }

                #[inline]
                pub const fn to_le_bytes(self) -> [u8; <AInt::<$type, BITS> as Number>::BYTES as usize] {
                    let mut ret = [0; Self::BYTES as usize];

                    let mut bi = 0;
                    while bi < (Self::BYTES as usize) {
                        ret[bi] = (self.value >> (bi * 8)) as u8;
                        bi += 1;
                    }
                    ret
                }

                #[inline]
                pub const fn to_ne_bytes(self) -> [u8; <AInt::<$type, BITS> as Number>::BYTES as usize] {
                    #[cfg(target_endian = "little")]
                    {
                        self.to_le_bytes()
                    }
                    #[cfg(target_endian = "big")]
                    {
                        self.to_be_bytes()
                    }
                }
            }
        )+
    }
}

aint_impl!(u8, u16, u32, u64, u128);
aint_impl!(i8, i16, i32, i64, i128);