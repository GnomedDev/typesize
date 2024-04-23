use secrecy::{zeroize::Zeroize, ExposeSecret};

use crate::{if_typesize_details, TypeSize};

impl<T: Zeroize + TypeSize> TypeSize for secrecy::Secret<T> {
    fn extra_size(&self) -> usize {
        self.expose_secret().extra_size()
    }

    if_typesize_details! {
        fn get_collection_item_count(&self) -> Option<usize> {
            self.expose_secret().get_collection_item_count()
        }

        fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
            self.expose_secret().get_size_details()
        }
    }
}
