use crate::Number;

use super::{UInt, UnsignedNumberType};


impl<T, const BITS: usize> defmt::Format for UInt<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: UnsignedNumberType + defmt::Format,
{
    #[inline]
    fn format(&self, f: defmt::Formatter) {
        self.value.format(f)
    }
}