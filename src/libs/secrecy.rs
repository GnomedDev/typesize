use secrecy::{zeroize::Zeroize, ExposeSecret};

use crate::TypeSize;

impl<T: Zeroize + TypeSize> TypeSize for secrecy::Secret<T> {
    fn extra_size(&self) -> usize {
        self.expose_secret().extra_size()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        self.expose_secret().get_collection_item_count()
    }
}
