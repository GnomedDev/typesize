use core::hash::{BuildHasher, Hash};
use hashbrown_15::{HashMap, HashSet};

use crate::TypeSize;

impl<K: Eq + Hash + TypeSize, V: TypeSize, S: BuildHasher> TypeSize for HashMap<K, V, S> {
    fn extra_size(&self) -> usize {
        let base_extra_size = self.allocation_size();
        let extra_extra_size = self
            .iter()
            .map(|(k, v)| K::extra_size(k) + V::extra_size(v))
            .sum::<usize>();

        base_extra_size + extra_extra_size
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<T: Eq + Hash + TypeSize, S: BuildHasher> TypeSize for HashSet<T, S> {
    fn extra_size(&self) -> usize {
        self.allocation_size() + self.iter().map(T::extra_size).sum::<usize>()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}
