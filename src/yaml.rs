use serde_yaml::{Mapping, Value};

pub(crate) fn mapping_string<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a str> {
    mapping
        .get(Value::String(key.to_owned()))
        .and_then(Value::as_str)
}

pub(crate) fn set_mapping_value(mapping: &mut Mapping, key: &str, value: Value) {
    mapping.insert(Value::String(key.to_owned()), value);
}
