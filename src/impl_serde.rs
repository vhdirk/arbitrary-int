use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::Number;

use super::{AInt, NumberType};

impl<T, const BITS: usize> Serialize for AInt<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: NumberType + Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.value.serialize(serializer)
    }
}

impl<'de, T: Display, const BITS: usize> Deserialize<'de> for AInt<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: NumberType + Deserialize<'de> + PartialOrd,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = T::deserialize(deserializer)?;

        Self::try_new(value).map_err(|err| serde::de::Error::custom(err))
    }
}

#[cfg(test)]
mod tests {
    use crate::aliases::*;

    #[test]
    fn serde() {
        use serde_test::{assert_de_tokens_error, assert_tokens, Token};

        let a = u7::new(0b0101_0101_u8);
        assert_tokens(&a, &[Token::U8(0b0101_0101_u8)]);

        let b = u63::new(0x1234_5678_9ABC_DEFE_u64);
        assert_tokens(&b, &[Token::U64(0x1234_5678_9ABC_DEFE_u64)]);

        // This requires https://github.com/serde-rs/test/issues/18 (Add Token::I128 and Token::U128 to serde_test)
        // let c = u127::new(0x1234_5678_9ABC_DEFE_DCBA_9876_5432_1010);
        // assert_tokens(&c, &[Token::U128(0x1234_5678_9ABC_DEFE_DCBA_9876_5432_1010)]);

        assert_de_tokens_error::<u2>(
            &[Token::U8(0b0101_0101_u8)],
            "number too large to fit in target type",
        );

        assert_de_tokens_error::<u100>(
            &[Token::I64(-1)],
            "invalid value: integer `-1`, expected u128",
        );
    }
}
