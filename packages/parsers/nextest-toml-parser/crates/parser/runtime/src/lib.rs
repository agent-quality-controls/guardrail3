/// Error surface for parser failures.
mod error;
/// Centralized filesystem boundary for parser file reads.
mod fs;
/// Parser module facade.
mod parser;

#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use nextest_toml_parser_types::{
    ArchiveConfig, ArchiveDepth, ArchiveInclude, ArchiveOnMissing, ExperimentalFeature,
    ExponentialRetryPolicyDetail, FailFastConfig, FailFastCount, FailFastDetail,
    FinalStatusLevel, FlakyResult, JunitConfig, JunitFlakyFailStatus, NextestBenchConfig,
    NextestProfile, NextestToml, NextestVersionConfig, NextestVersionDetail,
    OverrideBenchConfig, OverrideJunitConfig, PlatformConfig, PlatformDetail, ProfileOverride,
    ProfileScriptConfig, RelativeTo, RetryPolicy, RetryPolicyDetail, ScriptCommand,
    ScriptCommandDetail, ScriptJunitConfig, ScriptReference, ScriptsConfig, SetupScriptConfig,
    StatusLevel, StoreConfig, TargetRunnerMode, TerminateMode, TestGroupConfig,
    TestGroupMaxThreads, TestOutputDisplay, TestThreads, ThreadsRequired, TimeoutConfig,
    TimeoutDetail, TimeoutResult, WrapperScriptConfig,
};
#[cfg(feature = "api")]
pub use parser::{from_path, parse};
#[cfg(feature = "api")]
pub use toml::Value;
