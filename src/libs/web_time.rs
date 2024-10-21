#![cfg(all(target_family = "wasm", target_os = "unknown"))]

use web_time::{Instant, SystemTime};

use crate::TypeSize;

impl TypeSize for Instant {}

impl TypeSize for SystemTime {}
