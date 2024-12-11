use const_panic::concat_assert;
use std::marker::PhantomData;

use seq_macro::seq;

#[allow(unused)]
pub(crate) struct Assert<const COND: bool> {}

#[allow(unused)]
pub(crate) trait IsTrue {}

impl IsTrue for Assert<true> {}

#[allow(unused)]
pub(crate) trait IsFalse {}

impl IsFalse for Assert<false> {}


pub(crate) struct CompileTimeAssert<const A: usize, const B: usize> {}

impl<const A: usize, const B: usize> CompileTimeAssert<A, B> {
    #[allow(unused)]
    pub const LESSER_THAN: () = {
        assert!(A < B);
    };

    #[allow(unused)]
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



#[macro_export]
macro_rules! assert_bounds {
    ($n: expr, $min:expr, $max:expr) => {
        let _ = const {
            use const_panic::concat_assert;

            concat_assert!{
                $n >= $min,
                "Value ", $n as usize, " is less than minimum value ", $min as usize
            }

            concat_assert!{
                $n <= $max,
                "Value ", $n as usize, " is greater than maximum value ", $max as usize
            }

        };
    };

    ( ($n: expr, $min:expr, $max:expr ) => { $body: expr }) => {
        {
            assert_bounds!($n, $min, $max);

            $body
        }
    };

}

pub struct ConstVal;
pub trait ConstExprType {}

pub struct LesserOrEqual;

impl ConstExprType for LesserOrEqual {}

pub trait ConstExpr<E: ConstExprType, const N: usize, const LIMIT: usize> {}


#[derive(Debug, Default)]
pub struct ConstBounded<const N: usize, const MIN: usize, const MAX: usize>(PhantomData<()>);

// impl<const N: usize, const MIN: usize>  ConstMin<N, MIN> for () {}
// impl<const N: usize, const MAX: usize>  ConstMax<N, MAX> for () {}


#[cfg(not(feature = "generic_const_exprs"))]
impl<const N: usize, const MIN: usize, const MAX: usize> ConstBounded<N, MIN, MAX>
where
    ConstVal: ConstExpr<LesserOrEqual, MIN, N> + ConstExpr<LesserOrEqual, N, MAX>
{
    pub const fn new() -> Self {
        Self(PhantomData)
    }

    pub const ASSERT: () = {
        assert_bounds!(N, MIN, MAX);
    };

    pub const fn bounded<T>(value: T) -> T {
        let _ = Self::ASSERT;
        value
    }
}



#[cfg(feature = "generic_const_exprs")]
impl<const N: usize, const MIN: usize, const MAX: usize> ConstBounded<N, MIN, MAX>
where
    [(); (N >= MIN) as usize]:,
    [(); (N <= MAX) as usize]:,
{
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

macro_rules! impl_const_bound {
    ($n:expr, $limit:expr) => {
        #[cfg(not(feature = "generic_const_exprs"))]
        impl ConstExpr<LesserOrEqual, $n, $limit> for ConstVal {}
    };

    ($limit:expr ) => {
        seq!(N in 1..=$limit {
            #(
                impl_const_bound!(N, $limit);
            )*
        });
    };

    () => {
        seq!(LIMIT in 1..=128 {
            #(
                impl_const_bound!(LIMIT);
            )*
        });
    }
}

impl_const_bound!();

