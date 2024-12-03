use core::ops::{Add, Shr};
use crate::{AInt, AIntContainer, Number};

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
