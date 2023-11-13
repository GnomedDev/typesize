use crate::TypeSize;

impl<T: TypeSize> TypeSize for Option<T> {
    fn extra_size(&self) -> usize {
        self.as_ref().map(T::extra_size).unwrap_or_default()
    }
}
