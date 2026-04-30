#[cfg(feature = "api")]
use g3ts_style_config_checks_runtime as _;

#[cfg(feature = "api")]
mod run;

#[cfg(feature = "api")]
pub use run::{assert_runtime_check_exact_ids, assert_runtime_check_id_severity};
