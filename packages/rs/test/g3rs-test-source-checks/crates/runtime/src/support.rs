#![expect(
    clippy::indexing_slicing,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
#![expect(
    clippy::type_complexity,
    reason = "structural code pattern (parser/assertion helper) where lint conflicts with module architecture"
)]
use std::collections::{BTreeMap, BTreeSet};

use g3rs_test_types::G3RsTestAnalyzedSourceFile;
use g3rs_test_types::ast::{CfgTestModuleInfo, TestFunctionInfo, UseBinding};

/// `TestFileInput` struct.
pub(crate) struct TestFileInput<'a> {
    /// `file` item.
    pub(crate) file: &'a G3RsTestAnalyzedSourceFile,
}

impl<'a> TestFileInput<'a> {
    /// `new` function.
    pub(crate) const fn new(file: &'a G3RsTestAnalyzedSourceFile) -> Self {
        Self { file }
    }
}

/// `TestFunctionInput` struct.
pub(crate) struct TestFunctionInput<'a> {
    /// `file` item.
    pub(crate) file: &'a G3RsTestAnalyzedSourceFile,
    /// `function` item.
    pub(crate) function: &'a TestFunctionInfo,
    /// `proof_bearing_assertion_functions` item.
    pub(crate) proof_bearing_assertion_functions: Option<&'a BTreeSet<String>>,
}

impl<'a> TestFunctionInput<'a> {
    /// `new` function.
    pub(crate) const fn new(
        file: &'a G3RsTestAnalyzedSourceFile,
        function: &'a TestFunctionInfo,
        proof_bearing_assertion_functions: Option<&'a BTreeSet<String>>,
    ) -> Self {
        Self {
            file,
            function,
            proof_bearing_assertion_functions,
        }
    }
}

/// `CfgTestModuleInput` struct.
pub(crate) struct CfgTestModuleInput<'a> {
    /// `file` item.
    pub(crate) file: &'a G3RsTestAnalyzedSourceFile,
    /// `module` item.
    pub(crate) module: &'a CfgTestModuleInfo,
}

impl<'a> CfgTestModuleInput<'a> {
    /// `new` function.
    pub(crate) const fn new(
        file: &'a G3RsTestAnalyzedSourceFile,
        module: &'a CfgTestModuleInfo,
    ) -> Self {
        Self { file, module }
    }
}

/// `AssertionsModuleInput` struct.
pub(crate) struct AssertionsModuleInput<'a> {
    /// `file` item.
    pub(crate) file: &'a G3RsTestAnalyzedSourceFile,
    /// `proof_bearing_exported_functions` item.
    pub(crate) proof_bearing_exported_functions: &'a BTreeSet<String>,
}

impl<'a> AssertionsModuleInput<'a> {
    /// `new` function.
    pub(crate) const fn new(
        file: &'a G3RsTestAnalyzedSourceFile,
        proof_bearing_exported_functions: &'a BTreeSet<String>,
    ) -> Self {
        Self {
            file,
            proof_bearing_exported_functions,
        }
    }
}

/// `normalized_owned_assertion_relative_segments` function.
pub(crate) fn normalized_owned_assertion_relative_segments(
    binding: &UseBinding,
    assertions_package_name: &str,
    root_prefixes: &BTreeMap<String, Vec<String>>,
) -> Option<Vec<String>> {
    let package_root = assertions_package_name.replace('-', "_");
    let mut relative_segments = match binding.path_segments.first()?.as_str() {
        first if first == assertions_package_name || first == package_root => {
            binding.path_segments[1..].to_vec()
        }
        "crate" | "self" | "super" => {
            let target = binding.path_segments.get(1)?;
            let mut relative = root_prefixes.get(target)?.clone();
            relative.extend(binding.path_segments.iter().skip(2).cloned());
            relative
        }
        _ => return None,
    };

    if matches!(relative_segments.as_slice(), [segment] if segment == "self") {
        relative_segments.clear();
    }

    Some(relative_segments)
}
