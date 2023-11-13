use crate::TypeSize;

impl<Tz: chrono::TimeZone> TypeSize for chrono::DateTime<Tz> {}
