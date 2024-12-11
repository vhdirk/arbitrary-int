use std::collections::BTreeMap;

use crate::{AInt, AIntContainer, Number};

impl<T, const BITS: usize> borsh::BorshSerialize for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + borsh::BorshSerialize,
{
    fn serialize<W: borsh::io::Write>(&self, writer: &mut W) -> borsh::io::Result<()> {
        let serialized_byte_count = <Self as Number>::BYTES as usize;
        let mut buffer = [0u8; 16];
        self.value.serialize(&mut &mut buffer[..])?;
        writer.write(&buffer[0..serialized_byte_count])?;

        Ok(())
    }
}

impl<T, const BITS: usize> borsh::BorshDeserialize for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer + borsh::BorshDeserialize + PartialOrd<T>,
{
    fn deserialize_reader<R: borsh::io::Read>(reader: &mut R) -> borsh::io::Result<Self> {
        // Ideally, we'd want a buffer of size `BITS >> 3` or `size_of::<T>`, but that's not possible
        // with arrays at present (feature(generic_const_exprs), once stable, will allow this).
        // vec! would be an option, but an allocation is not expected at this level.
        // Therefore, allocate a 16 byte buffer and take a slice out of it.
        let serialized_byte_count = <Self as Number>::BYTES as usize;
        let underlying_byte_count = core::mem::size_of::<T>();
        let mut buf = [0u8; 16];

        // Read from the source, advancing cursor by the exact right number of bytes
        reader.read(&mut buf[..serialized_byte_count])?;

        // Deserialize the underlying type. We have to pass in the correct number of bytes of the
        // underlying type (or more, but let's be precise). The unused bytes are all still zero
        let value = T::deserialize(&mut &buf[..underlying_byte_count])?;

        match Self::try_new(value) {
            Ok(v) => Ok(v),
            Err(err) => Err(borsh::io::Error::new(
                borsh::io::ErrorKind::InvalidData,
                err.to_string(),
            )),
        }
    }
}

impl<T, const BITS: usize> borsh::BorshSchema for AInt<T, BITS>
where
    Self: Number<Container = T>,
    T: AIntContainer,
{
    fn add_definitions_recursively(
        definitions: &mut BTreeMap<borsh::schema::Declaration, borsh::schema::Definition>,
    ) {
        definitions.insert(
            Self::declaration(),
            borsh::schema::Definition::Primitive(Self::BYTES as u8),
        );
    }

    fn declaration() -> borsh::schema::Declaration {
        format!("{}{}", if Self::SIGNED { "i" } else { "u" }, Self::BITS)
    }
}

#[cfg(test)]
mod tests {
    use crate::Number;
    use crate::{aliases::*, AInt};
    use borsh::schema::BorshSchemaContainer;
    use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
    use std::fmt::Debug;

    fn test_roundtrip<T: Number + BorshSerialize + BorshDeserialize + PartialEq + Eq + Debug>(
        input: T,
        expected_buffer: &[u8],
    ) {
        let mut buf = Vec::new();

        // Serialize and compare against expected
        input.serialize(&mut buf).unwrap();
        assert_eq!(buf, expected_buffer);

        // Add to the buffer a second time - this is a better test for the deserialization
        // as it ensures we request the correct number of bytes
        input.serialize(&mut buf).unwrap();

        // Deserialize back and compare against input
        let output = T::deserialize(&mut buf.as_ref()).unwrap();
        let output2 = T::deserialize(&mut &buf[buf.len() / 2..]).unwrap();
        assert_eq!(input, output);
        assert_eq!(input, output2);
    }

    #[test]
    fn test_serialize_deserialize() {
        // Run against plain u64 first (not an arbitrary_int)
        test_roundtrip(
            0x12345678_9ABCDEF0u64,
            &[0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12],
        );

        // Now try various arbitrary ints
        test_roundtrip(u1::new(0b0), &[0]);
        test_roundtrip(u1::new(0b1), &[1]);
        test_roundtrip(u6::new(0b101101), &[0b101101]);
        test_roundtrip(u14::new(0b110101_11001101), &[0b11001101, 0b110101]);
        test_roundtrip(
            u72::new(0x36_01234567_89ABCDEF),
            &[0xEF, 0xCD, 0xAB, 0x89, 0x67, 0x45, 0x23, 0x01, 0x36],
        );

        // Pick a byte boundary (80; test one below and one above to ensure we get the right number
        // of bytes)
        test_roundtrip(
            u79::MAX,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F],
        );
        test_roundtrip(
            u80::MAX,
            &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF],
        );
        test_roundtrip(
            u81::MAX,
            &[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x01,
            ],
        );

        // Test actual u128 and arbitrary u128 (which is a legal one, though not a predefined)
        test_roundtrip(
            u128::MAX,
            &[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ],
        );
        test_roundtrip(
            AInt::<u128, 128>::MAX,
            &[
                0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                0xFF, 0xFF,
            ],
        );
    }

    fn verify_byte_count_in_schema<T: BorshSchema + ?Sized>(expected_byte_count: u8, name: &str) {
        let schema = BorshSchemaContainer::for_type::<T>();
        match schema.get_definition(name).expect("exists") {
            borsh::schema::Definition::Primitive(byte_count) => {
                assert_eq!(*byte_count, expected_byte_count);
            }
            _ => panic!("unexpected schema"),
        }
    }

    #[test]
    fn test_schema_byte_count() {
        verify_byte_count_in_schema::<u1>(1, "u1");

        verify_byte_count_in_schema::<u7>(1, "u7");

        verify_byte_count_in_schema::<AInt<u8, 8>>(1, "u8");
        verify_byte_count_in_schema::<AInt<u32, 8>>(1, "u8");

        verify_byte_count_in_schema::<u9>(2, "u9");

        verify_byte_count_in_schema::<u15>(2, "u15");
        verify_byte_count_in_schema::<AInt<u128, 15>>(2, "u15");

        verify_byte_count_in_schema::<u63>(8, "u63");

        verify_byte_count_in_schema::<u65>(9, "u65");
    }
}
