/// error module.
mod error;
/// fs module.
mod fs;
/// ingest module.
mod ingest;
/// parse module.
mod parse;
/// run module.
mod run;
/// select module.
mod select;

#[cfg(feature = "ingest")]
pub use error::G3RsClippyIngestionError;
#[cfg(feature = "ingest")]
pub use run::{IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks};
