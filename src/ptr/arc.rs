#![allow(clippy::module_name_repetitions)]

use alloc::sync::Arc;
use core::{marker::PhantomData, sync::atomic::AtomicUsize};

use crate::{if_typesize_details, TypeSize};

use super::{sealed::ShouldCountInner, Borrowed, Owned};

/// A wrapper around [`Arc`] to implement [`TypeSize`] by allowing you to decide if to count the inner T's size.
///
/// # Examples
/// ```
/// # use std::{cell::Cell, sync::Arc};
/// # use typesize::{TypeSize, ptr::{SizableArc, Owned, Borrowed}};
/// #
/// let arc = Arc::new(0);
/// let arc_borrow: SizableArc<u8, Borrowed> = arc.clone().into();
/// let arc_owner: SizableArc<u8, Owned> = arc.into();
///
/// // Just counts the pointer to the internal `ArcBox`.
/// assert_eq!(arc_borrow.get_size(), 0_usize.get_size());
/// // Counts the pointer to the `ArcBox`, plus the two AtomicUsize, and the value.
/// assert_eq!(arc_owner.get_size(), 0_usize.get_size() + (core::mem::size_of::<usize>() * 2) + 0_u8.get_size());
/// ```
pub struct SizableArc<T, SC: ShouldCountInner>(pub Arc<T>, PhantomData<SC>);

impl<T, SC: ShouldCountInner> From<Arc<T>> for SizableArc<T, SC> {
    fn from(value: Arc<T>) -> Self {
        SizableArc(value, PhantomData)
    }
}

impl<T, SC: ShouldCountInner> core::ops::Deref for SizableArc<T, SC> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: core::fmt::Debug, SC: ShouldCountInner> core::fmt::Debug for SizableArc<T, SC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> TypeSize for SizableArc<T, Borrowed> {}
impl<T: TypeSize> TypeSize for SizableArc<T, Owned> {
    fn extra_size(&self) -> usize {
        T::get_size(&self.0) + (core::mem::size_of::<AtomicUsize>() * 2)
    }

    if_typesize_details! {
        fn get_collection_item_count(&self) -> Option<usize> {
            T::get_collection_item_count(&self.0)
        }

        fn get_size_details(&self) -> alloc::vec::Vec<crate::Field> {
            T::get_size_details(&self.0)
        }
    }
}
