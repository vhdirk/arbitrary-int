use core::iter::Step;

use crate::traits::BitsSpec;
use crate::{AInt, Number, AIntContainer};

impl<T, Bits> Step for AInt<T, Bits>
where
    Self: Number<Container = T, Bits = Bits>,
    T: AIntContainer + Step,
    Bits: BitsSpec,
    <T as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>
{
    #[inline]
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        Step::steps_between(&start.value(), &end.value())
    }

    #[inline]
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(res) = Step::forward_checked(start.value(), count) {
            Self::try_new(res).ok()
        } else {
            None
        }
    }

    #[inline]
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if let Some(res) = Step::backward_checked(start.value(), count) {
            Self::try_new(res).ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aliases::*;

    #[test]
    fn range_agrees_with_underlying() {
        compare_range(u19::MIN, u19::MAX);
        compare_range(u37::new(95_993), u37::new(1_994_910));
        compare_range(u68::new(58_858_348), u68::new(58_860_000));
        compare_range(u122::new(111_222_333_444), u122::new(111_222_444_555));
        compare_range(u5::MIN, u5::MAX);
        compare_range(u23::MIN, u23::MAX);
        compare_range(u48::new(999_444), u48::new(1_005_000));
        compare_range(u99::new(12345), u99::new(54321));

        fn compare_range<T, Bits>(arb_start: AInt<T, Bits>, arb_end: AInt<T, Bits>)
        where
            Bits: BitsSpec,
            AInt<T, Bits>: Step + Number<Container = T, Bits=Bits>,
            T: AIntContainer + Copy + Step,
            <T as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
        {
            let arbint_range = (arb_start..=arb_end).map(AInt::value);
            let underlying_range = arb_start.value()..=arb_end.value();

            assert!(arbint_range.eq(underlying_range));
        }
    }

    #[test]
    fn forward_checked() {
        // In range
        assert_eq!(Some(u7::new(121)), Step::forward_checked(u7::new(120), 1));
        assert_eq!(Some(u7::new(127)), Step::forward_checked(u7::new(120), 7));

        // Out of range
        assert_eq!(None, Step::forward_checked(u7::new(120), 8));

        // Out of range for the underlying type
        assert_eq!(None, Step::forward_checked(u7::new(120), 140));
    }

    #[test]
    fn backward_checked() {
        // In range
        assert_eq!(Some(u7::new(1)), Step::backward_checked(u7::new(10), 9));
        assert_eq!(Some(u7::new(0)), Step::backward_checked(u7::new(10), 10));

        // Out of range (for both the arbitrary int and and the underlying type)
        assert_eq!(None, Step::backward_checked(u7::new(10), 11));
    }

    #[test]
    fn steps_between() {
        assert_eq!(Some(0), Step::steps_between(&u50::new(50), &u50::new(50)));

        assert_eq!(Some(4), Step::steps_between(&u24::new(5), &u24::new(9)));
        assert_eq!(None, Step::steps_between(&u24::new(9), &u24::new(5)));

        // this assumes usize is <= 64 bits. a test like this one exists in `core::iter::step`.
        assert_eq!(
            Some(usize::MAX),
            Step::steps_between(&u125::new(0x7), &u125::new(0x1_0000_0000_0000_0006))
        );
        assert_eq!(
            None,
            Step::steps_between(&u125::new(0x7), &u125::new(0x1_0000_0000_0000_0007))
        );
    }
}
