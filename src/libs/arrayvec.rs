use arrayvec::ArrayVec;

use crate::TypeSize;

impl<T: TypeSize, const CAP: usize> TypeSize for ArrayVec<T, CAP> {
    fn extra_size(&self) -> usize {
        self.iter().map(TypeSize::extra_size).sum()
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}
