use crate::TypeSize;

impl<T: TypeSize + Copy> TypeSize for core::cell::Cell<T> {
    fn extra_size(&self) -> usize {
        self.get().extra_size()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        self.get().get_collection_item_count()
    }

    #[cfg(feature = "details")]
    fn get_size_details(&self) -> Vec<crate::Field> {
        self.get().get_size_details()
    }
}

impl<T: TypeSize> TypeSize for core::cell::RefCell<T> {
    fn extra_size(&self) -> usize {
        self.borrow().extra_size()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        self.borrow().get_collection_item_count()
    }

    #[cfg(feature = "details")]
    fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
        self.borrow().get_size_details()
    }
}

impl<T: TypeSize> TypeSize for core::cell::OnceCell<T> {
    fn extra_size(&self) -> usize {
        self.get().map_or(0, TypeSize::extra_size)
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        self.get().and_then(TypeSize::get_collection_item_count)
    }

    #[cfg(feature = "details")]
    fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
        self.get().map_or(Vec::new(), TypeSize::get_size_details)
    }
}
