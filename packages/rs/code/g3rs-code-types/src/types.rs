use cargo_toml_parser::types::CargoToml;
use clippy_toml_parser::types::ClippyToml;
use deny_toml_parser::types::DenyToml;
use guardrail3_rs_toml_parser::types::Guardrail3RsToml;
use rust_toolchain_toml_parser::RustToolchainToml;
use rustfmt_toml_parser::RustfmtToml;

#[derive(Debug, Clone)]
pub enum G3RsCodeConfigFileKind {
    Guardrail3RsToml { guardrail3: Guardrail3RsToml },
    ClippyToml { clippy: ClippyToml },
    DenyToml { deny: DenyToml },
    CargoToml { cargo: CargoToml },
    RustfmtToml { rustfmt: RustfmtToml },
    RustToolchainToml { toolchain: RustToolchainToml },
}

#[derive(Debug, Clone)]
pub struct G3RsCodeConfigFile {
    pub rel_path: String,
    pub kind: G3RsCodeConfigFileKind,
}

#[derive(Debug, Clone)]
pub struct G3RsCodeExceptionComment {
    pub rel_path: String,
    pub line: usize,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct G3RsCodeConfigChecksInput {
    pub files: Vec<G3RsCodeConfigFile>,
    pub exception_comments: Vec<G3RsCodeExceptionComment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsSourceFile {
    pub rel_path: String,
    pub content: String,
    pub is_test: bool,
    pub profile_name: Option<String>,
    pub is_library_root: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeSourceChecksInput {
    pub source_file: G3RsSourceFile,
    pub is_shared_crate: bool,
    pub waivers: Vec<G3RsCodeWaiver>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeWaiver {
    pub rule: String,
    pub file: String,
    pub selector: String,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeStructuralCapRoot {
    pub root_rel_dir: String,
    pub cargo_rel_path: String,
    pub max_module_depth: usize,
    pub max_sibling_dirs: usize,
    pub max_sibling_rs_files: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3RsCodeFileTreeChecksInput {
    pub roots: Vec<G3RsCodeStructuralCapRoot>,
}
