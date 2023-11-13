use std::collections::HashMap;

use crate::TypeSize;

// TODO: Figure out more accurate overheads per Map and replace calls to this with more accurate calculations.
pub(crate) fn generic_map_extra_size<'a, K: TypeSize + 'a, V: TypeSize + 'a>(
    elements: impl Iterator<Item = (&'a K, &'a V)>,
    capacity: usize,
    length: usize,
) -> usize {
    let element_size: usize = elements.map(|(k, v)| k.get_size() + v.get_size()).sum();

    let free_space = capacity - length;
    let free_size = free_space * (std::mem::size_of::<K>() + std::mem::size_of::<V>());

    element_size + free_size
}

impl<K: TypeSize, V: TypeSize> TypeSize for HashMap<K, V> {
    fn extra_size(&self) -> usize {
        generic_map_extra_size(self.iter(), self.capacity(), self.len())
    }
}
