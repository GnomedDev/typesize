use smol_str::{SmolStr};

use crate::TypeSize;

impl TypeSize for SmolStr {
    fn extra_size(&self) -> usize {
        if self.is_heap_allocated() {
            size_of::<u8>() * self.as_str().len()
        } else {
            0
        }
    }
}