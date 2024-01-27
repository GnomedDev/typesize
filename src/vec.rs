use alloc::{collections::VecDeque, string::String, vec::Vec};

use crate::TypeSize;

pub(crate) fn generic_vec_extra_size<'a, T: TypeSize + 'a>(
    iter: impl Iterator<Item = &'a T>,
    capacity: usize,
    len: usize,
) -> usize {
    iter.map(TypeSize::get_size).sum::<usize>() + (capacity - len) * core::mem::size_of::<T>()
}

impl<T: TypeSize> TypeSize for Vec<T> {
    fn extra_size(&self) -> usize {
        generic_vec_extra_size::<T>(self.iter(), self.capacity(), self.len())
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<T: TypeSize> TypeSize for VecDeque<T> {
    fn extra_size(&self) -> usize {
        generic_vec_extra_size::<T>(self.iter(), self.capacity(), self.len())
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl TypeSize for String {
    fn extra_size(&self) -> usize {
        core::mem::size_of::<u8>() * self.capacity()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}
