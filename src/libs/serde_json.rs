use crate::{sizeof_impl, TypeSize};

sizeof_impl!(serde_json::Number);

impl TypeSize for serde_json::Map<String, serde_json::Value> {
    fn extra_size(&self) -> usize {
        // TODO: Find a way to get at the internals of serde_json's map.
        crate::map::generic_map_extra_size(self.iter(), self.len(), self.len())
    }
}

impl TypeSize for serde_json::Value {
    fn extra_size(&self) -> usize {
        match self {
            serde_json::Value::Null => 0,
            serde_json::Value::Bool(value) => value.extra_size(),
            serde_json::Value::Number(value) => value.extra_size(),
            serde_json::Value::String(value) => value.extra_size(),
            serde_json::Value::Array(value) => value.extra_size(),
            serde_json::Value::Object(value) => value.extra_size(),
        }
    }
}
