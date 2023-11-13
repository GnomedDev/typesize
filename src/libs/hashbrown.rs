use hashbrown::HashMap;

use crate::{map::generic_map_extra_size, TypeSize};

impl<K: TypeSize, V: TypeSize, S> TypeSize for HashMap<K, V, S> {
    fn extra_size(&self) -> usize {
        generic_map_extra_size(self.iter(), self.capacity(), self.len())
    }
}
