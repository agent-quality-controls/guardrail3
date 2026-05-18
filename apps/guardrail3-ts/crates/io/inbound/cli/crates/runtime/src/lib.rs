#![expect(
    clippy::multiple_crate_versions,
    reason = "transitive dep `siphasher` resolves to 0.3.11 (via swc_common in g3ts-arch-ingestion's SWC-based parser) and 1.0.2 (via other dependents); both versions are pinned by upstream crates this app does not own"
)]

/// CLI parsing types and helpers.
mod cli;
/// Validate repo / validate workspace command execution.
mod execute;
/// Centralized filesystem boundary used by the CLI runtime.
mod fs;
/// Marker-pair completeness walker for validate repo.
mod marker_pairs;
/// Centralized process-spawn boundary used by the CLI runtime.
mod process;
/// CLI dispatch entry points and shared CliOutput type.
mod run;
/// Required-tool presence checks for validate repo.
mod tool_presence;
/// Topology family dispatch and CLI family runner.
mod topology;

#[cfg(feature = "api")]
pub use cli::{Cli, Command, FamilyArg, ValidateCommand, parse_command_from};
#[cfg(feature = "api")]
pub use guardrail3_ts_packages::PackageRuntime;
#[cfg(feature = "api")]
pub use guardrail3_ts_report::PlainTextReportRenderer;
#[cfg(feature = "api")]
pub use run::{CliOutput, run_command, run_command_with_defaults};
#[cfg(feature = "api")]
pub use topology::CliFamilyRunner;
