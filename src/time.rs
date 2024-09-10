use crate::{sizeof_impl, TypeSize};

sizeof_impl!(core::time::Duration);

#[cfg(feature = "std")]
sizeof_impl!(std::time::Instant, std::time::SystemTime);
