use crate::{traits::BitsSpec, Number};
use super::{AInt, AIntContainer};


impl<T, Bits> defmt::Format for AInt<T, Bits>
where
    Self: Number<Container = T, Bits = Bits>,
    T: AIntContainer + defmt::Format,
    Bits: BitsSpec,
    <T as AIntContainer>::Bits: typenum::IsGreaterOrEqual<Bits, Output = typenum::True>,
{
    #[inline]
    fn format(&self, f: defmt::Formatter) {
        self.value.format(f)
    }
}