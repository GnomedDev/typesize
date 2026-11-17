//! [TypeSize] support for [jiff].
//!
//! This covers all [jiff] types which represent dates and/or times, such as [jiff:civil::Date] or
//! [jiff::ZonedDateTime]. It does not cover utility types used for formatting/rounding/etc, such
//! as [jiff::SignedDurationRound] or [jiff::ZonedDateTimeArithmetic].
use crate::TypeSize;

// SignedDuration is a struct containing two ints
impl TypeSize for jiff::SignedDuration {}

// Timestamp is a struct containing a SignedDuration
impl TypeSize for jiff::Timestamp {}

// Date is a struct containing three ints
impl TypeSize for jiff::civil::Date {}

// Time is a struct containing four ints
impl TypeSize for jiff::civil::Time {}

// DateTime is a struct containing a Date and Time
impl TypeSize for jiff::civil::DateTime {}

// Here's a trickier bit. A Span is a struct containing several ints, a Sign, and a UnitSet.
// The Sign is an enum with an i8 repr. The UnitSet is a wrapper around a u16 bitfield.
// But none of those should require extra_size.
impl TypeSize for jiff::Span {}

// Offset is a struct containing an int
impl TypeSize for jiff::tz::Offset {}

// TimeZone internally contains a pointer which may or may not point to owned data.
// https://github.com/BurntSushi/jiff/blob/1.0.0/crates/jiff/src/tz/timezone.rs#L2352-L2374
// But it supports a memory_usage method which returns what get_extra_size should return.
impl TypeSize for jiff::tz::TimeZone {
    fn extra_size(&self) -> usize {
        self.memory_usage()
    }
}

// Zoned contains a Timestamp, DateTime, Offset, and TimeZone. Therefore it also provides
// a memory_usage method.
impl TypeSize for jiff::ZonedDateTime {
    fn extra_size(&self) -> usize {
        self.memory_usage()
    }
}
