#[cfg(feature = "api")]
pub use g3rs_clippy_config_ingestion_runtime::{ingest_ast, ingest_config, ingest_file_tree};
#[cfg(feature = "api")]
pub use g3rs_clippy_config_ingestion_types::G3RsClippyConfigIngestionError;
