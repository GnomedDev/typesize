use crate::{if_typesize_details, TypeSize};

macro_rules! implement_for {
    ($krate:ident) => {
        impl<T: TypeSize> TypeSize for $krate::Mutex<T> {
            fn extra_size(&self) -> usize {
                self.lock().extra_size()
            }

            if_typesize_details! {
                fn get_collection_item_count(&self) -> Option<usize> {
                    self.lock().get_collection_item_count()
                }

                fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
                    self.lock().get_size_details()
                }
            }
        }

        impl<T: TypeSize> TypeSize for $krate::RwLock<T> {
            fn extra_size(&self) -> usize {
                self.read().extra_size()
            }

            if_typesize_details! {
                fn get_collection_item_count(&self) -> Option<usize> {
                    self.read().get_collection_item_count()
                }

                fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
                    self.read().get_size_details()
                }
            }
        }
    };
}

#[cfg(feature = "parking_lot")]
implement_for!(parking_lot_12);

#[cfg(feature = "parking_lot_13")]
implement_for!(parking_lot_13);
