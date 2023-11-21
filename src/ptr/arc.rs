#![allow(clippy::module_name_repetitions)]

use std::{
    marker::PhantomData,
    sync::{atomic::AtomicUsize, Arc},
};

use crate::TypeSize;

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
/// assert_eq!(arc_owner.get_size(), 0_usize.get_size() + (std::mem::size_of::<usize>() * 2) + 0_u8.get_size());
/// ```
pub struct SizableArc<T, SC: ShouldCountInner>(pub Arc<T>, PhantomData<SC>);

impl<T, SC: ShouldCountInner> From<Arc<T>> for SizableArc<T, SC> {
    fn from(value: Arc<T>) -> Self {
        SizableArc(value, PhantomData)
    }
}

impl<T, SC: ShouldCountInner> std::ops::Deref for SizableArc<T, SC> {
    type Target = Arc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: std::fmt::Debug, SC: ShouldCountInner> std::fmt::Debug for SizableArc<T, SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> TypeSize for SizableArc<T, Borrowed> {}
impl<T: TypeSize> TypeSize for SizableArc<T, Owned> {
    fn extra_size(&self) -> usize {
        T::get_size(&self.0) + (std::mem::size_of::<AtomicUsize>() * 2)
    }
}
