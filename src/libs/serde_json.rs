use alloc::string::String;

use crate::{sizeof_impl, TypeSize};

sizeof_impl!(serde_json::Number);

impl TypeSize for serde_json::Map<String, serde_json::Value> {
    fn extra_size(&self) -> usize {
        // TODO: Find a way to get at the internals of serde_json's map.
        crate::map::generic_map_extra_size(self.iter(), self.len(), self.len())
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl TypeSize for serde_json::Value {
    fn extra_size(&self) -> usize {
        match self {
            Self::Null => 0,
            Self::Bool(value) => value.extra_size(),
            Self::Number(value) => value.extra_size(),
            Self::String(value) => value.extra_size(),
            Self::Array(value) => value.extra_size(),
            Self::Object(value) => value.extra_size(),
        }
    }
}
