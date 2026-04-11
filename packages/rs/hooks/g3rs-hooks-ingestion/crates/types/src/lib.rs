mod error;

pub use error::G3RsHooksIngestionError;
pub use g3rs_hooks_source_checks_types::{G3RsHookScriptKind, G3RsHooksSourceChecksInput};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsHooksConfigChecksInput;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsHooksFileTreeChecksInput;
