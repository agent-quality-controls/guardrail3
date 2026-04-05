/// Typed Cargo.toml model definitions.
mod cargo_toml;

pub use cargo_toml::{
    BadgeTable, Badges, CargoToml, Dependency, DependencyDetail, FeatureList, FeatureMap,
    HintsConfig, InheritableStrings, InheritableValue, IntegerOrBool, IntegerOrString, LintDetail,
    LintTools, LintValue, LintsConfig, PackageBuildValue, PackageSection, PatchRegistryTable,
    PatchTable, ProfileConfig, StringOrBool, StringOrVec, TargetDependencyTables, TargetSection,
    TomlTrimPaths, TomlTrimPathsValue, ToolLints, VecStringOrBool, WorkspaceInheritance,
    WorkspacePackageSection, WorkspaceSection,
};
