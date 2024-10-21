#![cfg(all(target_family = "wasm", target_os = "unknown"))]

use web_time::Instant;

use crate::TypeSize;

impl TypeSize for Instant {}
