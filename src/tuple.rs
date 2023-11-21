use crate::TypeSize;

impl<T1: TypeSize> TypeSize for (T1,) {
    fn extra_size(&self) -> usize {
        self.0.extra_size()
    }
}
impl<T1: TypeSize, T2: TypeSize> TypeSize for (T1, T2) {
    fn extra_size(&self) -> usize {
        self.0.extra_size() + self.1.extra_size()
    }
}
impl<T1: TypeSize, T2: TypeSize, T3: TypeSize> TypeSize for (T1, T2, T3) {
    fn extra_size(&self) -> usize {
        self.0.extra_size() + self.1.extra_size() + self.2.extra_size()
    }
}
impl<T1: TypeSize, T2: TypeSize, T3: TypeSize, T4: TypeSize> TypeSize for (T1, T2, T3, T4) {
    fn extra_size(&self) -> usize {
        self.0.extra_size() + self.1.extra_size() + self.2.extra_size() + self.3.extra_size()
    }
}
