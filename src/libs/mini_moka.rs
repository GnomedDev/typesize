use std::hash::{BuildHasher, Hash};

use crate::{map::generic_map_extra_size, TypeSize};

impl<K: TypeSize + Hash + Eq + PartialEq, V: TypeSize, S: BuildHasher + Clone> TypeSize
    for mini_moka::unsync::Cache<K, V, S>
{
    #[allow(clippy::cast_possible_truncation)]
    fn extra_size(&self) -> usize {
        generic_map_extra_size::<K, V>(
            self.iter(),
            self.entry_count() as usize,
            self.entry_count() as usize,
        )
    }
}

#[cfg(feature = "dashmap")]
impl<'a, K: Eq + Hash, V, S: BuildHasher + Clone> crate::map::EntryRef<K, V>
    for mini_moka::sync::EntryRef<'a, K, V, S>
{
    fn get_ref(&self) -> (&K, &V) {
        self.pair()
    }
}

#[cfg(feature = "dashmap")]
impl<K: TypeSize + Hash + Eq + PartialEq, V: TypeSize, S: BuildHasher + Clone> TypeSize
    for mini_moka::sync::Cache<K, V, S>
{
    #[allow(clippy::cast_possible_truncation)]
    fn extra_size(&self) -> usize {
        generic_map_extra_size::<K, V>(
            self.iter(),
            self.entry_count() as usize,
            self.entry_count() as usize,
        )
    }
}
