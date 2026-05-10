#![allow(
    clippy::module_name_repetitions,
    reason = "parser document model types intentionally include the parser domain (AstroConfig) and document role"
)]

#[cfg(feature = "api")]
pub mod document;

use serde as _;
use serde_json as _;
