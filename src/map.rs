use crate::TypeSize;

/// Generalisation over (&K, &V) and types like dashmap's `RefMulti`.
pub(crate) trait EntryRef<K, V> {
    fn get_ref(&self) -> (&K, &V);
}

impl<K, V> EntryRef<K, V> for (&K, &V) {
    fn get_ref(&self) -> (&K, &V) {
        *self
    }
}

// TODO: Figure out more accurate overheads per Map and replace calls to this with more accurate calculations.
pub(crate) fn generic_map_extra_size<'a, K: TypeSize + 'a, V: TypeSize + 'a>(
    elements: impl Iterator<Item = impl EntryRef<K, V>>,
    capacity: usize,
    length: usize,
) -> usize {
    let element_size: usize = elements
        .map(|p| {
            let (key, value) = p.get_ref();
            key.get_size() + value.get_size()
        })
        .sum();

    let free_space = capacity - length;
    let free_size = free_space * (core::mem::size_of::<K>() + core::mem::size_of::<V>());

    element_size + free_size
}

#[cfg(feature = "std")]
impl<K: TypeSize, V: TypeSize, S> TypeSize for std::collections::HashMap<K, V, S> {
    fn extra_size(&self) -> usize {
        generic_map_extra_size::<K, V>(self.iter(), self.capacity(), self.len())
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}
