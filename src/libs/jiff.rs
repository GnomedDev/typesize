//! [TypeSize] support for [jiff].
//!
//! This covers all [jiff] types which represent dates and/or times. It does not cover utility
//! types used for formatting/rounding/etc.
use crate::TypeSize;

macro_rules! implement_for {
    ($krate:ident) => {
        // SignedDuration is a struct containing two ints
        impl TypeSize for $krate::SignedDuration {}

        // Timestamp is a struct containing a SignedDuration
        impl TypeSize for $krate::Timestamp {}

        // Date is a struct containing three ints
        impl TypeSize for $krate::civil::Date {}

        // Time is a struct containing four ints
        impl TypeSize for $krate::civil::Time {}

        // DateTime is a struct containing a Date and Time
        impl TypeSize for $krate::civil::DateTime {}

        // Here's a trickier bit. A Span is a struct containing several ints, a Sign, and a UnitSet.
        // The Sign is an enum with an i8 repr. The UnitSet is a wrapper around a u16 bitfield.
        // But none of those should require extra_size.
        impl TypeSize for $krate::Span {}

        // Offset is a struct containing an int
        impl TypeSize for $krate::tz::Offset {}

        // TimeZone internally contains a pointer which may or may not point to owned data.
        // https://github.com/BurntSushi/jiff/blob/1.0.0/crates/jiff/src/tz/timezone.rs#L2352-L2374
        // But it supports a memory_usage method which returns what get_extra_size should return.
        impl TypeSize for $krate::tz::TimeZone {
            fn extra_size(&self) -> usize {
                self.memory_usage()
            }
        }
    };
}

// Zoned/ZonedDateTime contains a Timestamp, DateTime, Offset, and TimeZone. Therefore, it also
// provides a memory_usage method.
#[cfg(feature = "jiff")]
impl TypeSize for jiff_0::Zoned {
    fn extra_size(&self) -> usize {
        self.memory_usage()
    }
}
#[cfg(feature = "jiff_1")]
impl TypeSize for jiff_1::ZonedDateTime {
    fn extra_size(&self) -> usize {
        self.memory_usage()
    }
}

#[cfg(feature = "jiff")]
implement_for!(jiff_0);
#[cfg(feature = "jiff_1")]
implement_for!(jiff_1);
