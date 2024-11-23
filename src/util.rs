
pub(crate) struct Assert<const COND: bool> {}

pub(crate) trait IsTrue {}

impl IsTrue for Assert<true> {}

pub(crate) trait IsFalse {}

impl IsFalse for Assert<false> {}


pub(crate) struct CompileTimeAssert<const A: usize, const B: usize> {}

impl<const A: usize, const B: usize> CompileTimeAssert<A, B> {
    pub const LESSER_THAN: () = {
        assert!(A < B);
    };

    pub const LESSER_OR_EQUAL: () = {
        assert!(A <= B);
    };

    pub const EQUAL: () = {
        assert!(A == B);
    };

    pub const NOT_EQUAL: () = {
        assert!(A != B);
    };

    pub const GREATER_OR_EQUAL: () = {
        assert!(A >= B);
    };

    pub const GREATER_THAN: () = {
        assert!(A > B);
    };
}