use bytes::{Bytes, BytesMut};

use crate::TypeSize;

impl TypeSize for Bytes {
    fn extra_size(&self) -> usize {
        core::mem::size_of::<u8>() * self.len()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl TypeSize for BytesMut {
    fn extra_size(&self) -> usize {
        core::mem::size_of::<u8>() * self.capacity()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}
