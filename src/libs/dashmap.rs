use std::hash::{BuildHasher, Hash};

use dashmap::{DashMap, RwLock, SharedValue};

use crate::TypeSize;

impl<T: TypeSize> TypeSize for RwLock<T> {
    fn extra_size(&self) -> usize {
        self.read().extra_size()
    }
}

impl<T: TypeSize> TypeSize for SharedValue<T> {
    fn extra_size(&self) -> usize {
        self.get().extra_size()
    }
}

impl<K: TypeSize, V: TypeSize, S> TypeSize for DashMap<K, V, S>
where
    K: Eq + Hash,
    S: Default + BuildHasher + Clone,
{
    fn extra_size(&self) -> usize {
        self.shards().iter().map(TypeSize::get_size).sum::<usize>()
    }
}
