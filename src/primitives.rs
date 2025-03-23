use core::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
        NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize, Wrapping,
    },
    sync::atomic::{
        AtomicBool, AtomicI16, AtomicI8, AtomicIsize, AtomicU16, AtomicU8, AtomicUsize,
    },
};

use crate::{sizeof_impl, TypeSize};

impl<const N: usize, T: TypeSize> TypeSize for [T; N] {
    fn extra_size(&self) -> usize {
        self.iter().map(T::extra_size).sum()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(N)
    }
}

#[rustfmt::skip]
sizeof_impl!(
    (),
    f32, f64,
    bool, AtomicBool,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    AtomicU8, AtomicU16, AtomicUsize,
    AtomicI8, AtomicI16, AtomicIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    Wrapping<u8>, Wrapping<u16>, Wrapping<u32>, Wrapping<u64>, Wrapping<u128>, Wrapping<usize>,
    Wrapping<i8>, Wrapping<i16>, Wrapping<i32>, Wrapping<i64>, Wrapping<i128>, Wrapping<isize>
);

#[cfg(target_has_atomic = "32")]
sizeof_impl!(core::sync::atomic::AtomicU32, core::sync::atomic::AtomicI32);

#[cfg(target_has_atomic = "64")]
sizeof_impl!(core::sync::atomic::AtomicU64, core::sync::atomic::AtomicI64);

#[cfg(feature = "saturating_impls")]
use core::num::Saturating;

#[cfg(feature = "saturating_impls")]
#[rustfmt::skip]
sizeof_impl!(
    Saturating<u8>, Saturating<u16>, Saturating<u32>, Saturating<u64>, Saturating<u128>, Saturating<usize>,
    Saturating<i8>, Saturating<i16>, Saturating<i32>, Saturating<i64>, Saturating<i128>, Saturating<isize>
);
