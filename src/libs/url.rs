use crate::TypeSize;

impl TypeSize for url::Url {
    fn extra_size(&self) -> usize {
        let serialization: String = self.clone().into();
        serialization.extra_size()
    }
}
