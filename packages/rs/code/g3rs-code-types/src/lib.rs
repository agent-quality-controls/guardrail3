use cargo_toml_parser::CargoToml;

#[derive(Debug, Clone)]
pub enum G3RsCodeConfigFileKind {
    CargoToml { cargo: CargoToml },
    Text,
}

#[derive(Debug, Clone)]
pub struct G3RsCodeConfigFile {
    pub rel_path: String,
    pub content: String,
    pub kind: G3RsCodeConfigFileKind,
}

#[derive(Debug, Clone)]
pub struct G3RsCodeConfigChecksInput {
    pub files: Vec<G3RsCodeConfigFile>,
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
