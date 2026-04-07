/// Error surface for parser failures.
mod error;
/// Centralized filesystem boundary for parser file reads.
mod fs;
/// Parser module facade.
mod parser;

#[cfg(feature = "api")]
pub use cargo_toml_parser_types::{
    BadgeTable, Badges, CargoToml, Dependency, DependencyDetail, FeatureList, FeatureMap,
    HintsConfig, InheritableStrings, InheritableValue, IntegerOrBool, IntegerOrString, LintDetail,
    LintTools, LintValue, LintsConfig, PackageBuildValue, PackageSection, PatchRegistryTable,
    PatchTable, ProfileConfig, StringOrBool, StringOrVec, TargetDependencyTables, TargetSection,
    TomlTrimPaths, TomlTrimPathsValue, ToolLints, VecStringOrBool, WorkspaceInheritance,
    WorkspacePackageSection, WorkspaceSection,
};
#[cfg(feature = "api")]
pub use error::Error;
#[cfg(feature = "api")]
pub use parser::{from_path, parse};
#[cfg(feature = "api")]
pub use toml::Value;
