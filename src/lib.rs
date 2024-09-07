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
//! ## MSRV
//! The Minimum Supported Rust Version is of this crate is 1.65, and it is considered breaking to raise this.
//!
//! This is without any library support features, as those libraries may require a higher MSRV.
//!
//! ## Features
//!
//! - `std`: Implements [`TypeSize`] for [`HashMap`] and [`HashSet`], default enabled.
//! - `details`: Adds [`TypeSize::get_size_details`] and [`TypeSize::get_collection_item_count`] to get field by field breakdowns of struct types.
//!
//! ### Library Support
//! - `dashmap`: Implements [`TypeSize`] for [`DashMap`] (**Only `5.x`, use the `typesize` feature of dashmap for `6.1`+**).
//! - `arrayvec`: Implements [`TypeSize`] for [`ArrayVec`] and [`ArrayString`] of any size.
//! - `simd_json`: Implements [`TypeSize`] for [`OwnedValue`] and [`StaticNode`], enables halfbrown.
//! - `halfbrown`: Implements [`TypeSize`] for [`SizedHashMap`], enables hashbrown.
//! - `extract_map_01`: Implements [`TypeSize`] for [`extract_map::ExtractMap`].
//! - `parking_lot`: Implements [`TypeSize`] for [`parking_lot::Mutex`] and [`parking_lot::RwLock`].
//! - `serde_json`: Implements [`TypeSize`] for [`serde_json::Value`] and [`serde_json::Map`].
//! - `mini_moka`: Implements [`TypeSize`] for [`mini_moka::unsync::Cache`], and [`mini_moka::sync::Cache`] if `dashmap` is enabled.
//! - `hashbrown`: Implements [`TypeSize`] for [`hashbrown::HashMap`].
//! - `secrecy`: Implements [`TypeSize`] for [`Secret`].
//! - `chrono`: Implements [`TypeSize`] for [`chrono::DateTime`] of any [`chrono::TimeZone`].
//! - `nonmax`: Implements [`TypeSize`] for all [`nonmax`] types.
//! - `time`: Implements [`TypeSize`] for [`time::OffsetDateTime`].
//! - `url`: Implements [`TypeSize`] for [`url::Url`].
//!
//! [`HashMap`]: std::collections::HashMap
//! [`HashSet`]: std::collections::HashSet
//! [`ArrayVec`]: arrayvec::ArrayVec
//! [`ArrayString`]: arrayvec::ArrayString
//! [`OwnedValue`]: simd_json::OwnedValue
//! [`StaticNode`]: simd_json::StaticNode
//! [`SizedHashMap`]: halfbrown::SizedHashMap
//! [`DashMap`]: dashmap::DashMap
//! [`Secret`]: secrecy::Secret
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::pedantic, rust_2018_idioms)]
#![forbid(unsafe_code)]

extern crate alloc;

mod r#box;
mod cell;
mod enums;
mod hash;
mod libs;
#[cfg(any(feature = "std", feature = "mini_moka", feature = "hashbrown"))]
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
    /// The number of bytes more than the [`core::mem::size_of`] that this value is using.
    #[must_use]
    fn extra_size(&self) -> usize {
        0
    }

    /// The total number of bytes that this type is using, both direct
    /// ([`core::mem::size_of`]) and indirect (behind allocations)
    ///
    /// There's no reason to ever override this method.
    #[must_use]
    fn get_size(&self) -> usize {
        core::mem::size_of::<Self>() + self.extra_size()
    }

    /// Returns information about the number of items this type is holding, if it is a collection.
    #[must_use]
    #[cfg(feature = "details")]
    fn get_collection_item_count(&self) -> Option<usize> {
        None
    }

    /// Returns detailed information about the current value's field sizes.
    ///
    /// This should generally be implemented by [`derive::TypeSize`]
    #[must_use]
    #[cfg(feature = "details")]
    fn get_size_details(&self) -> alloc::vec::Vec<Field> {
        alloc::vec::Vec::new()
    }
}

/// A description of a struct or enum field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg(feature = "details")]
pub struct Field {
    /// The name of the field being described.
    pub name: &'static str,
    /// The total size of the field.
    pub size: usize,
    /// How many items this collection is holding, if it is one.
    pub collection_items: Option<usize>,
}

/// Implements [`TypeSize`] for multiple types based on the return value of [`core::mem::size_of`].
#[macro_export]
macro_rules! sizeof_impl {
    ($($ty:ty),*) => {
        $(impl TypeSize for $ty {})*
    };
}

/// Passes through the given tokens if the `details` feature of `typesize` is enabled.
///
/// This is mainly useful for libaries making their own [`TypeSize`] to be compatible with `details` on or off.
#[macro_export]
#[cfg(feature = "details")]
macro_rules! if_typesize_details {
    ($($tt:tt)*) => {
        $($tt)*
    }
}

/// Passes through the given tokens if the `details` feature of `typesize` is enabled.
///
/// This is mainly useful for libaries making their own [`TypeSize`] to be compatible with `details` on or off.
#[macro_export]
#[cfg(not(feature = "details"))]
macro_rules! if_typesize_details {
    ($($tt:tt)*) => {};
}
