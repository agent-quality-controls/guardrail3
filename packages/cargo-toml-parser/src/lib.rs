#[cfg(feature = "api")]
pub use cargo_toml_parser_runtime::{
    BadgeTable, Badges, CargoToml, Dependency, DependencyDetail, Error, FeatureList, FeatureMap,
    HintsConfig, InheritableStrings, InheritableValue, IntegerOrBool, IntegerOrString, LintDetail,
    LintTools, LintValue, LintsConfig, PackageBuildValue, PackageSection, PatchRegistryTable,
    PatchTable, ProfileConfig, StringOrBool, StringOrVec, TargetDependencyTables, TargetSection,
    TomlTrimPaths, TomlTrimPathsValue, ToolLints, Value, VecStringOrBool, WorkspaceInheritance,
    WorkspacePackageSection, WorkspaceSection, from_path, parse,
};
