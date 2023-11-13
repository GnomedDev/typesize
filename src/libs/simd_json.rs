use crate::{sizeof_impl, TypeSize};

sizeof_impl!(simd_json::StaticNode);

impl TypeSize for simd_json::OwnedValue {
    fn extra_size(&self) -> usize {
        match self {
            simd_json::OwnedValue::Static(value) => value.extra_size(),
            simd_json::OwnedValue::String(value) => value.extra_size(),
            simd_json::OwnedValue::Object(value) => value.extra_size(),
            simd_json::OwnedValue::Array(value) => value.extra_size(),
        }
    }
}
