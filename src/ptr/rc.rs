#![allow(clippy::module_name_repetitions)]

use alloc::rc::Rc;
use core::{cell::Cell, marker::PhantomData};

use crate::{if_typesize_details, TypeSize};

use super::{sealed::ShouldCountInner, Borrowed, Owned};

/// A wrapper around [`Rc`] to implement [`TypeSize`] by allowing you to decide if to count the inner T's size.
///
/// # Examples
/// ```
/// # use std::{cell::Cell, rc::Rc};
/// # use typesize::{TypeSize, ptr::{SizableRc, Owned, Borrowed}};
/// #
/// let rc = Rc::new(0);
/// let rc_borrow: SizableRc<u8, Borrowed> = rc.clone().into();
/// let rc_owner: SizableRc<u8, Owned> = rc.into();
///
/// // Just counts the pointer to the internal `RcBox`.
/// assert_eq!(rc_borrow.get_size(), 0_usize.get_size());
/// // Counts the pointer to the `RcBox`, plus the two Cells, and the value.
/// assert_eq!(rc_owner.get_size(), 0_usize.get_size() + (core::mem::size_of::<usize>() * 2) + 0_u8.get_size());
/// ```
pub struct SizableRc<T, SC: ShouldCountInner>(pub Rc<T>, PhantomData<SC>);

impl<T, SC: ShouldCountInner> From<Rc<T>> for SizableRc<T, SC> {
    fn from(value: Rc<T>) -> Self {
        SizableRc(value, PhantomData)
    }
}

impl<T, SC: ShouldCountInner> core::ops::Deref for SizableRc<T, SC> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: core::fmt::Debug, SC: ShouldCountInner> core::fmt::Debug for SizableRc<T, SC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> TypeSize for SizableRc<T, Borrowed> {}
impl<T: TypeSize> TypeSize for SizableRc<T, Owned> {
    fn extra_size(&self) -> usize {
        T::get_size(&self.0) + (core::mem::size_of::<Cell<usize>>() * 2)
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
