use core::iter::Step;

use crate::Number;

use super::{Int, SignedNumberType};

impl<T, const BITS: usize> Step for Int<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: SignedNumberType + Step,
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
    use crate::uint::aliases::*;

    #[test]
    fn range_agrees_with_underlying() {
        compare_range(i19::MIN, i19::MAX);
        compare_range(i37::new(95_993), i37::new(1_994_910));
        compare_range(i68::new(58_858_348), i68::new(58_860_000));
        compare_range(i122::new(111_222_333_444), i122::new(111_222_444_555));
        compare_range(i5::MIN, i5::MAX);
        compare_range(i23::MIN, i23::MAX);
        compare_range(i48::new(999_444), i48::new(1_005_000));
        compare_range(i99::new(12345), i99::new(54321));

        fn compare_range<T, const BITS: usize>(arb_start: Int<T, BITS>, arb_end: Int<T, BITS>)
        where
            Int<T, BITS>: Step + Number<UnderlyingType = T>,
            T: SignedNumberType + Copy + Step,
        {
            let arbint_range = (arb_start..=arb_end).map(UInt::value);
            let underlying_range = arb_start.value()..=arb_end.value();

            assert!(arbint_range.eq(underlying_range));
        }
    }

    #[test]
    fn forward_checked() {
        // In range
        assert_eq!(Some(i7::new(121)), Step::forward_checked(i7::new(120), 1));
        assert_eq!(Some(i7::new(127)), Step::forward_checked(i7::new(120), 7));

        // Out of range
        assert_eq!(None, Step::forward_checked(i7::new(120), 8));

        // Out of range for the underlying type
        assert_eq!(None, Step::forward_checked(i7::new(120), 140));
    }

    #[test]
    fn backward_checked() {
        // In range
        assert_eq!(Some(i7::new(1)), Step::backward_checked(i7::new(10), 9));
        assert_eq!(Some(i7::new(0)), Step::backward_checked(i7::new(10), 10));

        // Out of range (for both the arbitrary int and and the underlying type)
        assert_eq!(None, Step::backward_checked(i7::new(10), 11));
    }

    #[test]
    fn steps_between() {
        assert_eq!(Some(0), Step::steps_between(&i50::new(50), &i50::new(50)));

        assert_eq!(Some(4), Step::steps_between(&i24::new(5), &i24::new(9)));
        assert_eq!(None, Step::steps_between(&i24::new(9), &i24::new(5)));

        // this assumes usize is <= 64 bits. a test like this one exists in `core::iter::step`.
        assert_eq!(
            Some(usize::MAX),
            Step::steps_between(&i125::new(0x7), &i125::new(0x1_0000_0000_0000_0006))
        );
        assert_eq!(
            None,
            Step::steps_between(&i125::new(0x7), &i125::new(0x1_0000_0000_0000_0007))
        );
    }
}
