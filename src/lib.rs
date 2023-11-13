//! # Typesize
//!
//! A library to fetch an accurate estimate of the total memory usage of a value.
//!
//! ## Features
//! ### Library Support
//! - `arrayvec`: Implements TypeSize for ArrayVec of any size.
//! - `simd_json`: Implements TypeSize for OwnedValue and StaticNode, enables halfbrown.
//! - `halfbrown`: Implements TypeSize for SizedHashMap, enables hashbrown.
//! - `dashmap`: Implements TypeSize for DashMap where K and V are TypeSize.
//! - `serde_json`: Implements TypeSize for Value and Map.
//! - `hashbrown`: Implements TypeSize for HashMap.
//! - `secrecy`: Implements TypeSize for SecretString.
//! - `chrono`: Implements TypeSize for DateTime of any TimeZone.
//! - `time`: Implements TypeSize for OffsetDateTime.
//! - `url`: Implements TypeSize for Url.

mod enums;
mod libs;
mod map;
mod primitives;
mod ptr;
mod vec;

pub use ptr::{Ref, RefMut};

pub mod derive {
    pub use typesize_derive::TypeSize;
}

/// A trait to fetch an accurate estimate of the total memory usage of a value.
///
/// Unless you are writing a data structure, you should derive this trait using [`derive::TypeSize`].
///
/// Note: Implementations cannot be relied on for any form of `unsafe` bound,
/// as this is entirely safe to implement incorrectly.
pub trait TypeSize: Sized {
    /// The number of bytes more than the [`std::mem::size_of`] that this value is using.
    fn extra_size(&self) -> usize {
        0
    }

    /// The total number of bytes that this type is using,
    /// both direct (size_of) and indirect (behind allocations)
    ///
    /// There's no reason to ever override this method.
    fn get_size(&self) -> usize {
        std::mem::size_of::<Self>() + self.extra_size()
    }
}

#[macro_export]
macro_rules! sizeof_impl {
    ($($ty:ty),*) => {
        $(impl TypeSize for $ty {})*
    };
}
