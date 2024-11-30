use std::process::Output;

use crate::{traits::BitsSpec, AInt, AIntContainer, Number};

#[allow(unused)]
pub(crate) struct Assert<const COND: bool> {}

#[allow(unused)]
pub(crate) trait IsTrue {}

impl IsTrue for Assert<true> {}

#[allow(unused)]
pub(crate) trait IsFalse {}

impl IsFalse for Assert<false> {}


// pub trait NotSame<TA, BitsA, TB, BitsB>
// where
//     TA: AIntContainer,
//     TB: AIntContainer,
//     <TA as AIntContainer>::Bits: typenum::IsNotEqual<<TB as AIntContainer>::Bits>,
//     BitsA: BitsSpec,
//     BitsB: BitsSpec + typenum::IsNotEqual<BitsA>,
// {}



pub trait NotSame<RhsT, RhsBits>
    where
    RhsT: AIntContainer,
    RhsBits: BitsSpec,
    <RhsT as AIntContainer>::Bits: typenum::IsGreaterOrEqual<RhsBits, Output = typenum::True>,
{}

impl<T, Bits, RhsT, RhsBits> NotSame<RhsT, RhsBits> for AInt<T, Bits> where
    // validate Self
    T: AIntContainer,
    Bits: BitsSpec,
    <T as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,

    // Validate From
    RhsT: AIntContainer,
    RhsBits: BitsSpec,
    <RhsT as AIntContainer>::Bits: typenum::IsGreaterOrEqual<RhsBits, Output = typenum::True>,

    Bits: typenum::IsNotEqual<RhsBits, Output = typenum::True>,
    <T as AIntContainer>::Bits: typenum::IsNotEqual<<RhsT as AIntContainer>::Bits, Output = typenum::True>,
{}


pub trait NotSame2 {}



impl<T, Bits, RhsT, RhsBits> NotSame2 for (AInt<T, Bits>, AInt<RhsT, RhsBits>) where
    // validate Self
    T: AIntContainer,
    Bits: BitsSpec,
    <T as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,

    // Validate From
    RhsT: AIntContainer,
    RhsBits: BitsSpec,
    <RhsT as AIntContainer>::Bits: typenum::IsGreaterOrEqual<RhsBits, Output = typenum::True>,

    Bits: typenum::IsNotEqual<RhsBits, Output = typenum::True>,
    <T as AIntContainer>::Bits: typenum::IsNotEqual<<RhsT as AIntContainer>::Bits, Output = typenum::True>,
{}




pub(crate) struct CompileTimeAssert<const A: usize, const B: usize> {}

impl<const A: usize, const B: usize> CompileTimeAssert<A, B> {
    #[allow(unused)]
    pub const LESSER_THAN: () = {
        assert!(A < B);
    };

    pub const LESSER_OR_EQUAL: () = {
        assert!(A <= B);
    };

    #[allow(unused)]
    pub const EQUAL: () = {
        assert!(A == B);
    };

    #[allow(unused)]
    pub const NOT_EQUAL: () = {
        assert!(A != B);
    };

    #[allow(unused)]
    pub const GREATER_OR_EQUAL: () = {
        assert!(A >= B);
    };

    #[allow(unused)]
    pub const GREATER_THAN: () = {
        assert!(A > B);
    };
}
