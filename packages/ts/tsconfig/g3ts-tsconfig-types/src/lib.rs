#[cfg(feature = "api")]
mod flags;

#[cfg(feature = "api")]
mod types;

#[cfg(feature = "api")]
pub use flags::inline_strict_flags;
#[cfg(feature = "api")]
pub use types::G3TsTsconfigBoolState;
#[cfg(feature = "api")]
pub use types::G3TsTsconfigChecksInput;
#[cfg(feature = "api")]
pub use types::G3TsTsconfigExtendsState;
#[cfg(feature = "api")]
pub use types::G3TsTsconfigInlineStrictFlags;
#[cfg(feature = "api")]
pub use types::G3TsTsconfigState;
