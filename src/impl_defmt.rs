use crate::Number;

use super::{AInt, NumberType};


impl<T, const BITS: usize> defmt::Format for AInt<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: NumberType + defmt::Format,
{
    #[inline]
    fn format(&self, f: defmt::Formatter) {
        self.value.format(f)
    }
}