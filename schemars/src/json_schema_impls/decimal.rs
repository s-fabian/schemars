use crate::gen::SchemaGenerator;
use crate::schema::*;
use crate::JsonSchema;
use std::borrow::Cow;

macro_rules! decimal_impl {
    ($type:ty) => {
        impl JsonSchema for $type {
            no_ref_schema!();

            fn schema_name() -> String {
                "Decimal".to_owned()
            }

            fn schema_id() -> Cow<'static, str> {
                Cow::Borrowed("Decimal")
            }

            fn json_schema(_: &mut SchemaGenerator) -> Schema {
                SchemaObject {
                    instance_type: Some(InstanceType::String.into()),
                    string: Some(Box::new(StringValidation {
                        pattern: Some(r"^-?[0-9]+(\.[0-9]+)?$".to_owned()),
                        ..Default::default()
                    })),
                    ..Default::default()
                }
                .into()
            }
        }
    };
}

#[cfg(all(feature = "rust_decimal", not(feature = "rust_decimal_float")))]
decimal_impl!(rust_decimal::Decimal);
#[cfg(all(feature = "rust_decimal", feature = "rust_decimal_float"))]
impl JsonSchema for rust_decimal::Decimal {
    no_ref_schema!();

    fn schema_name() -> String {
        "Decimal".to_owned()
    }

    fn schema_id() -> Cow<'static, str> {
        Cow::Borrowed("Decimal")
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        SchemaObject {
            instance_type: Some(InstanceType::Number.into()),
            ..Default::default()
        }
        .into()
    }
}

#[cfg(feature = "bigdecimal03")]
decimal_impl!(bigdecimal03::BigDecimal);
#[cfg(feature = "bigdecimal04")]
decimal_impl!(bigdecimal04::BigDecimal);
