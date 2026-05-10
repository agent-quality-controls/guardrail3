//! Reason policy validation primitives shared across guardrail3 families.

/// Reason validation issue variants surfaced to callers.
mod issue;
/// Reason policy configuration values and defaults.
mod policy;
/// Reason text validation against the active policy.
mod validation;

#[cfg(feature = "api")]
pub use issue::ReasonIssue;
#[cfg(feature = "api")]
pub use policy::DEFAULT_MIN_REASON_CHARS;
#[cfg(feature = "api")]
pub use policy::DEFAULT_MIN_REASON_WORDS;
#[cfg(feature = "api")]
pub use policy::ReasonPolicy;
#[cfg(feature = "api")]
pub use validation::reason_text_is_useful;
#[cfg(feature = "api")]
pub use validation::validate_reason_text;
#[cfg(feature = "api")]
pub use validation::validate_reason_text_with_policy;
