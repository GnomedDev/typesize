use alloc::boxed::Box;

use crate::{if_typesize_details, TypeSize};

impl<T: TypeSize> TypeSize for Box<[T]> {
    fn extra_size(&self) -> usize {
        self.iter().map(T::get_size).sum()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl TypeSize for Box<str> {
    fn extra_size(&self) -> usize {
        core::mem::size_of::<u8>() * self.len()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<T: TypeSize> TypeSize for Box<T> {
    fn extra_size(&self) -> usize {
        <T as TypeSize>::get_size(self)
    }

    if_typesize_details! {
        fn get_collection_item_count(&self) -> Option<usize> {
            <T as TypeSize>::get_collection_item_count(self)
        }

        fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
            <T as TypeSize>::get_size_details(self)
        }
    }
}
