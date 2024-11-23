extern crate core;

use std::collections::HashMap;

use std::num::ParseIntError;

use crate::*;

use crate::{AInt,};
use crate::error::{ParseNumberError, NumberErrorKind};
use crate::{Number, error::TryNewError};
use crate::aliases::*;
use paste::paste;

#[test]
fn constants() {
    // Make a constant to ensure new().value() works in a const-context
    const TEST_CONSTANT: u8 = u7::new(127).value();
    assert_eq!(TEST_CONSTANT, 127u8);

    // Same with widen()
    const TEST_CONSTANT2: u7 = u6::new(63).widen();
    assert_eq!(TEST_CONSTANT2, u7::new(63));

    // Same with widen()
    const TEST_CONSTANT3A: Result<u6, TryNewError> = u6::try_new(62);
    assert_eq!(TEST_CONSTANT3A, Ok(u6::new(62)));
    const TEST_CONSTANT3B: Result<u6, TryNewError> = u6::try_new(64);
    assert!(TEST_CONSTANT3B.is_err());
}

#[test]
fn create_simple() {
    let value7 = u7::new(123);
    let value8 = AInt::<u8, 8>::new(189);

    let value13 = u13::new(123);
    let value16 = AInt::<u16, 16>::new(60000);

    let value23 = u23::new(123);

    #[cfg(feature="128")]
    let value67 = u67::new(123);

    assert_eq!(value7.value(), 123);
    assert_eq!(value8.value(), 189);

    assert_eq!(value13.value(), 123);
    assert_eq!(value16.value(), 60000);

    assert_eq!(value23.value(), 123);

    #[cfg(feature="128")]
    assert_eq!(value67.value(), 123);
}

#[test]
fn create_try_new() {
    assert_eq!(u7::new(123).value(), 123);
    assert_eq!(u7::try_new(190).expect_err("No error seen"), TryNewError { kind: crate::NumberErrorKind::PosOverflow});
}

#[test]
#[should_panic]
fn create_panic_u7() {
    u7::new(128);
}

#[test]
#[should_panic]
fn create_panic_u15() {
    u15::new(32768);
}

#[test]
#[should_panic]
fn create_panic_u31() {
    u31::new(2147483648);
}

#[test]
#[should_panic]
fn create_panic_u63() {
    u63::new(0x8000_0000_0000_0000);
}

#[test]
#[should_panic]
fn create_panic_u127() {
    u127::new(0x8000_0000_0000_0000_0000_0000_0000_0000);
}

#[test]
fn add() {
    assert_eq!(u7::new(10) + u7::new(20), u7::new(30));
    assert_eq!(u7::new(100) + u7::new(27), u7::new(127));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn add_overflow() {
    let _ = u7::new(127) + u7::new(3);
}

#[cfg(not(debug_assertions))]
#[test]
fn add_no_overflow() {
    let _ = u7::new(127) + u7::new(3);
}

#[cfg(feature = "num-traits")]
#[test]
fn num_traits_add_wrapping() {
    let v1 = u7::new(120);
    let v2 = u7::new(10);
    let v3 = num_traits::WrappingAdd::wrapping_add(&v1, &v2);
    assert_eq!(v3, u7::new(2));
}

#[cfg(feature = "num-traits")]
#[test]
fn num_traits_sub_wrapping() {
    let v1 = u7::new(15);
    let v2 = u7::new(20);
    let v3 = num_traits::WrappingSub::wrapping_sub(&v1, &v2);
    assert_eq!(v3, u7::new(123));
}

#[cfg(feature = "num-traits")]
#[test]
fn num_traits_bounded() {
    use num_traits::bounds::Bounded;
    assert_eq!(u7::MAX, u7::max_value());

    #[cfg(feature="128")]
    assert_eq!(u119::MAX, u119::max_value());
    assert_eq!(u7::new(0), u7::min_value());

    #[cfg(feature="128")]
    assert_eq!(u119::new(0), u119::min_value());
}

#[test]
fn addassign() {
    let mut value = u9::new(500);
    value += u9::new(11);
    assert_eq!(value, u9::new(511));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn addassign_overflow() {
    let mut value = u9::new(500);
    value += u9::new(40);
}

#[cfg(not(debug_assertions))]
#[test]
fn addassign_no_overflow() {
    let mut value = u9::new(500);
    value += u9::new(28);
    assert_eq!(value, u9::new(16));
}

#[test]
fn sub() {
    assert_eq!(u7::new(22) - u7::new(10), u7::new(12));
    assert_eq!(u7::new(127) - u7::new(127), u7::new(0));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn sub_overflow() {
    let _ = u7::new(100) - u7::new(127);
}

#[cfg(not(debug_assertions))]
#[test]
fn sub_no_overflow() {
    let value = u7::new(100) - u7::new(127);
    assert_eq!(value, u7::new(101));
}

#[test]
fn subassign() {
    let mut value = u9::new(500);
    value -= u9::new(11);
    assert_eq!(value, u9::new(489));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn subassign_overflow() {
    let mut value = u9::new(30);
    value -= u9::new(40);
}

#[cfg(not(debug_assertions))]
#[test]
fn subassign_no_overflow() {
    let mut value = u9::new(30);
    value -= u9::new(40);
    assert_eq!(value, u9::new(502));
}

#[test]
fn mul() {
    assert_eq!(u7::new(22) * u7::new(4), u7::new(88));
    assert_eq!(u7::new(127) * u7::new(0), u7::new(0));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn mul_overflow() {
    let _ = u7::new(100) * u7::new(2);
}

#[cfg(not(debug_assertions))]
#[test]
fn mul_no_overflow() {
    let result = u7::new(100) * u7::new(2);
    assert_eq!(result, u7::new(72));
}

#[test]
fn mulassign() {
    let mut value = u9::new(240);
    value *= u9::new(2);
    assert_eq!(value, u9::new(480));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn mulassign_overflow() {
    let mut value = u9::new(500);
    value *= u9::new(2);
}

#[cfg(not(debug_assertions))]
#[test]
fn mulassign_no_overflow() {
    let mut value = u9::new(500);
    value *= u9::new(40);
    assert_eq!(value, u9::new(32));
}

#[test]
fn div() {
    // div just forwards to the underlying type, so there isn't much to do
    assert_eq!(u7::new(22) / u7::new(4), u7::new(5));
    assert_eq!(u7::new(127) / u7::new(1), u7::new(127));
    assert_eq!(u7::new(127) / u7::new(127), u7::new(1));
}

#[should_panic]
#[test]
fn div_by_zero() {
    let _ = u7::new(22) / u7::new(0);
}

#[test]
fn divassign() {
    let mut value = u9::new(240);
    value /= u9::new(2);
    assert_eq!(value, u9::new(120));
}

#[should_panic]
#[test]
fn divassign_by_zero() {
    let mut value = u9::new(240);
    value /= u9::new(0);
}

#[test]
fn bitand() {
    assert_eq!(
        u17::new(0b11001100) & u17::new(0b01101001),
        u17::new(0b01001000)
    );
    assert_eq!(u17::new(0b11001100) & u17::new(0), u17::new(0));
    assert_eq!(
        u17::new(0b11001100) & u17::new(0x1_FFFF),
        u17::new(0b11001100)
    );
}

#[test]
fn bitandassign() {
    let mut value = u4::new(0b0101);
    value &= u4::new(0b0110);
    assert_eq!(value, u4::new(0b0100));
}

#[test]
fn bitor() {
    assert_eq!(
        u17::new(0b11001100) | u17::new(0b01101001),
        u17::new(0b11101101)
    );
    assert_eq!(u17::new(0b11001100) | u17::new(0), u17::new(0b11001100));
    assert_eq!(
        u17::new(0b11001100) | u17::new(0x1_FFFF),
        u17::new(0x1_FFFF)
    );
}

#[test]
fn bitorassign() {
    let mut value = u4::new(0b0101);
    value |= u4::new(0b0110);
    assert_eq!(value, u4::new(0b0111));
}

#[test]
fn bitxor() {
    assert_eq!(
        u17::new(0b11001100) ^ u17::new(0b01101001),
        u17::new(0b10100101)
    );
    assert_eq!(u17::new(0b11001100) ^ u17::new(0), u17::new(0b11001100));
    assert_eq!(
        u17::new(0b11001100) ^ u17::new(0x1_FFFF),
        u17::new(0b1_11111111_00110011)
    );
}

#[test]
fn bitxorassign() {
    let mut value = u4::new(0b0101);
    value ^= u4::new(0b0110);
    assert_eq!(value, u4::new(0b0011));
}

#[test]
fn not() {
    assert_eq!(!u17::new(0), u17::new(0b1_11111111_11111111));
    assert_eq!(!u5::new(0b10101), u5::new(0b01010));
}

#[test]
fn shl() {
    assert_eq!(u17::new(0b1) << 5u8, u17::new(0b100000));
    // Ensure bits on the left are shifted out
    assert_eq!(u9::new(0b11110000) << 3u64, u9::new(0b1_10000000));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn shl_too_much8() {
    let _ = u53::new(123) << 53u8;
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn shl_too_much16() {
    let _ = u53::new(123) << 53u16;
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn shl_too_much32() {
    let _ = u53::new(123) << 53u32;
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn shl_too_much64() {
    let _ = u53::new(123) << 53u64;
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn shl_too_much128() {
    let _ = u53::new(123) << 53u128;
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn shl_too_much_usize() {
    let _ = u53::new(123) << 53usize;
}

#[test]
fn shlassign() {
    let mut value = u9::new(0b11110000);
    value <<= 3;
    assert_eq!(value, u9::new(0b1_10000000));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn shlassign_too_much() {
    let mut value = u9::new(0b11110000);
    value <<= 9;
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn shlassign_too_much2() {
    let mut value = u9::new(0b11110000);
    value <<= 10;
}

#[test]
fn shr() {
    assert_eq!(u17::new(0b100110) >> 5usize, u17::new(1));

    // Ensure there's no sign extension
    assert_eq!(u17::new(0b1_11111111_11111111) >> 8, u17::new(0b1_11111111));
}

#[test]
fn shrassign() {
    let mut value = u9::new(0b1_11110000);
    value >>= 6;
    assert_eq!(value, u9::new(0b0_00000111));
}

#[test]
fn compare() {
    assert_eq!(true, u4::new(0b1100) > u4::new(0b0011));
    assert_eq!(true, u4::new(0b1100) >= u4::new(0b0011));
    assert_eq!(false, u4::new(0b1100) < u4::new(0b0011));
    assert_eq!(false, u4::new(0b1100) <= u4::new(0b0011));
    assert_eq!(true, u4::new(0b1100) != u4::new(0b0011));
    assert_eq!(false, u4::new(0b1100) == u4::new(0b0011));

    assert_eq!(false, u4::new(0b1100) > u4::new(0b1100));
    assert_eq!(true, u4::new(0b1100) >= u4::new(0b1100));
    assert_eq!(false, u4::new(0b1100) < u4::new(0b1100));
    assert_eq!(true, u4::new(0b1100) <= u4::new(0b1100));
    assert_eq!(false, u4::new(0b1100) != u4::new(0b1100));
    assert_eq!(true, u4::new(0b1100) == u4::new(0b1100));

    assert_eq!(false, u4::new(0b0011) > u4::new(0b1100));
    assert_eq!(false, u4::new(0b0011) >= u4::new(0b1100));
    assert_eq!(true, u4::new(0b0011) < u4::new(0b1100));
    assert_eq!(true, u4::new(0b0011) <= u4::new(0b1100));
    assert_eq!(true, u4::new(0b0011) != u4::new(0b1100));
    assert_eq!(false, u4::new(0b0011) == u4::new(0b1100));
}

#[test]
fn min_max() {
    assert_eq!(0, u4::MIN.value());
    assert_eq!(0b1111, u4::MAX.value());
    assert_eq!(u4::new(0b1111), u4::MAX);

    assert_eq!(0, u15::MIN.value());
    assert_eq!(32767, u15::MAX.value());
    assert_eq!(u15::new(32767), u15::MAX);

    assert_eq!(0, u31::MIN.value());
    assert_eq!(2147483647, u31::MAX.value());

    assert_eq!(0, u63::MIN.value());
    assert_eq!(0x7FFF_FFFF_FFFF_FFFF, u63::MAX.value());


    #[cfg(feature="128")]
    assert_eq!(0, u127::MIN.value());

    #[cfg(feature="128")]
    assert_eq!(0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF, u127::MAX.value());
}

#[test]
fn bits() {
    assert_eq!(4, u4::BITS);
    assert_eq!(12, u12::BITS);
    assert_eq!(120, u120::BITS);
    assert_eq!(13, AInt::<u128, 13usize>::BITS);

    assert_eq!(8, u8::BITS);
    assert_eq!(16, u16::BITS);
}

#[test]
fn mask() {
    assert_eq!(0x1u8, u1::MASK);
    assert_eq!(0xFu8, u4::MASK);
    assert_eq!(0x3FFFFu32, u18::MASK);

    #[cfg(feature="128")]
    assert_eq!(0x7FFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_u128, u127::MASK);

    #[cfg(feature="128")]
    assert_eq!(0x7FFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_u128, u127::MASK);

    #[cfg(feature="128")]
    assert_eq!(0xFFFFFFFF_FFFFFFFF_FFFFFFFF_FFFFFFFF_u128, u128::MAX);
}

#[test]
fn min_max_fullwidth() {
    assert_eq!(u8::MIN, AInt::<u8, 8>::MIN.value());
    assert_eq!(u8::MAX, AInt::<u8, 8>::MAX.value());

    assert_eq!(u16::MIN, AInt::<u16, 16>::MIN.value());
    assert_eq!(u16::MAX, AInt::<u16, 16>::MAX.value());

    assert_eq!(u32::MIN, AInt::<u32, 32>::MIN.value());
    assert_eq!(u32::MAX, AInt::<u32, 32>::MAX.value());

    assert_eq!(u64::MIN, AInt::<u64, 64>::MIN.value());
    assert_eq!(u64::MAX, AInt::<u64, 64>::MAX.value());

    #[cfg(feature="128")]
    assert_eq!(u128::MIN, AInt::<u128, 128>::MIN.value());

    #[cfg(feature="128")]
    assert_eq!(u128::MAX, AInt::<u128, 128>::MAX.value());
}

#[allow(deprecated)]
#[test]
fn extract() {
    assert_eq!(u5::new(0b10000), u5::extract(0b11110000_u8, 0));
    assert_eq!(u5::new(0b11100), u5::extract(0b11110000_u8, 2));
    assert_eq!(u5::new(0b11110), u5::extract(0b11110000_u8, 3));

    // Use extract with a custom type (5 bits of u32)
    assert_eq!(
        AInt::<u32, 5>::new(0b11110),
        AInt::<u32, 5>::extract(0b11110000u32, 3)
    );
    assert_eq!(
        u5::new(0b11110),
        AInt::<u32, 5>::extract(0b11110000u32, 3).into()
    );
}

#[test]
fn extract_typed() {
    assert_eq!(u5::new(0b10000), u5::extract(0b11110000_u8, 0));
    assert_eq!(u5::new(0b00011), u5::extract(0b11110000_11110110_u16, 6));
    assert_eq!(
        u5::new(0b01011),
        u5::extract(0b11110010_11110110_00000000_00000000_u32, 22)
    );
    assert_eq!(
        u5::new(0b01011),
        u5::extract(
            0b11110010_11110110_00000000_00000000_00000000_00000000_00000000_00000000_u64,
            54
        )
    );

    #[cfg(feature="128")]
    assert_eq!(u5::new(0b01011), u5::extract(0b11110010_11110110_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_u128, 118));
}

#[test]
fn extract_full_width_typed() {
    assert_eq!(
        0b1010_0011,
        AInt::<u8, 8>::extract(0b1010_0011_u8, 0).value()
    );
    assert_eq!(
        0b1010_0011,
        AInt::<u8, 8>::extract(0b1111_1111_1010_0011_u16, 0).value()
    );
}

#[test]
#[should_panic]
fn extract_not_enough_bits_8() {
    let _ = u5::extract(0b11110000u8, 4);
}

#[test]
#[should_panic]
fn extract_not_enough_bits_8_full_width() {
    let _ = AInt::<u8, 8>::extract(0b11110000u8, 1);
}

#[test]
#[should_panic]
fn extract_not_enough_bits_16() {
    let _ = u5::extract(0b11110000u8, 12);
}

#[test]
#[should_panic]
fn extract_not_enough_bits_32() {
    let _ = u5::extract(0b11110000u8, 28);
}

#[test]
#[should_panic]
fn extract_not_enough_bits_64() {
    let _ = u5::extract(0b11110000u8, 60);
}

#[cfg(feature="128")]
#[test]
#[should_panic]
fn extract_not_enough_bits_128() {
    let _ = u5::extract(0b11110000u8, 124);
}

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

    #[cfg(feature="128")]
    assert_eq!(
        AInt::<u128, 5>::from(AInt::<u8, 5>::new(0b10101)),
        AInt::<u128, 5>::new(0b10101)
    );
    #[cfg(feature="128")]
    assert_eq!(
        AInt::<u128, 12>::from(AInt::<u16, 12>::new(0b10101)),
        AInt::<u128, 12>::new(0b10101)
    );
    #[cfg(feature="128")]
    assert_eq!(
        AInt::<u128, 26>::from(AInt::<u32, 26>::new(0b10101)),
        AInt::<u128, 26>::new(0b10101)
    );
    #[cfg(feature="128")]
    assert_eq!(
        AInt::<u128, 60>::from(AInt::<u64, 60>::new(0b10101)),
        AInt::<u128, 60>::new(0b10101)
    );

    #[cfg(feature="128")]
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

    #[cfg(feature="128")]
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

    #[cfg(feature="128")]
    type u128 = AInt<primitive::u128, 128>;

    assert_eq!(u8::from(0x80_u8), u8::new(0x80));
    assert_eq!(u16::from(0x8000_u16), u16::new(0x8000));
    assert_eq!(u32::from(0x8000_0000_u32), u32::new(0x8000_0000));
    assert_eq!(
        u64::from(0x8000_0000_0000_0000_u64),
        u64::new(0x8000_0000_0000_0000)
    );

    #[cfg(feature="128")]
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

    #[cfg(feature="128")]
    assert_eq!(u65::from(0x80_u8), u65::new(0x80));

    #[cfg(feature="128")]
    assert_eq!(u65::from(0x8000_u16), u65::new(0x8000));

    #[cfg(feature="128")]
    assert_eq!(u65::from(0x8000_0000_u32), u65::new(0x8000_0000));

    #[cfg(feature="128")]
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

    #[cfg(feature="128")]
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

    #[cfg(feature="128")]
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

#[test]
fn widen() {
    // As From() can't be used while keeping the base-data-type, there's widen

    assert_eq!(u5::new(0b11011).widen::<6>(), u6::new(0b11011));
    assert_eq!(u5::new(0b11011).widen::<8>(), AInt::<u8, 8>::new(0b11011));
    assert_eq!(u10::new(0b11011).widen::<11>(), u11::new(0b11011));
    assert_eq!(u20::new(0b11011).widen::<24>(), u24::new(0b11011));
    assert_eq!(u60::new(0b11011).widen::<61>(), u61::new(0b11011));
    assert_eq!(u80::new(0b11011).widen::<127>().value(), 0b11011);
}

#[test]
fn to_string() {
    assert_eq!("Value: 5", format!("Value: {}", 5u32.to_string()));
    assert_eq!("Value: 5", format!("Value: {}", u5::new(5).to_string()));
    assert_eq!("Value: 5", format!("Value: {}", u11::new(5).to_string()));
    assert_eq!("Value: 5", format!("Value: {}", u17::new(5).to_string()));
    assert_eq!("Value: 5", format!("Value: {}", u38::new(5).to_string()));
    assert_eq!("Value: 60", format!("Value: {}", u65::new(60).to_string()));
}

#[test]
fn display() {
    assert_eq!("Value: 5", format!("Value: {}", 5u32));
    assert_eq!("Value: 5", format!("Value: {}", u5::new(5)));
    assert_eq!("Value: 5", format!("Value: {}", u11::new(5)));
    assert_eq!("Value: 5", format!("Value: {}", u17::new(5)));
    assert_eq!("Value: 5", format!("Value: {}", u38::new(5)));
    assert_eq!("Value: 60", format!("Value: {}", u65::new(60)));
}

#[test]
fn debug() {
    assert_eq!("Value: 5", format!("Value: {:?}", 5u32));
    assert_eq!("Value: 5", format!("Value: {:?}", u5::new(5)));
    assert_eq!("Value: 5", format!("Value: {:?}", u11::new(5)));
    assert_eq!("Value: 5", format!("Value: {:?}", u17::new(5)));
    assert_eq!("Value: 5", format!("Value: {:?}", u38::new(5)));
    assert_eq!("Value: 60", format!("Value: {:?}", u65::new(60)));
}

#[test]
fn lower_hex() {
    assert_eq!("Value: a", format!("Value: {:x}", 10u32));
    assert_eq!("Value: a", format!("Value: {:x}", u5::new(10)));
    assert_eq!("Value: a", format!("Value: {:x}", u11::new(10)));
    assert_eq!("Value: a", format!("Value: {:x}", u17::new(10)));
    assert_eq!("Value: a", format!("Value: {:x}", u38::new(10)));
    assert_eq!("Value: 3c", format!("Value: {:x}", 60));
    assert_eq!("Value: 3c", format!("Value: {:x}", u65::new(60)));
}

#[test]
fn upper_hex() {
    assert_eq!("Value: A", format!("Value: {:X}", 10u32));
    assert_eq!("Value: A", format!("Value: {:X}", u5::new(10)));
    assert_eq!("Value: A", format!("Value: {:X}", u11::new(10)));
    assert_eq!("Value: A", format!("Value: {:X}", u17::new(10)));
    assert_eq!("Value: A", format!("Value: {:X}", u38::new(10)));
    assert_eq!("Value: 3C", format!("Value: {:X}", 60));
    assert_eq!("Value: 3C", format!("Value: {:X}", u65::new(60)));
}

#[test]
fn lower_hex_fancy() {
    assert_eq!("Value: 0xa", format!("Value: {:#x}", 10u32));
    assert_eq!("Value: 0xa", format!("Value: {:#x}", u5::new(10)));
    assert_eq!("Value: 0xa", format!("Value: {:#x}", u11::new(10)));
    assert_eq!("Value: 0xa", format!("Value: {:#x}", u17::new(10)));
    assert_eq!("Value: 0xa", format!("Value: {:#x}", u38::new(10)));
    assert_eq!("Value: 0x3c", format!("Value: {:#x}", 60));
    assert_eq!("Value: 0x3c", format!("Value: {:#x}", u65::new(60)));
}

#[test]
fn upper_hex_fancy() {
    assert_eq!("Value: 0xA", format!("Value: {:#X}", 10u32));
    assert_eq!("Value: 0xA", format!("Value: {:#X}", u5::new(10)));
    assert_eq!("Value: 0xA", format!("Value: {:#X}", u11::new(10)));
    assert_eq!("Value: 0xA", format!("Value: {:#X}", u17::new(10)));
    assert_eq!("Value: 0xA", format!("Value: {:#X}", u38::new(10)));
    assert_eq!("Value: 0x3C", format!("Value: {:#X}", 60));
    assert_eq!("Value: 0x3C", format!("Value: {:#X}", u65::new(60)));
}

#[test]
fn debug_lower_hex_fancy() {
    assert_eq!("Value: 0xa", format!("Value: {:#x?}", 10u32));
    assert_eq!("Value: 0xa", format!("Value: {:#x?}", u5::new(10)));
    assert_eq!("Value: 0xa", format!("Value: {:#x?}", u11::new(10)));
    assert_eq!("Value: 0xa", format!("Value: {:#x?}", u17::new(10)));
    assert_eq!("Value: 0xa", format!("Value: {:#x?}", u38::new(10)));
    assert_eq!("Value: 0x3c", format!("Value: {:#x?}", 60));
    assert_eq!("Value: 0x3c", format!("Value: {:#x?}", u65::new(60)));
}

#[test]
fn debug_upper_hex_fancy() {
    assert_eq!("Value: 0xA", format!("Value: {:#X?}", 10u32));
    assert_eq!("Value: 0xA", format!("Value: {:#X?}", u5::new(10)));
    assert_eq!("Value: 0xA", format!("Value: {:#X?}", u11::new(10)));
    assert_eq!("Value: 0xA", format!("Value: {:#X?}", u17::new(10)));
    assert_eq!("Value: 0xA", format!("Value: {:#X?}", u38::new(10)));
    assert_eq!("Value: 0x3C", format!("Value: {:#X?}", 60));
    assert_eq!("Value: 0x3C", format!("Value: {:#X?}", u65::new(60)));
}

#[test]
fn octal() {
    assert_eq!("Value: 12", format!("Value: {:o}", 10u32));
    assert_eq!("Value: 12", format!("Value: {:o}", u5::new(10)));
    assert_eq!("Value: 12", format!("Value: {:o}", u11::new(10)));
    assert_eq!("Value: 12", format!("Value: {:o}", u17::new(10)));
    assert_eq!("Value: 12", format!("Value: {:o}", u38::new(10)));
    assert_eq!("Value: 74", format!("Value: {:o}", 0o74));
    assert_eq!("Value: 74", format!("Value: {:o}", u65::new(0o74)));
}

#[test]
fn binary() {
    assert_eq!("Value: 1010", format!("Value: {:b}", 10u32));
    assert_eq!("Value: 1010", format!("Value: {:b}", u5::new(10)));
    assert_eq!("Value: 1010", format!("Value: {:b}", u11::new(10)));
    assert_eq!("Value: 1010", format!("Value: {:b}", u17::new(10)));
    assert_eq!("Value: 1010", format!("Value: {:b}", u38::new(10)));
    assert_eq!("Value: 111100", format!("Value: {:b}", 0b111100));
    assert_eq!("Value: 111100", format!("Value: {:b}", u65::new(0b111100)));
}

#[test]
fn hash() {
    let mut hashmap = HashMap::<u5, u7>::new();

    hashmap.insert(u5::new(11), u7::new(9));

    assert_eq!(Some(&u7::new(9)), hashmap.get(&u5::new(11)));
    assert_eq!(None, hashmap.get(&u5::new(12)));
}

#[test]
fn swap_bytes() {
    assert_eq!(u24::new(0x12_34_56).swap_bytes(), u24::new(0x56_34_12));
    assert_eq!(
        AInt::<u64, 24>::new(0x12_34_56).swap_bytes(),
        AInt::<u64, 24>::new(0x56_34_12)
    );
    assert_eq!(
        AInt::<u128, 24>::new(0x12_34_56).swap_bytes(),
        AInt::<u128, 24>::new(0x56_34_12)
    );

    assert_eq!(
        u40::new(0x12_34_56_78_9A).swap_bytes(),
        u40::new(0x9A_78_56_34_12)
    );
    assert_eq!(
        AInt::<u128, 40>::new(0x12_34_56_78_9A).swap_bytes(),
        AInt::<u128, 40>::new(0x9A_78_56_34_12)
    );

    assert_eq!(
        u48::new(0x12_34_56_78_9A_BC).swap_bytes(),
        u48::new(0xBC_9A_78_56_34_12)
    );
    assert_eq!(
        AInt::<u128, 48>::new(0x12_34_56_78_9A_BC).swap_bytes(),
        AInt::<u128, 48>::new(0xBC_9A_78_56_34_12)
    );

    assert_eq!(
        u56::new(0x12_34_56_78_9A_BC_DE).swap_bytes(),
        u56::new(0xDE_BC_9A_78_56_34_12)
    );
    assert_eq!(
        AInt::<u128, 56>::new(0x12_34_56_78_9A_BC_DE).swap_bytes(),
        AInt::<u128, 56>::new(0xDE_BC_9A_78_56_34_12)
    );

    assert_eq!(
        u72::new(0x12_34_56_78_9A_BC_DE_FE_DC).swap_bytes(),
        u72::new(0xDC_FE_DE_BC_9A_78_56_34_12)
    );

    assert_eq!(
        u80::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA).swap_bytes(),
        u80::new(0xBA_DC_FE_DE_BC_9A_78_56_34_12)
    );

    assert_eq!(
        u88::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98).swap_bytes(),
        u88::new(0x98_BA_DC_FE_DE_BC_9A_78_56_34_12)
    );

    assert_eq!(
        u96::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76).swap_bytes(),
        u96::new(0x76_98_BA_DC_FE_DE_BC_9A_78_56_34_12)
    );

    assert_eq!(
        u104::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54).swap_bytes(),
        u104::new(0x54_76_98_BA_DC_FE_DE_BC_9A_78_56_34_12)
    );

    assert_eq!(
        u112::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32).swap_bytes(),
        u112::new(0x32_54_76_98_BA_DC_FE_DE_BC_9A_78_56_34_12)
    );

    assert_eq!(
        u120::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32_10).swap_bytes(),
        u120::new(0x10_32_54_76_98_BA_DC_FE_DE_BC_9A_78_56_34_12)
    );
}

#[test]
fn to_le_and_be_bytes() {
    assert_eq!(u24::new(0x12_34_56).to_le_bytes(), [0x56, 0x34, 0x12]);
    assert_eq!(
        AInt::<u64, 24>::new(0x12_34_56).to_le_bytes(),
        [0x56, 0x34, 0x12]
    );
    assert_eq!(
        AInt::<u128, 24>::new(0x12_34_56).to_le_bytes(),
        [0x56, 0x34, 0x12]
    );

    assert_eq!(u24::new(0x12_34_56).to_be_bytes(), [0x12, 0x34, 0x56]);


    assert_eq!(u24::new(0x12_34_56).to_be_bytes(), [0x12, 0x34, 0x56]);

 /*    let v = u24::new(0x12_34_56);
    let b  = v.to_be_bytes::<1>();



 */

    assert_eq!(
        AInt::<u64, 24>::new(0x12_34_56).to_be_bytes(),
        [0x12, 0x34, 0x56]
    );
    assert_eq!(
        AInt::<u128, 24>::new(0x12_34_56).to_be_bytes(),
        [0x12, 0x34, 0x56]
    );

    assert_eq!(
        u40::new(0x12_34_56_78_9A).to_le_bytes(),
        [0x9A, 0x78, 0x56, 0x34, 0x12]
    );
    assert_eq!(
        AInt::<u128, 40>::new(0x12_34_56_78_9A).to_le_bytes(),
        [0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u40::new(0x12_34_56_78_9A).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A]
    );
    assert_eq!(
        AInt::<u128, 40>::new(0x12_34_56_78_9A).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A]
    );

    assert_eq!(
        u48::new(0x12_34_56_78_9A_BC).to_le_bytes(),
        [0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );
    assert_eq!(
        AInt::<u128, 48>::new(0x12_34_56_78_9A_BC).to_le_bytes(),
        [0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u48::new(0x12_34_56_78_9A_BC).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]
    );
    assert_eq!(
        AInt::<u128, 48>::new(0x12_34_56_78_9A_BC).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]
    );

    assert_eq!(
        u56::new(0x12_34_56_78_9A_BC_DE).to_le_bytes(),
        [0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );
    assert_eq!(
        AInt::<u128, 56>::new(0x12_34_56_78_9A_BC_DE).to_le_bytes(),
        [0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u56::new(0x12_34_56_78_9A_BC_DE).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE]
    );
    assert_eq!(
        AInt::<u128, 56>::new(0x12_34_56_78_9A_BC_DE).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE]
    );

    assert_eq!(
        u72::new(0x12_34_56_78_9A_BC_DE_FE_DC).to_le_bytes(),
        [0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u72::new(0x12_34_56_78_9A_BC_DE_FE_DC).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC]
    );

    assert_eq!(
        u80::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA).to_le_bytes(),
        [0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u80::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA]
    );

    assert_eq!(
        u88::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98).to_le_bytes(),
        [0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u88::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98]
    );

    assert_eq!(
        u96::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76).to_le_bytes(),
        [0x76, 0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u96::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98, 0x76]
    );

    assert_eq!(
        u104::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54).to_le_bytes(),
        [0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u104::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54]
    );

    assert_eq!(
        u112::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32).to_le_bytes(),
        [0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]
    );

    assert_eq!(
        u112::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32).to_be_bytes(),
        [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32]
    );

    assert_eq!(
        u120::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32_10).to_le_bytes(),
        [
            0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34,
            0x12
        ]
    );

    assert_eq!(
        u120::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32_10).to_be_bytes(),
        [
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
            0x10
        ]
    );
}

#[test]
fn from_le_and_be_bytes() {
    assert_eq!(u24::from_le_bytes([0x56, 0x34, 0x12]), u24::new(0x12_34_56));
    assert_eq!(
        AInt::<u64, 24>::from_le_bytes([0x56, 0x34, 0x12]),
        AInt::<u64, 24>::new(0x12_34_56)
    );
    assert_eq!(
        AInt::<u128, 24>::from_le_bytes([0x56, 0x34, 0x12]),
        AInt::<u128, 24>::new(0x12_34_56)
    );

    assert_eq!(u24::from_be_bytes([0x12, 0x34, 0x56]), u24::new(0x12_34_56));
    assert_eq!(
        AInt::<u64, 24>::from_be_bytes([0x12, 0x34, 0x56]),
        AInt::<u64, 24>::new(0x12_34_56)
    );
    assert_eq!(
        AInt::<u128, 24>::from_be_bytes([0x12, 0x34, 0x56]),
        AInt::<u128, 24>::new(0x12_34_56)
    );

    assert_eq!(
        u40::from_le_bytes([0x9A, 0x78, 0x56, 0x34, 0x12]),
        u40::new(0x12_34_56_78_9A)
    );
    assert_eq!(
        AInt::<u128, 40>::from_le_bytes([0x9A, 0x78, 0x56, 0x34, 0x12]),
        AInt::<u128, 40>::new(0x12_34_56_78_9A)
    );

    assert_eq!(
        u40::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A]),
        u40::new(0x12_34_56_78_9A)
    );
    assert_eq!(
        AInt::<u128, 40>::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A]),
        AInt::<u128, 40>::new(0x12_34_56_78_9A)
    );

    assert_eq!(
        u48::from_le_bytes([0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]),
        u48::new(0x12_34_56_78_9A_BC)
    );
    assert_eq!(
        AInt::<u128, 48>::from_le_bytes([0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]),
        AInt::<u128, 48>::new(0x12_34_56_78_9A_BC)
    );

    assert_eq!(
        u48::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]),
        u48::new(0x12_34_56_78_9A_BC)
    );
    assert_eq!(
        AInt::<u128, 48>::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC]),
        AInt::<u128, 48>::new(0x12_34_56_78_9A_BC)
    );

    assert_eq!(
        u56::from_le_bytes([0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]),
        u56::new(0x12_34_56_78_9A_BC_DE)
    );
    assert_eq!(
        AInt::<u128, 56>::from_le_bytes([0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]),
        AInt::<u128, 56>::new(0x12_34_56_78_9A_BC_DE)
    );

    assert_eq!(
        u56::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE]),
        u56::new(0x12_34_56_78_9A_BC_DE)
    );
    assert_eq!(
        AInt::<u128, 56>::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE]),
        AInt::<u128, 56>::new(0x12_34_56_78_9A_BC_DE)
    );

    assert_eq!(
        u72::from_le_bytes([0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]),
        u72::new(0x12_34_56_78_9A_BC_DE_FE_DC)
    );

    assert_eq!(
        u72::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC]),
        u72::new(0x12_34_56_78_9A_BC_DE_FE_DC)
    );

    assert_eq!(
        u80::from_le_bytes([0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]),
        u80::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA)
    );

    assert_eq!(
        u80::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA]),
        u80::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA)
    );

    assert_eq!(
        u88::from_le_bytes([0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12]),
        u88::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98)
    );

    assert_eq!(
        u88::from_be_bytes([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98]),
        u88::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98)
    );

    assert_eq!(
        u96::from_le_bytes([
            0x76, 0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12
        ]),
        u96::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76)
    );

    assert_eq!(
        u96::from_be_bytes([
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98, 0x76
        ]),
        u96::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76)
    );

    assert_eq!(
        u104::from_le_bytes([
            0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12
        ]),
        u104::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54)
    );

    assert_eq!(
        u104::from_be_bytes([
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54
        ]),
        u104::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54)
    );

    assert_eq!(
        u112::from_le_bytes([
            0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12
        ]),
        u112::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32)
    );

    assert_eq!(
        u112::from_be_bytes([
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32
        ]),
        u112::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32)
    );

    assert_eq!(
        u120::from_le_bytes([
            0x10, 0x32, 0x54, 0x76, 0x98, 0xBA, 0xDC, 0xFE, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34,
            0x12
        ]),
        u120::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32_10)
    );

    assert_eq!(
        u120::from_be_bytes([
            0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32,
            0x10
        ]),
        u120::new(0x12_34_56_78_9A_BC_DE_FE_DC_BA_98_76_54_32_10)
    );
}

#[test]
fn to_ne_bytes() {
    if cfg!(target_endian = "little") {
        assert_eq!(
            u40::new(0x12_34_56_78_9A).to_ne_bytes(),
            [0x9A, 0x78, 0x56, 0x34, 0x12]
        );
    } else {
        assert_eq!(
            u40::new(0x12_34_56_78_9A).to_ne_bytes(),
            [0x12, 0x34, 0x56, 0x78, 0x9A]
        );
    }
}

#[test]
fn from_ne_bytes() {
    if cfg!(target_endian = "little") {
        assert_eq!(
            u40::from_ne_bytes([0x9A, 0x78, 0x56, 0x34, 0x12]),
            u40::new(0x12_34_56_78_9A)
        );
    } else {
        assert_eq!(
            u40::from_ne_bytes([0x12, 0x34, 0x56, 0x78, 0x9A]),
            u40::new(0x12_34_56_78_9A)
        );
    }
}

#[test]
fn simple_le_be() {
    const REGULAR: u40 = u40::new(0x12_34_56_78_9A);
    const SWAPPED: u40 = u40::new(0x9A_78_56_34_12);
    if cfg!(target_endian = "little") {
        assert_eq!(REGULAR.to_le(), REGULAR);
        assert_eq!(REGULAR.to_be(), SWAPPED);
        assert_eq!(u40::from_le(REGULAR), REGULAR);
        assert_eq!(u40::from_be(REGULAR), SWAPPED);
    } else {
        assert_eq!(REGULAR.to_le(), SWAPPED);
        assert_eq!(REGULAR.to_be(), REGULAR);
        assert_eq!(u40::from_le(REGULAR), SWAPPED);
        assert_eq!(u40::from_be(REGULAR), REGULAR);
    }
}

#[test]
fn wrapping_add() {
    assert_eq!(u7::new(120).wrapping_add(u7::new(1)), u7::new(121));
    assert_eq!(u7::new(120).wrapping_add(u7::new(10)), u7::new(2));
    assert_eq!(u7::new(127).wrapping_add(u7::new(127)), u7::new(126));
}

#[test]
fn wrapping_sub() {
    assert_eq!(u7::new(120).wrapping_sub(u7::new(1)), u7::new(119));
    assert_eq!(u7::new(10).wrapping_sub(u7::new(20)), u7::new(118));
    assert_eq!(u7::new(0).wrapping_sub(u7::new(1)), u7::new(127));
}

#[test]
fn wrapping_mul() {
    assert_eq!(u7::new(120).wrapping_mul(u7::new(0)), u7::new(0));
    assert_eq!(u7::new(120).wrapping_mul(u7::new(1)), u7::new(120));

    // Overflow u7
    assert_eq!(u7::new(120).wrapping_mul(u7::new(2)), u7::new(112));

    // Overflow the underlying type
    assert_eq!(u7::new(120).wrapping_mul(u7::new(3)), u7::new(104));
}

#[test]
fn wrapping_div() {
    assert_eq!(u7::new(120).wrapping_div(u7::new(1)), u7::new(120));
    assert_eq!(u7::new(120).wrapping_div(u7::new(2)), u7::new(60));
    assert_eq!(u7::new(120).wrapping_div(u7::new(120)), u7::new(1));
    assert_eq!(u7::new(120).wrapping_div(u7::new(121)), u7::new(0));
}

#[should_panic]
#[test]
fn wrapping_div_by_zero() {
    let _ = u7::new(120).wrapping_div(u7::new(0));
}

#[test]
fn wrapping_shl() {
    assert_eq!(u7::new(0b010_1101).wrapping_shl(0), u7::new(0b010_1101));
    assert_eq!(u7::new(0b010_1101).wrapping_shl(1), u7::new(0b101_1010));
    assert_eq!(u7::new(0b010_1101).wrapping_shl(6), u7::new(0b100_0000));
    assert_eq!(u7::new(0b010_1101).wrapping_shl(7), u7::new(0b010_1101));
    assert_eq!(u7::new(0b010_1101).wrapping_shl(8), u7::new(0b101_1010));
    assert_eq!(u7::new(0b010_1101).wrapping_shl(14), u7::new(0b010_1101));
    assert_eq!(u7::new(0b010_1101).wrapping_shl(15), u7::new(0b101_1010));
}

#[test]
fn wrapping_shr() {
    assert_eq!(u7::new(0b010_1101).wrapping_shr(0), u7::new(0b010_1101));
    assert_eq!(u7::new(0b010_1101).wrapping_shr(1), u7::new(0b001_0110));
    assert_eq!(u7::new(0b010_1101).wrapping_shr(5), u7::new(0b000_0001));
    assert_eq!(u7::new(0b010_1101).wrapping_shr(7), u7::new(0b010_1101));
    assert_eq!(u7::new(0b010_1101).wrapping_shr(8), u7::new(0b001_0110));
    assert_eq!(u7::new(0b010_1101).wrapping_shr(14), u7::new(0b010_1101));
    assert_eq!(u7::new(0b010_1101).wrapping_shr(15), u7::new(0b001_0110));
}

#[test]
fn saturating_add() {
    assert_eq!(u7::new(120).saturating_add(u7::new(1)), u7::new(121));
    assert_eq!(u7::new(120).saturating_add(u7::new(10)), u7::new(127));
    assert_eq!(u7::new(127).saturating_add(u7::new(127)), u7::new(127));
    assert_eq!(
        AInt::<u8, 8>::new(250).saturating_add(AInt::<u8, 8>::new(10)),
        AInt::<u8, 8>::new(255)
    );
}

#[test]
fn saturating_sub() {
    assert_eq!(u7::new(120).saturating_sub(u7::new(30)), u7::new(90));
    assert_eq!(u7::new(120).saturating_sub(u7::new(119)), u7::new(1));
    assert_eq!(u7::new(120).saturating_sub(u7::new(120)), u7::new(0));
    assert_eq!(u7::new(120).saturating_sub(u7::new(121)), u7::new(0));
    assert_eq!(u7::new(0).saturating_sub(u7::new(127)), u7::new(0));
}

#[test]
fn saturating_mul() {
    // Fast-path: Only the arbitrary int is bounds checked
    assert_eq!(u4::new(5).saturating_mul(u4::new(2)), u4::new(10));
    assert_eq!(u4::new(5).saturating_mul(u4::new(3)), u4::new(15));
    assert_eq!(u4::new(5).saturating_mul(u4::new(4)), u4::new(15));
    assert_eq!(u4::new(5).saturating_mul(u4::new(5)), u4::new(15));
    assert_eq!(u4::new(5).saturating_mul(u4::new(6)), u4::new(15));
    assert_eq!(u4::new(5).saturating_mul(u4::new(7)), u4::new(15));

    // Slow-path (well, one more comparison)
    assert_eq!(u5::new(5).saturating_mul(u5::new(2)), u5::new(10));
    assert_eq!(u5::new(5).saturating_mul(u5::new(3)), u5::new(15));
    assert_eq!(u5::new(5).saturating_mul(u5::new(4)), u5::new(20));
    assert_eq!(u5::new(5).saturating_mul(u5::new(5)), u5::new(25));
    assert_eq!(u5::new(5).saturating_mul(u5::new(6)), u5::new(30));
    assert_eq!(u5::new(5).saturating_mul(u5::new(7)), u5::new(31));
    assert_eq!(u5::new(30).saturating_mul(u5::new(1)), u5::new(30));
    assert_eq!(u5::new(30).saturating_mul(u5::new(2)), u5::new(31));
    assert_eq!(u5::new(30).saturating_mul(u5::new(10)), u5::new(31));
}

#[test]
fn saturating_div() {
    assert_eq!(u4::new(5).saturating_div(u4::new(1)), u4::new(5));
    assert_eq!(u4::new(5).saturating_div(u4::new(2)), u4::new(2));
    assert_eq!(u4::new(5).saturating_div(u4::new(3)), u4::new(1));
    assert_eq!(u4::new(5).saturating_div(u4::new(4)), u4::new(1));
    assert_eq!(u4::new(5).saturating_div(u4::new(5)), u4::new(1));
}

#[test]
#[should_panic]
fn saturating_divby0() {
    // saturating_div throws an exception on zero
    let _ = u4::new(5).saturating_div(u4::new(0));
}

#[test]
fn saturating_pow() {
    assert_eq!(u7::new(5).saturating_pow(0), u7::new(1));
    assert_eq!(u7::new(5).saturating_pow(1), u7::new(5));
    assert_eq!(u7::new(5).saturating_pow(2), u7::new(25));
    assert_eq!(u7::new(5).saturating_pow(3), u7::new(125));
    assert_eq!(u7::new(5).saturating_pow(4), u7::new(127));
    assert_eq!(u7::new(5).saturating_pow(255), u7::new(127));
}

#[test]
fn checked_add() {
    assert_eq!(u7::new(120).checked_add(u7::new(1)), Some(u7::new(121)));
    assert_eq!(u7::new(120).checked_add(u7::new(7)), Some(u7::new(127)));
    assert_eq!(u7::new(120).checked_add(u7::new(10)), None);
    assert_eq!(u7::new(127).checked_add(u7::new(127)), None);
    assert_eq!(
        AInt::<u8, 8>::new(250).checked_add(AInt::<u8, 8>::new(10)),
        None
    );
}

#[test]
fn checked_sub() {
    assert_eq!(u7::new(120).checked_sub(u7::new(30)), Some(u7::new(90)));
    assert_eq!(u7::new(120).checked_sub(u7::new(119)), Some(u7::new(1)));
    assert_eq!(u7::new(120).checked_sub(u7::new(120)), Some(u7::new(0)));
    assert_eq!(u7::new(120).checked_sub(u7::new(121)), None);
    assert_eq!(u7::new(0).checked_sub(u7::new(127)), None);
}

#[test]
fn checked_mul() {
    // Fast-path: Only the arbitrary int is bounds checked
    assert_eq!(u4::new(5).checked_mul(u4::new(2)), Some(u4::new(10)));
    assert_eq!(u4::new(5).checked_mul(u4::new(3)), Some(u4::new(15)));
    assert_eq!(u4::new(5).checked_mul(u4::new(4)), None);
    assert_eq!(u4::new(5).checked_mul(u4::new(5)), None);
    assert_eq!(u4::new(5).checked_mul(u4::new(6)), None);
    assert_eq!(u4::new(5).checked_mul(u4::new(7)), None);

    // Slow-path (well, one more comparison)
    assert_eq!(u5::new(5).checked_mul(u5::new(2)), Some(u5::new(10)));
    assert_eq!(u5::new(5).checked_mul(u5::new(3)), Some(u5::new(15)));
    assert_eq!(u5::new(5).checked_mul(u5::new(4)), Some(u5::new(20)));
    assert_eq!(u5::new(5).checked_mul(u5::new(5)), Some(u5::new(25)));
    assert_eq!(u5::new(5).checked_mul(u5::new(6)), Some(u5::new(30)));
    assert_eq!(u5::new(5).checked_mul(u5::new(7)), None);
    assert_eq!(u5::new(30).checked_mul(u5::new(1)), Some(u5::new(30)));
    assert_eq!(u5::new(30).checked_mul(u5::new(2)), None);
    assert_eq!(u5::new(30).checked_mul(u5::new(10)), None);
}

#[test]
fn checked_div() {
    // checked_div handles division by zero without exception, unlike saturating_div
    assert_eq!(u4::new(5).checked_div(u4::new(0)), None);
    assert_eq!(u4::new(5).checked_div(u4::new(1)), Some(u4::new(5)));
    assert_eq!(u4::new(5).checked_div(u4::new(2)), Some(u4::new(2)));
    assert_eq!(u4::new(5).checked_div(u4::new(3)), Some(u4::new(1)));
    assert_eq!(u4::new(5).checked_div(u4::new(4)), Some(u4::new(1)));
    assert_eq!(u4::new(5).checked_div(u4::new(5)), Some(u4::new(1)));
}

#[test]
fn checked_shl() {
    assert_eq!(
        u7::new(0b010_1101).checked_shl(0),
        Some(u7::new(0b010_1101))
    );
    assert_eq!(
        u7::new(0b010_1101).checked_shl(1),
        Some(u7::new(0b101_1010))
    );
    assert_eq!(
        u7::new(0b010_1101).checked_shl(6),
        Some(u7::new(0b100_0000))
    );
    assert_eq!(u7::new(0b010_1101).checked_shl(7), None);
    assert_eq!(u7::new(0b010_1101).checked_shl(8), None);
    assert_eq!(u7::new(0b010_1101).checked_shl(14), None);
    assert_eq!(u7::new(0b010_1101).checked_shl(15), None);
}

#[test]
fn checked_shr() {
    assert_eq!(
        u7::new(0b010_1101).checked_shr(0),
        Some(u7::new(0b010_1101))
    );
    assert_eq!(
        u7::new(0b010_1101).checked_shr(1),
        Some(u7::new(0b001_0110))
    );
    assert_eq!(
        u7::new(0b010_1101).checked_shr(5),
        Some(u7::new(0b000_0001))
    );
    assert_eq!(u7::new(0b010_1101).checked_shr(7), None);
    assert_eq!(u7::new(0b010_1101).checked_shr(8), None);
    assert_eq!(u7::new(0b010_1101).checked_shr(14), None);
    assert_eq!(u7::new(0b010_1101).checked_shr(15), None);
}

#[test]
fn overflowing_add() {
    assert_eq!(
        u7::new(120).overflowing_add(u7::new(1)),
        (u7::new(121), false)
    );
    assert_eq!(
        u7::new(120).overflowing_add(u7::new(7)),
        (u7::new(127), false)
    );
    assert_eq!(
        u7::new(120).overflowing_add(u7::new(10)),
        (u7::new(2), true)
    );
    assert_eq!(
        u7::new(127).overflowing_add(u7::new(127)),
        (u7::new(126), true)
    );
    assert_eq!(
        AInt::<u8, 8>::new(250).overflowing_add(AInt::<u8, 8>::new(5)),
        (AInt::<u8, 8>::new(255), false)
    );
    assert_eq!(
        AInt::<u8, 8>::new(250).overflowing_add(AInt::<u8, 8>::new(10)),
        (AInt::<u8, 8>::new(4), true)
    );
}

#[test]
fn overflowing_sub() {
    assert_eq!(
        u7::new(120).overflowing_sub(u7::new(30)),
        (u7::new(90), false)
    );
    assert_eq!(
        u7::new(120).overflowing_sub(u7::new(119)),
        (u7::new(1), false)
    );
    assert_eq!(
        u7::new(120).overflowing_sub(u7::new(120)),
        (u7::new(0), false)
    );
    assert_eq!(
        u7::new(120).overflowing_sub(u7::new(121)),
        (u7::new(127), true)
    );
    assert_eq!(u7::new(0).overflowing_sub(u7::new(127)), (u7::new(1), true));
}

#[test]
fn overflowing_mul() {
    // Fast-path: Only the arbitrary int is bounds checked
    assert_eq!(u4::new(5).overflowing_mul(u4::new(2)), (u4::new(10), false));
    assert_eq!(u4::new(5).overflowing_mul(u4::new(3)), (u4::new(15), false));
    assert_eq!(u4::new(5).overflowing_mul(u4::new(4)), (u4::new(4), true));
    assert_eq!(u4::new(5).overflowing_mul(u4::new(5)), (u4::new(9), true));
    assert_eq!(u4::new(5).overflowing_mul(u4::new(6)), (u4::new(14), true));
    assert_eq!(u4::new(5).overflowing_mul(u4::new(7)), (u4::new(3), true));

    // Slow-path (well, one more comparison)
    assert_eq!(u5::new(5).overflowing_mul(u5::new(2)), (u5::new(10), false));
    assert_eq!(u5::new(5).overflowing_mul(u5::new(3)), (u5::new(15), false));
    assert_eq!(u5::new(5).overflowing_mul(u5::new(4)), (u5::new(20), false));
    assert_eq!(u5::new(5).overflowing_mul(u5::new(5)), (u5::new(25), false));
    assert_eq!(u5::new(5).overflowing_mul(u5::new(6)), (u5::new(30), false));
    assert_eq!(u5::new(5).overflowing_mul(u5::new(7)), (u5::new(3), true));
    assert_eq!(
        u5::new(30).overflowing_mul(u5::new(1)),
        (u5::new(30), false)
    );
    assert_eq!(u5::new(30).overflowing_mul(u5::new(2)), (u5::new(28), true));
    assert_eq!(
        u5::new(30).overflowing_mul(u5::new(10)),
        (u5::new(12), true)
    );
}

#[test]
fn overflowing_div() {
    assert_eq!(u4::new(5).overflowing_div(u4::new(1)), (u4::new(5), false));
    assert_eq!(u4::new(5).overflowing_div(u4::new(2)), (u4::new(2), false));
    assert_eq!(u4::new(5).overflowing_div(u4::new(3)), (u4::new(1), false));
    assert_eq!(u4::new(5).overflowing_div(u4::new(4)), (u4::new(1), false));
    assert_eq!(u4::new(5).overflowing_div(u4::new(5)), (u4::new(1), false));
}

#[should_panic]
#[test]
fn overflowing_div_by_zero() {
    let _ = u4::new(5).overflowing_div(u4::new(0));
}

#[test]
fn overflowing_shl() {
    assert_eq!(
        u7::new(0b010_1101).overflowing_shl(0),
        (u7::new(0b010_1101), false)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shl(1),
        (u7::new(0b101_1010), false)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shl(6),
        (u7::new(0b100_0000), false)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shl(7),
        (u7::new(0b010_1101), true)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shl(8),
        (u7::new(0b101_1010), true)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shl(14),
        (u7::new(0b010_1101), true)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shl(15),
        (u7::new(0b101_1010), true)
    );
}

#[test]
fn overflowing_shr() {
    assert_eq!(
        u7::new(0b010_1101).overflowing_shr(0),
        (u7::new(0b010_1101), false)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shr(1),
        (u7::new(0b001_0110), false)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shr(5),
        (u7::new(0b000_0001), false)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shr(7),
        (u7::new(0b010_1101), true)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shr(8),
        (u7::new(0b001_0110), true)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shr(14),
        (u7::new(0b010_1101), true)
    );
    assert_eq!(
        u7::new(0b010_1101).overflowing_shr(15),
        (u7::new(0b001_0110), true)
    );
}

#[test]
fn reverse_bits() {
    const A: u5 = u5::new(0b11101);
    const B: u5 = A.reverse_bits();
    assert_eq!(B, u5::new(0b10111));

    assert_eq!(
        AInt::<u128, 6>::new(0b101011),
        AInt::<u128, 6>::new(0b110101).reverse_bits()
    );

    assert_eq!(u1::new(1).reverse_bits().value(), 1);
    assert_eq!(u1::new(0).reverse_bits().value(), 0);
}

#[test]
fn count_ones_and_zeros() {
    assert_eq!(4, u5::new(0b10111).count_ones());
    assert_eq!(1, u5::new(0b10111).count_zeros());
    assert_eq!(1, u5::new(0b10111).leading_ones());
    assert_eq!(0, u5::new(0b10111).leading_zeros());
    assert_eq!(3, u5::new(0b10111).trailing_ones());
    assert_eq!(0, u5::new(0b10111).trailing_zeros());

    assert_eq!(2, u5::new(0b10100).trailing_zeros());
    assert_eq!(3, u5::new(0b00011).leading_zeros());

    assert_eq!(0, u5::new(0b00000).count_ones());
    assert_eq!(5, u5::new(0b00000).count_zeros());

    assert_eq!(5, u5::new(0b11111).count_ones());
    assert_eq!(0, u5::new(0b11111).count_zeros());

    assert_eq!(3, u127::new(0b111).count_ones());
    assert_eq!(124, u127::new(0b111).count_zeros());
}

#[test]
fn rotate_left() {
    assert_eq!(u1::new(0b1), u1::new(0b1).rotate_left(1));
    assert_eq!(u2::new(0b01), u2::new(0b10).rotate_left(1));

    assert_eq!(u5::new(0b10111), u5::new(0b10111).rotate_left(0));
    assert_eq!(u5::new(0b01111), u5::new(0b10111).rotate_left(1));
    assert_eq!(u5::new(0b11110), u5::new(0b10111).rotate_left(2));
    assert_eq!(u5::new(0b11101), u5::new(0b10111).rotate_left(3));
    assert_eq!(u5::new(0b11011), u5::new(0b10111).rotate_left(4));
    assert_eq!(u5::new(0b10111), u5::new(0b10111).rotate_left(5));
    assert_eq!(u5::new(0b01111), u5::new(0b10111).rotate_left(6));
    assert_eq!(u5::new(0b01111), u5::new(0b10111).rotate_left(556));

    assert_eq!(u24::new(0x0FFEEC), u24::new(0xC0FFEE).rotate_left(4));
}

#[test]
fn rotate_right() {
    assert_eq!(u1::new(0b1), u1::new(0b1).rotate_right(1));
    assert_eq!(u2::new(0b01), u2::new(0b10).rotate_right(1));

    assert_eq!(u5::new(0b10011), u5::new(0b10011).rotate_right(0));
    assert_eq!(u5::new(0b11001), u5::new(0b10011).rotate_right(1));
    assert_eq!(u5::new(0b11100), u5::new(0b10011).rotate_right(2));
    assert_eq!(u5::new(0b01110), u5::new(0b10011).rotate_right(3));
    assert_eq!(u5::new(0b00111), u5::new(0b10011).rotate_right(4));
    assert_eq!(u5::new(0b10011), u5::new(0b10011).rotate_right(5));
    assert_eq!(u5::new(0b11001), u5::new(0b10011).rotate_right(6));

    assert_eq!(u24::new(0xEC0FFE), u24::new(0xC0FFEE).rotate_right(4));
}




macro_rules! auto_test {
    ($int:ident, $bits:literal, $min:literal, $max:literal) => {
        paste! {
            #[test]
            fn [<$int _consts>]() {
                let bits = $bits;
                let min = $min;
                let max = $max;

                assert_eq!($int::BITS, bits);
                assert_eq!($int::MIN.value(), min);
                assert_eq!($int::MAX.value(), max);
                assert_eq!($int::MIN, $int!($min));
                assert_eq!($int::MAX, $int!($max));
            }

            #[test]
            fn [<$int _new>]() {
                let min = $min;
                let max = $max;

                assert_eq!($int::new(min), $int!($min));
                assert_eq!($int::new(max), $int!($max));
                assert_eq!($int::try_new(max + 1), Err(TryNewError{kind: NumberErrorKind::PosOverflow}));
                // assert_eq!($int::new(min.wrapping_sub(1)), None);
            }

            // #[test]
            // fn [<$int _new_wrapping>]() {
            //     let min = $min;
            //     let max = $max;

            //     assert_eq!($int::new_wrapping(min), $int!($min));
            //     assert_eq!($int::new_wrapping(max), $int!($max));
            //     assert_eq!($int::new_wrapping(max + 1), $int!($min));
            //     assert_eq!($int::new_wrapping(min.wrapping_sub(1)), $int!($max));
            // }

            // #[test]
            // fn [<$int _new_saturating>]() {
            //     let min = $min;
            //     let max = $max;

            //     assert_eq!($int::new_saturating(min), $int!($min));
            //     assert_eq!($int::new_saturating(max), $int!($max));
            //     assert_eq!($int::new_saturating(max + 1), $int!($max));
            //     if min != 0 {
            //         assert_eq!($int::new_saturating(min - 1), $int!($min));
            //     }
            // }
        }
    };
}

auto_test!(u1, 1, 0, 1);
auto_test!(u2, 2, 0, 3);
auto_test!(u3, 3, 0, 7);
auto_test!(u4, 4, 0, 15);
auto_test!(u5, 5, 0, 31);
auto_test!(u6, 6, 0, 63);
auto_test!(u7, 7, 0, 127);
auto_test!(u9, 9, 0, 511);
auto_test!(u10, 10, 0, 1023);
auto_test!(u11, 11, 0, 2047);
auto_test!(u12, 12, 0, 4095);
auto_test!(u13, 13, 0, 8191);
auto_test!(u14, 14, 0, 16383);
auto_test!(u15, 15, 0, 32767);
auto_test!(u17, 17, 0, 131071);
auto_test!(u18, 18, 0, 262143);
auto_test!(u19, 19, 0, 524287);
auto_test!(u20, 20, 0, 1048575);
auto_test!(u21, 21, 0, 2097151);
auto_test!(u22, 22, 0, 4194303);
auto_test!(u23, 23, 0, 8388607);
auto_test!(u24, 24, 0, 16777215);
auto_test!(u25, 25, 0, 33554431);
auto_test!(u26, 26, 0, 67108863);
auto_test!(u27, 27, 0, 134217727);
auto_test!(u28, 28, 0, 268435455);
auto_test!(u29, 29, 0, 536870911);
auto_test!(u30, 30, 0, 1073741823);
auto_test!(u31, 31, 0, 2147483647);
auto_test!(u33, 33, 0, 8589934591);
auto_test!(u34, 34, 0, 17179869183);
auto_test!(u35, 35, 0, 34359738367);
auto_test!(u36, 36, 0, 68719476735);
auto_test!(u37, 37, 0, 137438953471);
auto_test!(u38, 38, 0, 274877906943);
auto_test!(u39, 39, 0, 549755813887);
auto_test!(u40, 40, 0, 1099511627775);
auto_test!(u41, 41, 0, 2199023255551);
auto_test!(u42, 42, 0, 4398046511103);
auto_test!(u43, 43, 0, 8796093022207);
auto_test!(u44, 44, 0, 17592186044415);
auto_test!(u45, 45, 0, 35184372088831);
auto_test!(u46, 46, 0, 70368744177663);
auto_test!(u47, 47, 0, 140737488355327);
auto_test!(u48, 48, 0, 281474976710655);
auto_test!(u49, 49, 0, 562949953421311);
auto_test!(u50, 50, 0, 1125899906842623);
auto_test!(u51, 51, 0, 2251799813685247);
auto_test!(u52, 52, 0, 4503599627370495);
auto_test!(u53, 53, 0, 9007199254740991);
auto_test!(u54, 54, 0, 18014398509481983);
auto_test!(u55, 55, 0, 36028797018963967);
auto_test!(u56, 56, 0, 72057594037927935);
auto_test!(u57, 57, 0, 144115188075855871);
auto_test!(u58, 58, 0, 288230376151711743);
auto_test!(u59, 59, 0, 576460752303423487);
auto_test!(u60, 60, 0, 1152921504606846975);
auto_test!(u61, 61, 0, 2305843009213693951);
auto_test!(u62, 62, 0, 4611686018427387903);
auto_test!(u63, 63, 0, 9223372036854775807);

mod test_128{
        use super::*;

    auto_test!(u65, 65, 0, 36893488147419103231);
    auto_test!(u66, 66, 0, 73786976294838206463);
    auto_test!(u67, 67, 0, 147573952589676412927);
    auto_test!(u68, 68, 0, 295147905179352825855);
    auto_test!(u69, 69, 0, 590295810358705651711);
    auto_test!(u70, 70, 0, 1180591620717411303423);
    auto_test!(u71, 71, 0, 2361183241434822606847);
    auto_test!(u72, 72, 0, 4722366482869645213695);
    auto_test!(u73, 73, 0, 9444732965739290427391);
    auto_test!(u74, 74, 0, 18889465931478580854783);
    auto_test!(u75, 75, 0, 37778931862957161709567);
    auto_test!(u76, 76, 0, 75557863725914323419135);
    auto_test!(u77, 77, 0, 151115727451828646838271);
    auto_test!(u78, 78, 0, 302231454903657293676543);
    auto_test!(u79, 79, 0, 604462909807314587353087);
    auto_test!(u80, 80, 0, 1208925819614629174706175);
    auto_test!(u81, 81, 0, 2417851639229258349412351);
    auto_test!(u82, 82, 0, 4835703278458516698824703);
    auto_test!(u83, 83, 0, 9671406556917033397649407);
    auto_test!(u84, 84, 0, 19342813113834066795298815);
    auto_test!(u85, 85, 0, 38685626227668133590597631);
    auto_test!(u86, 86, 0, 77371252455336267181195263);
    auto_test!(u87, 87, 0, 154742504910672534362390527);
    auto_test!(u88, 88, 0, 309485009821345068724781055);
    auto_test!(u89, 89, 0, 618970019642690137449562111);
    auto_test!(u90, 90, 0, 1237940039285380274899124223);
    auto_test!(u91, 91, 0, 2475880078570760549798248447);
    auto_test!(u92, 92, 0, 4951760157141521099596496895);
    auto_test!(u93, 93, 0, 9903520314283042199192993791);
    auto_test!(u94, 94, 0, 19807040628566084398385987583);
    auto_test!(u95, 95, 0, 39614081257132168796771975167);
    auto_test!(u96, 96, 0, 79228162514264337593543950335);
    auto_test!(u97, 97, 0, 158456325028528675187087900671);
    auto_test!(u98, 98, 0, 316912650057057350374175801343);
    auto_test!(u99, 99, 0, 633825300114114700748351602687);
    auto_test!(u100, 100, 0, 1267650600228229401496703205375);
    auto_test!(u101, 101, 0, 2535301200456458802993406410751);
    auto_test!(u102, 102, 0, 5070602400912917605986812821503);
    auto_test!(u103, 103, 0, 10141204801825835211973625643007);
    auto_test!(u104, 104, 0, 20282409603651670423947251286015);
    auto_test!(u105, 105, 0, 40564819207303340847894502572031);
    auto_test!(u106, 106, 0, 81129638414606681695789005144063);
    auto_test!(u107, 107, 0, 162259276829213363391578010288127);
    auto_test!(u108, 108, 0, 324518553658426726783156020576255);
    auto_test!(u109, 109, 0, 649037107316853453566312041152511);
    auto_test!(u110, 110, 0, 1298074214633706907132624082305023);
    auto_test!(u111, 111, 0, 2596148429267413814265248164610047);
    auto_test!(u112, 112, 0, 5192296858534827628530496329220095);
    auto_test!(u113, 113, 0, 10384593717069655257060992658440191);
    auto_test!(u114, 114, 0, 20769187434139310514121985316880383);
    auto_test!(u115, 115, 0, 41538374868278621028243970633760767);
    auto_test!(u116, 116, 0, 83076749736557242056487941267521535);
    auto_test!(u117, 117, 0, 166153499473114484112975882535043071);
    auto_test!(u118, 118, 0, 332306998946228968225951765070086143);
    auto_test!(u119, 119, 0, 664613997892457936451903530140172287);
    auto_test!(u120, 120, 0, 1329227995784915872903807060280344575);
    auto_test!(u121, 121, 0, 2658455991569831745807614120560689151);
    auto_test!(u122, 122, 0, 5316911983139663491615228241121378303);
    auto_test!(u123, 123, 0, 10633823966279326983230456482242756607);
    auto_test!(u124, 124, 0, 21267647932558653966460912964485513215);
    auto_test!(u125, 125, 0, 42535295865117307932921825928971026431);
    auto_test!(u126, 126, 0, 85070591730234615865843651857942052863);
    auto_test!(u127, 127, 0, 170141183460469231731687303715884105727);

}
// auto_test!(i1, 1, -1, 0);
// auto_test!(i2, 2, -2, 1);
// auto_test!(i3, 3, -4, 3);
// auto_test!(i4, 4, -8, 7);
// auto_test!(i5, 5, -16, 15);
// auto_test!(i6, 6, -32, 31);
// auto_test!(i7, 7, -64, 63);
// auto_test!(i9, 9, -256, 255);
// auto_test!(i10, 10, -512, 511);
// auto_test!(i11, 11, -1024, 1023);
// auto_test!(i12, 12, -2048, 2047);
// auto_test!(i13, 13, -4096, 4095);
// auto_test!(i14, 14, -8192, 8191);
// auto_test!(i15, 15, -16384, 16383);
// auto_test!(i17, 17, -65536, 65535);
// auto_test!(i18, 18, -131072, 131071);
// auto_test!(i19, 19, -262144, 262143);
// auto_test!(i20, 20, -524288, 524287);
// auto_test!(i21, 21, -1048576, 1048575);
// auto_test!(i22, 22, -2097152, 2097151);
// auto_test!(i23, 23, -4194304, 4194303);
// auto_test!(i24, 24, -8388608, 8388607);
// auto_test!(i25, 25, -16777216, 16777215);
// auto_test!(i26, 26, -33554432, 33554431);
// auto_test!(i27, 27, -67108864, 67108863);
// auto_test!(i28, 28, -134217728, 134217727);
// auto_test!(i29, 29, -268435456, 268435455);
// auto_test!(i30, 30, -536870912, 536870911);
// auto_test!(i31, 31, -1073741824, 1073741823);
// auto_test!(i33, 33, -4294967296, 4294967295);
// auto_test!(i34, 34, -8589934592, 8589934591);
// auto_test!(i35, 35, -17179869184, 17179869183);
// auto_test!(i36, 36, -34359738368, 34359738367);
// auto_test!(i37, 37, -68719476736, 68719476735);
// auto_test!(i38, 38, -137438953472, 137438953471);
// auto_test!(i39, 39, -274877906944, 274877906943);
// auto_test!(i40, 40, -549755813888, 549755813887);
// auto_test!(i41, 41, -1099511627776, 1099511627775);
// auto_test!(i42, 42, -2199023255552, 2199023255551);
// auto_test!(i43, 43, -4398046511104, 4398046511103);
// auto_test!(i44, 44, -8796093022208, 8796093022207);
// auto_test!(i45, 45, -17592186044416, 17592186044415);
// auto_test!(i46, 46, -35184372088832, 35184372088831);
// auto_test!(i47, 47, -70368744177664, 70368744177663);
// auto_test!(i48, 48, -140737488355328, 140737488355327);
// auto_test!(i49, 49, -281474976710656, 281474976710655);
// auto_test!(i50, 50, -562949953421312, 562949953421311);
// auto_test!(i51, 51, -1125899906842624, 1125899906842623);
// auto_test!(i52, 52, -2251799813685248, 2251799813685247);
// auto_test!(i53, 53, -4503599627370496, 4503599627370495);
// auto_test!(i54, 54, -9007199254740992, 9007199254740991);
// auto_test!(i55, 55, -18014398509481984, 18014398509481983);
// auto_test!(i56, 56, -36028797018963968, 36028797018963967);
// auto_test!(i57, 57, -72057594037927936, 72057594037927935);
// auto_test!(i58, 58, -144115188075855872, 144115188075855871);
// auto_test!(i59, 59, -288230376151711744, 288230376151711743);
// auto_test!(i60, 60, -576460752303423488, 576460752303423487);
// auto_test!(i61, 61, -1152921504606846976, 1152921504606846975);
// auto_test!(i62, 62, -2305843009213693952, 2305843009213693951);
// auto_test!(i63, 63, -4611686018427387904, 4611686018427387903);
// auto_test!(i65, 65, -18446744073709551616, 18446744073709551615);
// auto_test!(i66, 66, -36893488147419103232, 36893488147419103231);
// auto_test!(i67, 67, -73786976294838206464, 73786976294838206463);
// auto_test!(i68, 68, -147573952589676412928, 147573952589676412927);
// auto_test!(i69, 69, -295147905179352825856, 295147905179352825855);
// auto_test!(i70, 70, -590295810358705651712, 590295810358705651711);
// auto_test!(i71, 71, -1180591620717411303424, 1180591620717411303423);
// auto_test!(i72, 72, -2361183241434822606848, 2361183241434822606847);
// auto_test!(i73, 73, -4722366482869645213696, 4722366482869645213695);
// auto_test!(i74, 74, -9444732965739290427392, 9444732965739290427391);
// auto_test!(i75, 75, -18889465931478580854784, 18889465931478580854783);
// auto_test!(i76, 76, -37778931862957161709568, 37778931862957161709567);
// auto_test!(i77, 77, -75557863725914323419136, 75557863725914323419135);
// auto_test!(i78, 78, -151115727451828646838272, 151115727451828646838271);
// auto_test!(i79, 79, -302231454903657293676544, 302231454903657293676543);
// auto_test!(i80, 80, -604462909807314587353088, 604462909807314587353087);
// auto_test!(
//     i81,
//     81,
//     -1208925819614629174706176,
//     1208925819614629174706175
// );
// auto_test!(
//     i82,
//     82,
//     -2417851639229258349412352,
//     2417851639229258349412351
// );
// auto_test!(
//     i83,
//     83,
//     -4835703278458516698824704,
//     4835703278458516698824703
// );
// auto_test!(
//     i84,
//     84,
//     -9671406556917033397649408,
//     9671406556917033397649407
// );
// auto_test!(
//     i85,
//     85,
//     -19342813113834066795298816,
//     19342813113834066795298815
// );
// auto_test!(
//     i86,
//     86,
//     -38685626227668133590597632,
//     38685626227668133590597631
// );
// auto_test!(
//     i87,
//     87,
//     -77371252455336267181195264,
//     77371252455336267181195263
// );
// auto_test!(
//     i88,
//     88,
//     -154742504910672534362390528,
//     154742504910672534362390527
// );
// auto_test!(
//     i89,
//     89,
//     -309485009821345068724781056,
//     309485009821345068724781055
// );
// auto_test!(
//     i90,
//     90,
//     -618970019642690137449562112,
//     618970019642690137449562111
// );
// auto_test!(
//     i91,
//     91,
//     -1237940039285380274899124224,
//     1237940039285380274899124223
// );
// auto_test!(
//     i92,
//     92,
//     -2475880078570760549798248448,
//     2475880078570760549798248447
// );
// auto_test!(
//     i93,
//     93,
//     -4951760157141521099596496896,
//     4951760157141521099596496895
// );
// auto_test!(
//     i94,
//     94,
//     -9903520314283042199192993792,
//     9903520314283042199192993791
// );
// auto_test!(
//     i95,
//     95,
//     -19807040628566084398385987584,
//     19807040628566084398385987583
// );
// auto_test!(
//     i96,
//     96,
//     -39614081257132168796771975168,
//     39614081257132168796771975167
// );
// auto_test!(
//     i97,
//     97,
//     -79228162514264337593543950336,
//     79228162514264337593543950335
// );
// auto_test!(
//     i98,
//     98,
//     -158456325028528675187087900672,
//     158456325028528675187087900671
// );
// auto_test!(
//     i99,
//     99,
//     -316912650057057350374175801344,
//     316912650057057350374175801343
// );
// auto_test!(
//     i100,
//     100,
//     -633825300114114700748351602688,
//     633825300114114700748351602687
// );
// auto_test!(
//     i101,
//     101,
//     -1267650600228229401496703205376,
//     1267650600228229401496703205375
// );
// auto_test!(
//     i102,
//     102,
//     -2535301200456458802993406410752,
//     2535301200456458802993406410751
// );
// auto_test!(
//     i103,
//     103,
//     -5070602400912917605986812821504,
//     5070602400912917605986812821503
// );
// auto_test!(
//     i104,
//     104,
//     -10141204801825835211973625643008,
//     10141204801825835211973625643007
// );
// auto_test!(
//     i105,
//     105,
//     -20282409603651670423947251286016,
//     20282409603651670423947251286015
// );
// auto_test!(
//     i106,
//     106,
//     -40564819207303340847894502572032,
//     40564819207303340847894502572031
// );
// auto_test!(
//     i107,
//     107,
//     -81129638414606681695789005144064,
//     81129638414606681695789005144063
// );
// auto_test!(
//     i108,
//     108,
//     -162259276829213363391578010288128,
//     162259276829213363391578010288127
// );
// auto_test!(
//     i109,
//     109,
//     -324518553658426726783156020576256,
//     324518553658426726783156020576255
// );
// auto_test!(
//     i110,
//     110,
//     -649037107316853453566312041152512,
//     649037107316853453566312041152511
// );
// auto_test!(
//     i111,
//     111,
//     -1298074214633706907132624082305024,
//     1298074214633706907132624082305023
// );
// auto_test!(
//     i112,
//     112,
//     -2596148429267413814265248164610048,
//     2596148429267413814265248164610047
// );
// auto_test!(
//     i113,
//     113,
//     -5192296858534827628530496329220096,
//     5192296858534827628530496329220095
// );
// auto_test!(
//     i114,
//     114,
//     -10384593717069655257060992658440192,
//     10384593717069655257060992658440191
// );
// auto_test!(
//     i115,
//     115,
//     -20769187434139310514121985316880384,
//     20769187434139310514121985316880383
// );
// auto_test!(
//     i116,
//     116,
//     -41538374868278621028243970633760768,
//     41538374868278621028243970633760767
// );
// auto_test!(
//     i117,
//     117,
//     -83076749736557242056487941267521536,
//     83076749736557242056487941267521535
// );
// auto_test!(
//     i118,
//     118,
//     -166153499473114484112975882535043072,
//     166153499473114484112975882535043071
// );
// auto_test!(
//     i119,
//     119,
//     -332306998946228968225951765070086144,
//     332306998946228968225951765070086143
// );
// auto_test!(
//     i120,
//     120,
//     -664613997892457936451903530140172288,
//     664613997892457936451903530140172287
// );
// auto_test!(
//     i121,
//     121,
//     -1329227995784915872903807060280344576,
//     1329227995784915872903807060280344575
// );
// auto_test!(
//     i122,
//     122,
//     -2658455991569831745807614120560689152,
//     2658455991569831745807614120560689151
// );
// auto_test!(
//     i123,
//     123,
//     -5316911983139663491615228241121378304,
//     5316911983139663491615228241121378303
// );
// auto_test!(
//     i124,
//     124,
//     -10633823966279326983230456482242756608,
//     10633823966279326983230456482242756607
// );
// auto_test!(
//     i125,
//     125,
//     -21267647932558653966460912964485513216,
//     21267647932558653966460912964485513215
// );
// auto_test!(
//     i126,
//     126,
//     -42535295865117307932921825928971026432,
//     42535295865117307932921825928971026431
// );
// auto_test!(
//     i127,
//     127,
//     -85070591730234615865843651857942052864,
//     85070591730234615865843651857942052863
// );
