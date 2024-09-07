use crate::TypeSize;

// These are in std::hash, but older versions require accessing via these paths.
#[cfg(feature = "std")]
crate::sizeof_impl!(
    std::collections::hash_map::RandomState,
    std::collections::hash_map::DefaultHasher
);

impl<H> TypeSize for core::hash::BuildHasherDefault<H> {}
