use std::collections::HashSet;

use crate::{vec::generic_vec_extra_size, TypeSize};

impl<T: TypeSize, S> TypeSize for HashSet<T, S> {
    fn extra_size(&self) -> usize {
        // TODO: Not this!!
        generic_vec_extra_size::<T>(self.iter(), self.capacity(), self.len())
    }
}
