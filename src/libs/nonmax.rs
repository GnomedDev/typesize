#[rustfmt::skip]
use nonmax::{
    NonMaxU8, NonMaxU16, NonMaxU32, NonMaxU64, NonMaxU128, NonMaxUsize,
    NonMaxI8, NonMaxI16, NonMaxI32, NonMaxI64, NonMaxI128, NonMaxIsize
};

use crate::{sizeof_impl, TypeSize};

#[rustfmt::skip]
sizeof_impl!(
    NonMaxU8, NonMaxU16, NonMaxU32, NonMaxU64, NonMaxU128, NonMaxUsize,
    NonMaxI8, NonMaxI16, NonMaxI32, NonMaxI64, NonMaxI128, NonMaxIsize
);
