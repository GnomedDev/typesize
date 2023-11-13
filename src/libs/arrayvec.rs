use arrayvec::ArrayVec;

use crate::TypeSize;

impl<T: TypeSize, const CAP: usize> TypeSize for ArrayVec<T, CAP> {
    fn extra_size(&self) -> usize {
        self.iter().map(TypeSize::extra_size).sum()
    }
}
