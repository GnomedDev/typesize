# `typesize` ChangeLog

## 0.1.1

### Added

#### `ptr::{SizableRc, SizableArc}`

Wrapper types around `Rc<T>` or `Arc<T>` which allow the user to provide if the inner `T`
should be counted for size calculations or not via a type parameter.

#### `ptr::{Ptr, PtrMut}`

Wrapper types around `*const T` and `*mut T` to just count the pointer size.

## 0.1.0

Initial version
