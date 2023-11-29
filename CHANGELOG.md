# `typesize` ChangeLog

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
- Tuples containing values that implement `Typesize` up to size 4 have had implementations added.

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
