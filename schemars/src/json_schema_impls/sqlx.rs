use crate::gen::SchemaGenerator;
use crate::schema::Schema;
use crate::JsonSchema;
use std::borrow::Cow;

impl<T: JsonSchema> JsonSchema for sqlx::types::Json<T> {
    fn is_referenceable() -> bool {
        T::is_referenceable()
    }

    fn schema_name() -> String {
        T::schema_name()
    }

    fn schema_id() -> Cow<'static, str> {
        T::schema_id()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        T::json_schema(gen)
    }

    fn _schemars_private_non_optional_json_schema(gen: &mut SchemaGenerator) -> Schema {
        T::_schemars_private_non_optional_json_schema(gen)
    }

    fn _schemars_private_is_option() -> bool {
        T::_schemars_private_is_option()
    }
}
