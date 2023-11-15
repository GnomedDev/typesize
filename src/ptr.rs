//! Wrappers around &T and &mut T to implement [`TypeSize`].
//!
//! These have to be explicitly added to prevent automatic deref causing
//! a field which does not implement Typesize to instead return the size
//! of a reference.

use std::ops::{Deref, DerefMut};

use crate::TypeSize;

/// A wrapper around &T to implement [`TypeSize`].
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ref<'a, T: ?Sized>(pub &'a T);

impl<'a, T: ?Sized> TypeSize for Ref<'a, T> {}
impl<'a, T: ?Sized> Deref for Ref<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

/// A wrapper around &mut T to implement [`TypeSize`].
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RefMut<'a, T: ?Sized>(pub &'a mut T);

impl<'a, T: ?Sized> TypeSize for RefMut<'a, T> {}
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
