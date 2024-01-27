//! Wrappers around pointer/reference types to correctly implement [`TypeSize`].

use core::ops::{Deref, DerefMut};

use crate::TypeSize;

pub use arc::SizableArc;
pub use rc::SizableRc;

mod arc;
mod rc;

macro_rules! create_ref {
    (
        $(#[$meta:meta]),*
        pub struct $name:ident<$( $lt:lifetime, )? T: ?Sized>(pub $inner:ty)
    ) => {
        $(#[$meta]),*
        #[doc = concat!("A wrapper around `", stringify!($inner), "` to implement [`TypeSize`].")]
        ///
        /// This does not consider the size of the inner `T`, simply the size of the pointer.
        pub struct $name<$($lt, )? T: ?Sized>(pub $inner);

        impl<$($lt, )? T: ?Sized> TypeSize for $name<$($lt, )? T> {}
    };
}

create_ref!(
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Ref<'a, T: ?Sized>(pub &'a T)
);

create_ref!(
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct RefMut<'a, T: ?Sized>(pub &'a mut T)
);

impl<'a, T: ?Sized> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T: ?Sized> Deref for RefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<'a, T: ?Sized> DerefMut for RefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

create_ref!(pub struct Ptr<T: ?Sized>(pub *const T));
create_ref!(pub struct PtrMut<T: ?Sized>(pub *mut T));

mod sealed {
    pub trait ShouldCountInner {}

    impl ShouldCountInner for super::Borrowed {}
    impl ShouldCountInner for super::Owned {}
}

/// Marker type for reference counted types such as [`SizableRc`] or [`SizableArc`]
pub struct Borrowed;
/// Marker type for reference counted types such as [`SizableRc`] or [`SizableArc`]
pub struct Owned;
