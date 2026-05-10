use std::collections::BTreeSet;

use g3rs_test_types::G3RsTestAnalyzedSourceFile;

/// `TestSupportFileInput` struct.
pub(crate) struct TestSupportFileInput<'a> {
    /// `file` item.
    pub(crate) file: &'a G3RsTestAnalyzedSourceFile,
    /// `sibling_files` item.
    pub(crate) sibling_files: &'a [G3RsTestAnalyzedSourceFile],
    /// `local_runtime_packages` item.
    pub(crate) local_runtime_packages: &'a BTreeSet<String>,
    /// `local_assertions_packages` item.
    pub(crate) local_assertions_packages: &'a BTreeSet<String>,
}

impl<'a> TestSupportFileInput<'a> {
    /// `new` function.
    pub(crate) const fn new(
        file: &'a G3RsTestAnalyzedSourceFile,
        sibling_files: &'a [G3RsTestAnalyzedSourceFile],
        local_runtime_packages: &'a BTreeSet<String>,
        local_assertions_packages: &'a BTreeSet<String>,
    ) -> Self {
        Self {
            file,
            sibling_files,
            local_runtime_packages,
            local_assertions_packages,
        }
    }
}
