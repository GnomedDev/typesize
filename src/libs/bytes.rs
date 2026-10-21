use bytes::{Bytes, BytesMut};
use core::{
    fmt::{self, Debug},
    marker::PhantomData,
    ops::Deref,
};

use crate::{
    ptr::{sealed::ShouldCountInner, Borrowed, Owned},
    TypeSize,
};

pub struct SizableBytes<SC: ShouldCountInner>(pub Bytes, PhantomData<SC>);

impl<SC: ShouldCountInner> From<Bytes> for SizableBytes<SC> {
    fn from(value: Bytes) -> Self {
        Self(value, PhantomData)
    }
}

impl<SC: ShouldCountInner> Deref for SizableBytes<SC> {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<SC: ShouldCountInner> Debug for SizableBytes<SC> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl TypeSize for SizableBytes<Borrowed> {}

impl TypeSize for SizableBytes<Owned> {
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
