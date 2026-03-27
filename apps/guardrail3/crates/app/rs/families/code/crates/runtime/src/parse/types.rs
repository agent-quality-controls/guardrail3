#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LargeTypeItem {
    Struct {
        line: usize,
        name: String,
        field_count: usize,
    },
    Enum {
        line: usize,
        name: String,
        variant_count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplAllowInfo {
    pub line: usize,
    pub lint: String,
    pub method_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DenyForbidInfo {
    pub line: usize,
    pub lint: String,
    pub level: String,
    pub crate_level_inner: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncludeMacroInfo {
    pub line: usize,
    pub macro_name: String,
    pub build_script_pattern: bool,
    pub path_traversal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathAttrInfo {
    pub line: usize,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicResultErrorKind {
    String,
    BoxDynError,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicResultErrorInfo {
    pub line: usize,
    pub fn_name: String,
    pub kind: PublicResultErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacadeBodyItemInfo {
    pub line: usize,
    pub kind: &'static str,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitMethodCountInfo {
    pub line: usize,
    pub trait_name: String,
    pub method_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignModAllowInfo {
    pub line: usize,
    pub lint: String,
    pub via_cfg_attr: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestExpectCallInfo {
    pub line: usize,
    pub message: Option<String>,
}
