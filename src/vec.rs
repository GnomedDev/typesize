use std::collections::VecDeque;

use crate::TypeSize;

pub(crate) fn generic_vec_extra_size<'a, T: TypeSize + 'a>(
    iter: impl Iterator<Item = &'a T>,
    capacity: usize,
    len: usize,
) -> usize {
    iter.map(TypeSize::get_size).sum::<usize>() + (capacity - len) * std::mem::size_of::<T>()
}

impl<T: TypeSize> TypeSize for Vec<T> {
    fn extra_size(&self) -> usize {
        generic_vec_extra_size::<T>(self.iter(), self.capacity(), self.len())
    }
}

impl<T: TypeSize> TypeSize for VecDeque<T> {
    fn extra_size(&self) -> usize {
        generic_vec_extra_size::<T>(self.iter(), self.capacity(), self.len())
    }
}

impl TypeSize for String {
    fn extra_size(&self) -> usize {
        std::mem::size_of::<u8>() * self.capacity()
    }
}
