/// Shared error types used across the app boundary.
mod errors;
/// Shared validation report payloads.
mod report;
/// Shared request payloads for app commands.
mod request;
/// Shared family identifiers exposed by the app.
mod supported_family;
/// Shared runtime traits for crawling, running, and rendering.
mod traits;

#[cfg(feature = "api")]
pub use errors::{FamilyRunError, WorkspaceCrawlError};
#[cfg(feature = "api")]
pub use report::{FamilyResults, FamilyRun, ValidateReport};
#[cfg(feature = "api")]
pub use request::{
    AppCommand, InitCommand, InitRepoRequest, InitWorkspaceRequest, ValidateCommand,
    ValidateRepoRequest, ValidateRequest,
};
#[cfg(feature = "api")]
pub use supported_family::{SUPPORTED_FAMILIES, SupportedFamily};
#[cfg(feature = "api")]
pub use traits::{FamilyRunner, ReportRenderer, WorkspaceCrawler};
