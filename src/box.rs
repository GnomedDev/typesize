use crate::TypeSize;

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
        std::mem::size_of::<u8>() * self.len()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<T: ?Sized + TypeSize> TypeSize for Box<T> {
    fn extra_size(&self) -> usize {
        <T as TypeSize>::get_size(self)
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        <T as TypeSize>::get_collection_item_count(self)
    }
}
