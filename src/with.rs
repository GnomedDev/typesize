use std::{rc::Rc, sync::Arc};

use crate::TypeSize;

macro_rules! size_of_with {
    ($(
        #[type_name = $type_name:literal]
        $pub:vis fn $name:ident<T: ?Sized>(_: $type:ty) -> usize;
    )*) => {$(
        #[doc = concat!("Returns the extra size from the pointer/metadata part of ",  $type_name, ".\n")]
        #[doc = "This does not consider the inner `T`'s size, and therefore is always `0`."]
        $pub fn $name<T: ?Sized>(_: &$type) -> usize { 0 }
    )*};
}

size_of_with! {
    #[type_name = "a reference"]
    pub fn ref_size<T: ?Sized>(_: &T) -> usize;

    #[type_name = "a mutable reference"]
    pub fn ref_mut_size<T: ?Sized>(_: &mut T) -> usize;

    #[type_name = "a constant pointer"]
    pub fn ptr_size<T: ?Sized>(_: *const T) -> usize;

    #[type_name = "a mutable pointer"]
    pub fn ptr_mut_size<T: ?Sized>(_: *mut T) -> usize;

    #[type_name = "a Rc"]
    pub fn rc_size_without_inner<T: ?Sized>(_: Rc<T>) -> usize;

    #[type_name = "an Arc"]
    pub fn arc_size_without_inner<T: ?Sized>(_: Arc<T>) -> usize;
}

/// Returns the extra size of an `Rc<T>`, assuming that this is the only copy.
pub fn rc_size_with_inner<T: TypeSize>(val: &Rc<T>) -> usize {
    <T as TypeSize>::get_size(val)
        + Rc::strong_count(val).get_size()
        + Rc::weak_count(val).get_size()
}

/// Returns the extra size of an `Arc<T>`, assuming that this is the only copy.
///
/// Use [`arc_size_without_inner`] to avoid counting the copies.
pub fn arc_size_with_inner<T: TypeSize>(val: &Arc<T>) -> usize {
    <T as TypeSize>::extra_size(val)
        + Arc::strong_count(val).get_size()
        + Arc::weak_count(val).get_size()
}
