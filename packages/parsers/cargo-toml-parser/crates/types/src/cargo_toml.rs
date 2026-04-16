use std::collections::BTreeMap;

use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
use toml::Value;

pub type FeatureList = Vec<String>;
pub type FeatureMap = BTreeMap<String, FeatureList>;
pub type BadgeTable = BTreeMap<String, Value>;
pub type Badges = BTreeMap<String, BadgeTable>;
pub type PatchTable = BTreeMap<String, Dependency>;
pub type PatchRegistryTable = BTreeMap<String, PatchTable>;
pub type ToolLints = BTreeMap<String, LintValue>;
pub type LintTools = BTreeMap<String, ToolLints>;
pub type InheritableStrings = InheritableValue<Vec<String>>;

/// Typed representation of a `Cargo.toml` file.
///
/// The model stays close to the manifest's file shape. Known sections and
/// fields are typed, inheritance syntax is represented directly, and unknown
/// keys are preserved in `extra`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct CargoToml {
    #[serde(default)]
    pub cargo_features: Vec<String>,
    pub package: Option<PackageSection>,
    pub project: Option<PackageSection>,
    #[serde(default)]
    pub badges: Badges,
    #[serde(default)]
    pub features: FeatureMap,
    pub lib: Option<TargetSection>,
    #[serde(default)]
    pub bin: Vec<TargetSection>,
    #[serde(default)]
    pub example: Vec<TargetSection>,
    #[serde(default)]
    pub test: Vec<TargetSection>,
    #[serde(default)]
    pub bench: Vec<TargetSection>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "dev-dependencies", alias = "dev_dependencies", default)]
    pub dev_dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "build-dependencies", alias = "build_dependencies", default)]
    pub build_dependencies: BTreeMap<String, Dependency>,
    #[serde(default)]
    pub target: BTreeMap<String, TargetDependencyTables>,
    pub lints: Option<LintsConfig>,
    pub hints: Option<HintsConfig>,
    pub workspace: Option<WorkspaceSection>,
    #[serde(default)]
    pub profile: BTreeMap<String, ProfileConfig>,
    #[serde(default)]
    pub patch: PatchRegistryTable,
    #[serde(default)]
    pub replace: BTreeMap<String, Dependency>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InheritableValue<T> {
    Value(T),
    Inherit(WorkspaceInheritance),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct WorkspaceInheritance {
    pub workspace: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VecStringOrBool {
    VecString(Vec<String>),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrVec {
    String(String),
    Vec(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegerOrString {
    Integer(i64),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IntegerOrBool {
    Integer(i64),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TomlTrimPaths {
    Values(Vec<TomlTrimPathsValue>),
    All,
}

impl<'de> Deserialize<'de> for TomlTrimPaths {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        /// Visitor for Cargo's `trim-paths` mixed TOML forms.
        struct V;

        impl<'de> serde::de::Visitor<'de> for V {
            type Value = TomlTrimPaths;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "a boolean, \"none\", \"diagnostics\", \"macro\", \"object\", \"all\", or an array with these options"
                )
            }

            fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(if value {
                    TomlTrimPaths::All
                } else {
                    TomlTrimPaths::Values(Vec::new())
                })
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "none" => Ok(TomlTrimPaths::Values(Vec::new())),
                    "all" => Ok(TomlTrimPaths::All),
                    "diagnostics" => {
                        Ok(TomlTrimPaths::Values(vec![TomlTrimPathsValue::Diagnostics]))
                    }
                    "macro" => Ok(TomlTrimPaths::Values(vec![TomlTrimPathsValue::Macro])),
                    "object" => Ok(TomlTrimPaths::Values(vec![TomlTrimPathsValue::Object])),
                    other => Err(E::invalid_value(de::Unexpected::Str(other), &self)),
                }
            }

            fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let values = Vec::<TomlTrimPathsValue>::deserialize(
                    de::value::SeqAccessDeserializer::new(seq),
                )?;
                Ok(TomlTrimPaths::Values(values))
            }
        }

        deserializer.deserialize_any(V)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TomlTrimPathsValue {
    Diagnostics,
    Macro,
    Object,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct PackageSection {
    pub edition: Option<InheritableValue<String>>,
    pub rust_version: Option<InheritableValue<String>>,
    pub name: Option<String>,
    pub version: Option<InheritableValue<String>>,
    pub authors: Option<InheritableStrings>,
    pub build: Option<PackageBuildValue>,
    pub metabuild: Option<StringOrVec>,
    pub default_target: Option<String>,
    pub forced_target: Option<String>,
    pub links: Option<String>,
    pub exclude: Option<InheritableStrings>,
    pub include: Option<InheritableStrings>,
    pub publish: Option<InheritableValue<VecStringOrBool>>,
    pub workspace: Option<String>,
    pub im_a_teapot: Option<bool>,
    pub autolib: Option<bool>,
    pub autobins: Option<bool>,
    pub autoexamples: Option<bool>,
    pub autotests: Option<bool>,
    pub autobenches: Option<bool>,
    pub default_run: Option<String>,
    pub description: Option<InheritableValue<String>>,
    pub homepage: Option<InheritableValue<String>>,
    pub documentation: Option<InheritableValue<String>>,
    pub readme: Option<InheritableValue<StringOrBool>>,
    pub keywords: Option<InheritableStrings>,
    pub categories: Option<InheritableStrings>,
    pub license: Option<InheritableValue<String>>,
    pub license_file: Option<InheritableValue<String>>,
    pub repository: Option<InheritableValue<String>>,
    pub resolver: Option<String>,
    pub metadata: Option<Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageBuildValue {
    Auto(bool),
    SingleScript(String),
    MultipleScript(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct WorkspaceSection {
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(rename = "default-members", default)]
    pub default_members: Vec<String>,
    pub resolver: Option<String>,
    pub metadata: Option<Value>,
    pub package: Option<WorkspacePackageSection>,
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
    pub lints: Option<LintsConfig>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct WorkspacePackageSection {
    pub version: Option<String>,
    #[serde(default)]
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<StringOrBool>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub repository: Option<String>,
    pub publish: Option<VecStringOrBool>,
    pub edition: Option<String>,
    #[serde(default)]
    pub badges: Badges,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub include: Vec<String>,
    pub rust_version: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Detailed(Box<DependencyDetail>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct DependencyDetail {
    pub version: Option<String>,
    pub registry: Option<String>,
    pub registry_index: Option<String>,
    pub path: Option<String>,
    pub base: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    #[serde(default)]
    pub features: Vec<String>,
    pub optional: Option<bool>,
    #[serde(rename = "default-features", alias = "default_features")]
    pub default_features: Option<bool>,
    pub package: Option<String>,
    pub workspace: Option<bool>,
    pub public: Option<bool>,
    pub artifact: Option<StringOrVec>,
    pub lib: Option<bool>,
    pub target: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LintValue {
    Level(String),
    Detailed(LintDetail),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct LintDetail {
    pub level: String,
    pub priority: Option<i64>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct LintsConfig {
    pub workspace: Option<bool>,
    #[serde(flatten)]
    pub tools: LintTools,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct HintsConfig {
    pub mostly_unused: Option<Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct TargetDependencyTables {
    #[serde(default)]
    pub dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "dev-dependencies", alias = "dev_dependencies", default)]
    pub dev_dependencies: BTreeMap<String, Dependency>,
    #[serde(rename = "build-dependencies", alias = "build_dependencies", default)]
    pub build_dependencies: BTreeMap<String, Dependency>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct TargetSection {
    pub name: Option<String>,
    #[serde(rename = "crate-type", alias = "crate_type", default)]
    pub crate_type: Vec<String>,
    pub path: Option<String>,
    pub filename: Option<String>,
    pub test: Option<bool>,
    pub doctest: Option<bool>,
    pub bench: Option<bool>,
    pub doc: Option<bool>,
    pub doc_scrape_examples: Option<bool>,
    #[serde(rename = "proc-macro", alias = "proc_macro")]
    pub proc_macro: Option<bool>,
    pub harness: Option<bool>,
    #[serde(default)]
    pub required_features: Vec<String>,
    pub edition: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[non_exhaustive]
pub struct ProfileConfig {
    pub opt_level: Option<IntegerOrString>,
    pub lto: Option<StringOrBool>,
    pub codegen_backend: Option<String>,
    pub codegen_units: Option<u32>,
    pub debug: Option<IntegerOrBool>,
    pub split_debuginfo: Option<String>,
    pub debug_assertions: Option<bool>,
    pub rpath: Option<bool>,
    pub panic: Option<String>,
    pub overflow_checks: Option<bool>,
    pub incremental: Option<bool>,
    pub dir_name: Option<String>,
    pub inherits: Option<String>,
    pub strip: Option<StringOrBool>,
    #[serde(default)]
    pub rustflags: Vec<String>,
    #[serde(default)]
    pub package: BTreeMap<String, Self>,
    pub build_override: Option<Box<Self>>,
    pub trim_paths: Option<TomlTrimPaths>,
    pub hint_mostly_unused: Option<bool>,
    pub frame_pointers: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
