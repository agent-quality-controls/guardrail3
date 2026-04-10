mod error;

pub use error::G3RsHooksRsIngestionError;
pub use g3rs_hooks_rs_source_checks_types::G3RsHooksRsSourceChecksInput;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsHooksRsConfigChecksInput;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsHooksRsFileTreeChecksInput;
