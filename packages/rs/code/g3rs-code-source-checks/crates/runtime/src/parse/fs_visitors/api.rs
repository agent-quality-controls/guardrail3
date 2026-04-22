use syn::visit::Visit;

use super::{
    inline_std_fs::InlineStdFsVisitor, std_fs_glob_import::StdFsGlobImportVisitor,
    std_fs_import::StdFsImportVisitor,
};

pub(crate) fn find_std_fs_import_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsImportVisitor::default();
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_inline_std_fs_call_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = InlineStdFsVisitor::default();
    visitor.visit_file(source);
    visitor.out
}

pub(crate) fn find_std_fs_glob_import_lines(source: &syn::File) -> Vec<usize> {
    let mut visitor = StdFsGlobImportVisitor::default();
    visitor.visit_file(source);
    visitor.out
}
