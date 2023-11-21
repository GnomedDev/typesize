use crate::{map::generic_map_extra_size, TypeSize};

impl<K: TypeSize, V: TypeSize, S, const GROW_LIMIT: usize> TypeSize
    for halfbrown::SizedHashMap<K, V, S, GROW_LIMIT>
{
    fn extra_size(&self) -> usize {
        // TODO: Write tests for this.

        let size: usize = self
            .iter()
            .take(GROW_LIMIT)
            .map(|(k, v)| k.extra_size() + v.extra_size())
            .sum();

        size + generic_map_extra_size(
            self.iter().skip(GROW_LIMIT),
            self.capacity().saturating_sub(GROW_LIMIT),
            self.len().saturating_sub(GROW_LIMIT),
        )
    }

    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        Some(self.len())
    }
}
