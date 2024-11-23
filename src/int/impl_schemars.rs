use schemars::JsonSchema;

use crate::Number;

use super::{Int, SignedNumberType};

impl<T, const BITS: usize> JsonSchema for Int<T, BITS>
where
    Self: Number<UnderlyingType = T>,
    T: SignedNumberType
{
    fn schema_name() -> String {
        ["int", &BITS.to_string()].concat()
    }

    fn json_schema(_gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::{NumberValidation, Schema, SchemaObject};
        let schema_object = SchemaObject {
            instance_type: Some(schemars::schema::InstanceType::Integer.into()),
            format: Some(Self::schema_name()),
            number: Some(Box::new(NumberValidation {
                // can be done with https://github.com/rust-lang/rfcs/pull/2484
                // minimum: Some(Self::MIN.value().try_into().ok().unwrap()),
                // maximum: Some(Self::MAX.value().try_into().ok().unwrap()),
                ..Default::default()
            })),
            ..Default::default()
        };
        Schema::Object(schema_object)
    }
}

#[cfg(test)]
mod tests {
    use crate::int::aliases::*;

    #[test]
    fn schemars() {
        use schemars::schema_for;
        let mut i8 = schema_for!(i8);
        let i9 = schema_for!(i9);
        assert_eq!(
            i8.schema.format.clone().unwrap().replace("8", "9"),
            i9.schema.format.clone().unwrap()
        );
        i8.schema.format = i9.schema.format.clone();
        assert_eq!(
            i8.schema
                .metadata
                .clone()
                .unwrap()
                .title
                .unwrap()
                .replace("8", "9"),
            i9.schema.metadata.clone().unwrap().title.unwrap()
        );
        i8.schema.metadata = i9.schema.metadata.clone();
        i8.schema.number = i9.schema.number.clone();
        assert_eq!(i8, i9);
    }
}
