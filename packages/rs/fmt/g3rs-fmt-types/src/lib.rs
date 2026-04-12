use cargo_toml_parser::CargoToml;
use rust_toolchain_toml_parser::RustToolchainToml;
use rustfmt_toml_parser::RustfmtToml;

#[derive(Debug, Clone)]
pub enum G3RsFmtRustfmtConfigState {
    Parsed(RustfmtToml),
    ParseError,
}

#[derive(Debug, Clone)]
pub enum G3RsFmtCargoState {
    Parsed(CargoToml),
    Missing,
    ParseError,
}

#[derive(Debug, Clone)]
pub enum G3RsFmtToolchainState {
    Parsed(RustToolchainToml),
    Missing,
    ParseError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsFmtEscapeHatch {
    pub family: String,
    pub file: String,
    pub kind: String,
    pub selector: String,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct G3RsFmtConfigChecksInput {
    pub rustfmt_rel_path: String,
    pub rustfmt_state: G3RsFmtRustfmtConfigState,
    pub cargo_rel_path: String,
    pub cargo_state: G3RsFmtCargoState,
    pub toolchain_rel_path: String,
    pub toolchain_state: G3RsFmtToolchainState,
    pub escape_hatches: Vec<G3RsFmtEscapeHatch>,
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
