use hashbrown::{HashMap, HashSet};

use crate::{map::generic_map_extra_size, vec::generic_vec_extra_size, TypeSize};

impl<K: TypeSize, V: TypeSize, S> TypeSize for HashMap<K, V, S> {
    fn extra_size(&self) -> usize {
        generic_map_extra_size(self.iter(), self.capacity(), self.len())
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<T: TypeSize, S> TypeSize for HashSet<T, S> {
    fn extra_size(&self) -> usize {
        generic_vec_extra_size(self.iter(), self.capacity(), self.len())
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}
