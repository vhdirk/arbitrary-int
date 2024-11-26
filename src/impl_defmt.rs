use crate::Number;

use super::{AInt, UnsignedNumberType};


impl<T, const BITS: usize> defmt::Format for AInt<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: UnsignedNumberType + defmt::Format,
{
    #[inline]
    fn format(&self, f: defmt::Formatter) {
        self.value.format(f)
    }
}