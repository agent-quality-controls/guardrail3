/// Profile enum — application vs library workspace.
mod profile;
/// Check result type returned by all guardrail3 check packages.
mod result;
/// Severity levels for check results.
mod severity;

#[cfg(feature = "types")]
pub use profile::G3Profile;
#[cfg(feature = "types")]
pub use result::G3CheckResult;
#[cfg(feature = "types")]
pub use severity::G3Severity;
