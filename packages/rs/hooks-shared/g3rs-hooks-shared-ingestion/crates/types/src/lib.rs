mod error;

pub use error::G3RsHooksSharedIngestionError;
pub use g3rs_hooks_shared_source_checks_types::{
    G3RsHookScriptKind, G3RsHooksSharedSourceChecksInput,
};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsHooksSharedConfigChecksInput;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsHooksSharedFileTreeChecksInput;
