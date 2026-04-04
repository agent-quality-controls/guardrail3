/// Typed Cargo.toml model definitions.
mod cargo_toml;

pub use cargo_toml::{
    CargoToml, Dependency, DependencyDetail, LintDetail, LintValue, LintsConfig, NamedTarget,
    PackageSection, TargetDependencyTables, WorkspacePackageSection, WorkspaceSection,
};
