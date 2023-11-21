//! # Typesize
//!
//! A library to fetch an accurate estimate of the total memory usage of a value.
//!
//! The goal of this library is to produce the most accurate estimate possible, however without being deeply
//! integrated into the entire ecosystem this cannot be possible. This leads to the real goal being to get
//! "close enough" for getting a sense of memory usage in your program. If one of the [`TypeSize`]
//! implementations built-in could be improved, a PR would be greatly appreciated.
//!
//! An example usage of this library would be to wrap all the types you want to measure recursively in
//! the [`derive::TypeSize`] derive macro, and for any types which perform their own heap allocation
//! to manually implement [`TypeSize`] while overriding the [`TypeSize::extra_size`] method.
//!
//! ## Features
//! ### Library Support
//! - `dashmap`: Implements [`TypeSize`] for [`DashMap`].
//! - `arrayvec`: Implements [`TypeSize`] for [`ArrayVec`] of any size.
//! - `simd_json`: Implements [`TypeSize`] for [`OwnedValue`] and [`StaticNode`], enables halfbrown.
//! - `halfbrown`: Implements [`TypeSize`] for [`SizedHashMap`], enables hashbrown.
//! - `parking_lot`: Implements [`TypeSize`] for [`parking_lot::Mutex`] and [`parking_lot::RwLock`].
//! - `serde_json`: Implements [`TypeSize`] for [`serde_json::Value`] and [`serde_json::Map`].
//! - `mini_moka`: Implements [`TypeSize`] for [`mini_moka::unsync::Cache`], and [`mini_moka::sync::Cache`] if `dashmap` is enabled.
//! - `hashbrown`: Implements [`TypeSize`] for [`hashbrown::HashMap`].
//! - `secrecy`: Implements [`TypeSize`] for [`Secret`].
//! - `chrono`: Implements [`TypeSize`] for [`chrono::DateTime`] of any [`chrono::TimeZone`].
//! - `time`: Implements [`TypeSize`] for [`time::OffsetDateTime`].
//! - `url`: Implements [`TypeSize`] for [`url::Url`].
//!
//! [`ArrayVec`]: arrayvec::ArrayVec
//! [`OwnedValue`]: simd_json::OwnedValue
//! [`StaticNode`]: simd_json::StaticNode
//! [`SizedHashMap`]: halfbrown::SizedHashMap
//! [`DashMap`]: dashmap::DashMap
//! [`Secret`]: secrecy::Secret
#![warn(clippy::pedantic, rust_2018_idioms)]
#![forbid(unsafe_code)]

mod enums;
mod libs;
mod map;
mod primitives;
pub mod ptr;
mod set;
mod tuple;
mod vec;

#[deprecated = "Use ptr::{Ref, RefMut}"]
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

    /// The total number of bytes that this type is using, both direct
    /// ([`std::mem::size_of`]) and indirect (behind allocations)
    ///
    /// There's no reason to ever override this method.
    fn get_size(&self) -> usize {
        std::mem::size_of::<Self>() + self.extra_size()
    }
}

/// Implements [`TypeSize`] for multiple types based on the return value of [`std::mem::size_of`].
#[macro_export]
macro_rules! sizeof_impl {
    ($($ty:ty),*) => {
        $(impl TypeSize for $ty {})*
    };
}
