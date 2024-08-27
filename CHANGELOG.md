# `typesize` ChangeLog

## 0.1.8

### Added

- `typesize::if_typesize_details!` allows libraries to implement the `details` methods optionally.
- `TypeSize` is now implemented for `core::sync::atomic::Atomic*`
- `TypeSize` is now implemented for `Cell`, and `RefCell`

## 0.1.7

- MSRV is now documented, at Rust 1.65, however this may be changed by enabling library support features.

## 0.1.6

### Added

#### New library integrations

- `TypeSize` is now implemented for `extract_map::ExtractMap`

#### Other

- Basic no-std support has been added, but this is experimental.

## 0.1.5

### Added

#### New library integrations

- `nonzero` have had size_of `typesize::TypeSize` implementations added for all integer types.

## 0.1.4

### Added

#### New implementations

`typesize::TypeSize` is now implemented for `Box<str>` and `Box<[T]>`, and the missing
`arrayvec::ArrayString` implementation has been added with the`arrayvec` feature.

## 0.1.3

### Added

#### Support for repr(packed) structs

`typesize::derive::TypeSize` now supports `struct`s annotated with `repr(packed)`.

## 0.1.2

### Added

#### New feature: `details`

Added `TypeSize::{get_size_details, get_collection_item_count}` to allow breaking structs down into each field for size optimisation. `get_size_details` is automatically implemented for structs with `derive::TypeSize` and `get_collection_item_count` has been implemented on all built-in collection implementations.

#### New library integrations

- `Mutex` and `RwLock` from `parking_lot` have had passthrough implementations added.
- `sync::Cache` and `unsync::Cache` from `mini-moka` have had basic implementations added.

#### New implementations

- `HashSet`s from `std` and `hashbrown` have had basic implementations added.
- Tuples containing values that implement `TypeSize` up to size 4 have had implementations added.

#### Other

- A derived Debug implementation for `Sizable{Arc,Rc}` has been added.

## 0.1.1

### Added

#### `ptr::{SizableRc, SizableArc}`

Wrapper types around `Rc<T>` or `Arc<T>` which allow the user to provide if the inner `T`
should be counted for size calculations or not via a type parameter.

#### `ptr::{Ptr, PtrMut}`

Wrapper types around `*const T` and `*mut T` to just count the pointer size.

## 0.1.0

Initial version
