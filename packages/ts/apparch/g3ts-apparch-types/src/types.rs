#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsApparchLayer {
    App,
    Types,
    Logic,
    IoInbound,
    IoOutbound,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsApparchImportKind {
    Import,
    Reexport,
    DynamicImport,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct G3TsApparchSourceFile {
    pub rel_path: String,
    pub layer: G3TsApparchLayer,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct G3TsApparchInternalEdge {
    pub from_rel_path: String,
    pub from_layer: G3TsApparchLayer,
    pub to_rel_path: String,
    pub to_layer: G3TsApparchLayer,
    pub kind: G3TsApparchImportKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct G3TsApparchExternalImport {
    pub from_rel_path: String,
    pub from_layer: G3TsApparchLayer,
    pub module_name: String,
    pub kind: G3TsApparchImportKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum G3TsApparchPublicItemKind {
    Interface,
    Function,
    Class,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct G3TsApparchPublicItem {
    pub rel_path: String,
    pub layer: G3TsApparchLayer,
    pub item_name: String,
    pub kind: G3TsApparchPublicItemKind,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsApparchConfigChecksInput {
    pub files: Vec<G3TsApparchSourceFile>,
    pub internal_edges: Vec<G3TsApparchInternalEdge>,
    pub external_imports: Vec<G3TsApparchExternalImport>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct G3TsApparchSourceChecksInput {
    pub files: Vec<G3TsApparchSourceFile>,
    pub public_items: Vec<G3TsApparchPublicItem>,
}
