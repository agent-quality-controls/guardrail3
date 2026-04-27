mod error;
mod fs;
mod ingest;
mod parse;
mod run;
mod select;

#[cfg(feature = "ingest")]
pub use error::G3RsClippyIngestionError;
#[cfg(feature = "ingest")]
pub use run::{IngestionError, ingest_for_config_checks, ingest_for_file_tree_checks};
