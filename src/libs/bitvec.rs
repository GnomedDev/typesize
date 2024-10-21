use bitvec::{array::BitArray, order::BitOrder, store::BitStore, vec::BitVec, view::BitViewSized};

use crate::TypeSize;

impl<A: BitViewSized + TypeSize, O: BitOrder> TypeSize for BitArray<A, O> {
    fn extra_size(&self) -> usize {
        self.data.extra_size()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<T: BitStore, O: BitOrder> TypeSize for BitVec<T, O> {
    fn extra_size(&self) -> usize {
        div_ceil(self.capacity(), bitvec::mem::bits_of::<T>())
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}

// reimplementation of `usize::div_ceil` because MSRV < 1.73
#[inline]
pub const fn div_ceil(n: usize, rhs: usize) -> usize {
    let d = n / rhs;
    let r = n % rhs;
    if r > 0 {
        d + 1
    } else {
        d
    }
}
