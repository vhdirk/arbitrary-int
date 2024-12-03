use super::{AInt, AIntContainer};
use crate::Number;

impl<T, const BITS: usize> defmt::Format for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + defmt::Format,
{
    #[inline]
    fn format(&self, f: defmt::Formatter) {
        self.value.format(f)
    }
}
