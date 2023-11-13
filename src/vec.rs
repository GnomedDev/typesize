use std::collections::VecDeque;

use crate::TypeSize;

impl<T: TypeSize> TypeSize for Vec<T> {
    fn extra_size(&self) -> usize {
        self.iter().map(TypeSize::get_size).sum::<usize>()
            + (self.capacity() - self.len()) * std::mem::size_of::<T>()
    }
}

impl<T: TypeSize> TypeSize for VecDeque<T> {
    fn extra_size(&self) -> usize {
        self.iter().map(TypeSize::get_size).sum::<usize>()
            + (self.capacity() - self.len()) * std::mem::size_of::<T>()
    }
}

impl TypeSize for String {
    fn extra_size(&self) -> usize {
        std::mem::size_of::<u8>() * self.capacity()
    }
}
