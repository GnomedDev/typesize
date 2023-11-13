use std::num::*;

use crate::{sizeof_impl, TypeSize};

impl<const N: usize, T: TypeSize> TypeSize for [T; N] {
    fn extra_size(&self) -> usize {
        self.iter().map(T::extra_size).sum()
    }
}

impl<T: ?Sized + TypeSize> TypeSize for Box<T> {
    fn extra_size(&self) -> usize {
        <T as TypeSize>::get_size(self)
    }
}

#[rustfmt::skip]
sizeof_impl!(
    (),
    bool,
    f32, f64,
    std::time::Duration,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize
);
