use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{uint::UInt, Number};

use super::UnsignedNumberType;

impl<T, const BITS: usize> Serialize for UInt<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: UnsignedNumberType + Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

impl<'de, T: Display, const BITS: usize> Deserialize<'de> for UInt<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: UnsignedNumberType + Deserialize<'de> + PartialOrd,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = T::deserialize(deserializer)?;

        Self::try_new(value).map_err(|err| serde::de::Error::custom(err))
    }
}

#[cfg(test)]
mod tests {
    use crate::uint::aliases::*;

    #[test]
    fn serde() {
        use serde_test::{assert_de_tokens_error, assert_tokens, Token};

        let a = u7::new(0b0101_0101);
        assert_tokens(&a, &[Token::U8(0b0101_0101)]);

        let b = u63::new(0x1234_5678_9ABC_DEFE);
        assert_tokens(&b, &[Token::U64(0x1234_5678_9ABC_DEFE)]);

        // This requires https://github.com/serde-rs/test/issues/18 (Add Token::I128 and Token::U128 to serde_test)
        // let c = u127::new(0x1234_5678_9ABC_DEFE_DCBA_9876_5432_1010);
        // assert_tokens(&c, &[Token::U128(0x1234_5678_9ABC_DEFE_DCBA_9876_5432_1010)]);

        assert_de_tokens_error::<u2>(
            &[Token::U8(0b0101_0101)],
            "invalid value: integer `85`, expected a value between `0` and `3`",
        );

        assert_de_tokens_error::<u100>(
            &[Token::I64(-1)],
            "invalid value: integer `-1`, expected u128",
        );
    }
}
