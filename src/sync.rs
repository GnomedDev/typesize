// TODO: Implement the calculations for the heap based Mutex impls on some platforms (like MacOS).

use std::sync::{Mutex, PoisonError, RwLock};

use crate::{if_typesize_details, TypeSize};

impl<T: TypeSize> TypeSize for Mutex<T> {
    fn extra_size(&self) -> usize {
        self.lock()
            .unwrap_or_else(PoisonError::into_inner)
            .extra_size()
    }

    if_typesize_details! {
        fn get_collection_item_count(&self) -> Option<usize> {
            self.lock()
                .unwrap_or_else(PoisonError::into_inner)
                .get_collection_item_count()
        }

        fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
            self.lock()
                .unwrap_or_else(PoisonError::into_inner)
                .get_size_details()
        }
    }
}

impl<T: TypeSize> TypeSize for RwLock<T> {
    fn extra_size(&self) -> usize {
        self.read()
            .unwrap_or_else(PoisonError::into_inner)
            .extra_size()
    }

    if_typesize_details! {
        fn get_collection_item_count(&self) -> Option<usize> {
            self.read()
                .unwrap_or_else(PoisonError::into_inner)
                .get_collection_item_count()
        }

        fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
            self.read()
                .unwrap_or_else(PoisonError::into_inner)
                .get_size_details()
        }
    }
}
