use crate::TypeSize;

impl<T: TypeSize> TypeSize for parking_lot::Mutex<T> {
    fn extra_size(&self) -> usize {
        self.lock().extra_size()
    }
}

impl<T: TypeSize> TypeSize for parking_lot::RwLock<T> {
    fn extra_size(&self) -> usize {
        self.read().extra_size()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        self.read().get_collection_item_count()
    }
}
