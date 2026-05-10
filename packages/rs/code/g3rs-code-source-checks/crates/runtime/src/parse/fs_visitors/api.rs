use syn::visit::Visit;

use super::{
    inline_std_fs::InlineStdFsVisitor, std_fs_glob_import::StdFsGlobImportVisitor,
    std_fs_import::StdFsImportVisitor,
};

/// Implements `find std fs import lines`.
pub(crate) fn find_std_fs_import_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsImportVisitor::default();
    visitor.visit_file(source);
    visitor.out
}

/// Implements `find inline std fs call lines`.
pub(crate) fn find_inline_std_fs_call_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = InlineStdFsVisitor::default();
    visitor.visit_file(source);
    visitor.out
}

/// Implements `find std fs glob import lines`.
pub(crate) fn find_std_fs_glob_import_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsGlobImportVisitor::default();
    visitor.visit_file(source);
    visitor.out
}
