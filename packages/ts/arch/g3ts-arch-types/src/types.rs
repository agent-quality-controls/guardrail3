#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsArchEntryPointSource {
    ExportsDot,
    Types,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchDeclaredEntryPoint {
    pub source: G3TsArchEntryPointSource,
    pub rel_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchManifestSnapshot {
    pub rel_path: String,
    pub declared_entrypoints: Vec<G3TsArchDeclaredEntryPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsArchManifestState {
    Missing,
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { snapshot: G3TsArchManifestSnapshot },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchConfigChecksInput {
    pub manifest: G3TsArchManifestState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchFileTreeChecksInput {
    pub manifest: G3TsArchManifestState,
    pub existing_entrypoints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchFacadeItem {
    pub line: usize,
    pub kind: &'static str,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchFacadeReexport {
    pub line: usize,
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchFacadeSurface {
    pub rel_path: String,
    pub body_items: Vec<G3TsArchFacadeItem>,
    pub broad_reexports: Vec<G3TsArchFacadeReexport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum G3TsArchFacadeFileState {
    Unreadable { rel_path: String, reason: String },
    ParseError { rel_path: String, reason: String },
    Parsed { surface: G3TsArchFacadeSurface },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsArchSourceChecksInput {
    pub facades: Vec<G3TsArchFacadeFileState>,
}
