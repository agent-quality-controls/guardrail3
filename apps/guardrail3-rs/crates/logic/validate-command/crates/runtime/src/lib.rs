/// Command execution flow for validate requests.
mod execute;
/// Final CLI outcome payload.
mod outcome;
/// Family-selection helpers shared by command execution.
mod selection;

#[cfg(feature = "api")]
pub use execute::execute;
#[cfg(feature = "api")]
pub use outcome::ExecutionOutcome;
#[cfg(feature = "api")]
pub use selection::{family_cli_name, selected_families};
