/// Check result type returned by all guardrail3 check packages.
mod result;
/// Severity levels for check results.
mod severity;
/// Profile enum — application vs library workspace.
mod profile;

#[cfg(feature = "types")]
pub use profile::GrdzProfile;
#[cfg(feature = "types")]
pub use result::GrdzCheckResult;
#[cfg(feature = "types")]
pub use severity::GrdzSeverity;
