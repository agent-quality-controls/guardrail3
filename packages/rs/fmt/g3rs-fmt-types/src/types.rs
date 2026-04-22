#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsFmtRustfmtFacts {
    pub edition: Option<String>,
    pub style_edition: Option<String>,
    pub max_width: Option<i64>,
    pub tab_spaces: Option<i64>,
    pub use_field_init_shorthand: Option<bool>,
    pub use_try_shorthand: Option<bool>,
    pub reorder_imports: Option<bool>,
    pub reorder_modules: Option<bool>,
    pub explicit_keys: Vec<String>,
    pub nightly_keys: Vec<String>,
    pub ignore: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsFmtCargoFacts {
    pub edition: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct G3RsFmtToolchainFacts {
    pub channel: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsFmtRustfmtConfigState {
    Parsed(G3RsFmtRustfmtFacts),
    Unreadable,
    ParseError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsFmtCargoState {
    Parsed(G3RsFmtCargoFacts),
    Missing,
    Unreadable,
    ParseError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsFmtToolchainState {
    Parsed(G3RsFmtToolchainFacts),
    Missing,
    Unreadable,
    ParseError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsFmtWaiver {
    pub rule: String,
    pub file: String,
    pub selector: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3RsFmtRustPolicyState {
    Missing,
    Unreadable {
        rel_path: String,
        reason: String,
    },
    ParseError {
        rel_path: String,
        reason: String,
    },
    Parsed {
        rel_path: String,
        waivers: Vec<G3RsFmtWaiver>,
    },
}

#[derive(Debug, Clone)]
pub struct G3RsFmtConfigChecksInput {
    pub rustfmt_rel_path: String,
    pub rustfmt_state: G3RsFmtRustfmtConfigState,
    pub cargo_rel_path: String,
    pub cargo_state: G3RsFmtCargoState,
    pub toolchain_rel_path: String,
    pub toolchain_state: G3RsFmtToolchainState,
    pub rust_policy: G3RsFmtRustPolicyState,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsFmtSourceChecksInput;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum G3RsFmtConfigFileKind {
    RustfmtToml,
    DotRustfmtToml,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsFmtNestedConfigFile {
    pub rel_path: String,
    pub kind: G3RsFmtConfigFileKind,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct G3RsFmtFileTreeChecksInput {
    pub root_rustfmt_toml_rel_path: Option<String>,
    pub root_dot_rustfmt_toml_rel_path: Option<String>,
    pub nested_config_files: Vec<G3RsFmtNestedConfigFile>,
    pub dual_conflict_dirs: Vec<String>,
}
