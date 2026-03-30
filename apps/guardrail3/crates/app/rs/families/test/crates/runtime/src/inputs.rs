use std::collections::BTreeSet;

use super::facts::{
    DiscoveredTestFile, InputFailureFacts, RuntimeAssertionsViolation, SidecarViolation,
    TestRootFacts,
};
use super::parse::{CfgTestModuleInfo, ParsedTestFile, TestFunctionInfo};

pub struct RootTestInput<'a> {
    pub(crate) root: &'a TestRootFacts,
    pub(crate) has_tests: bool,
    pub(crate) has_tokio_tests: bool,
    pub(crate) cargo_mutants_installed: bool,
    pub(crate) mutation_hook_files: &'a [String],
}

impl<'a> RootTestInput<'a> {
    pub const fn new(
        root: &'a TestRootFacts,
        has_tests: bool,
        has_tokio_tests: bool,
        cargo_mutants_installed: bool,
        mutation_hook_files: &'a [String],
    ) -> Self {
        Self {
            root,
            has_tests,
            has_tokio_tests,
            cargo_mutants_installed,
            mutation_hook_files,
        }
    }
}

pub struct TestFileInput<'a> {
    pub(crate) file: &'a DiscoveredTestFile,
    pub(crate) parsed: &'a ParsedTestFile,
}

impl<'a> TestFileInput<'a> {
    pub const fn new(file: &'a DiscoveredTestFile, parsed: &'a ParsedTestFile) -> Self {
        Self { file, parsed }
    }
}

pub struct TestFunctionInput<'a> {
    pub(crate) file: &'a DiscoveredTestFile,
    pub(crate) parsed: &'a ParsedTestFile,
    pub(crate) function: &'a TestFunctionInfo,
    pub(crate) proof_bearing_assertion_functions: Option<&'a BTreeSet<String>>,
}

impl<'a> TestFunctionInput<'a> {
    pub const fn new(
        file: &'a DiscoveredTestFile,
        parsed: &'a ParsedTestFile,
        function: &'a TestFunctionInfo,
        proof_bearing_assertion_functions: Option<&'a BTreeSet<String>>,
    ) -> Self {
        Self {
            file,
            parsed,
            function,
            proof_bearing_assertion_functions,
        }
    }
}

pub struct CfgTestModuleInput<'a> {
    pub(crate) file: &'a DiscoveredTestFile,
    pub(crate) module: &'a CfgTestModuleInfo,
}

impl<'a> CfgTestModuleInput<'a> {
    pub const fn new(file: &'a DiscoveredTestFile, module: &'a CfgTestModuleInfo) -> Self {
        Self { file, module }
    }
}

pub struct InputFailureTestInput<'a> {
    pub(crate) failure: &'a InputFailureFacts,
}

impl<'a> InputFailureTestInput<'a> {
    pub const fn new(failure: &'a InputFailureFacts) -> Self {
        Self { failure }
    }
}

pub struct SidecarViolationInput<'a> {
    pub(crate) violation: &'a SidecarViolation,
}

impl<'a> SidecarViolationInput<'a> {
    pub const fn new(violation: &'a SidecarViolation) -> Self {
        Self { violation }
    }
}

pub struct RuntimeAssertionsViolationInput<'a> {
    pub(crate) violation: &'a RuntimeAssertionsViolation,
}

impl<'a> RuntimeAssertionsViolationInput<'a> {
    pub const fn new(violation: &'a RuntimeAssertionsViolation) -> Self {
        Self { violation }
    }
}

pub struct AssertionsModuleInput<'a> {
    pub(crate) file: &'a DiscoveredTestFile,
    pub(crate) parsed: &'a ParsedTestFile,
    pub(crate) proof_bearing_exported_functions: &'a BTreeSet<String>,
}

impl<'a> AssertionsModuleInput<'a> {
    pub const fn new(
        file: &'a DiscoveredTestFile,
        parsed: &'a ParsedTestFile,
        proof_bearing_exported_functions: &'a BTreeSet<String>,
    ) -> Self {
        Self {
            file,
            parsed,
            proof_bearing_exported_functions,
        }
    }
}

pub struct TestSupportFileInput<'a> {
    pub(crate) file: &'a DiscoveredTestFile,
    pub(crate) parsed: &'a ParsedTestFile,
    pub(crate) local_runtime_packages: &'a BTreeSet<String>,
    pub(crate) local_assertions_packages: &'a BTreeSet<String>,
}

impl<'a> TestSupportFileInput<'a> {
    pub const fn new(
        file: &'a DiscoveredTestFile,
        parsed: &'a ParsedTestFile,
        local_runtime_packages: &'a BTreeSet<String>,
        local_assertions_packages: &'a BTreeSet<String>,
    ) -> Self {
        Self {
            file,
            parsed,
            local_runtime_packages,
            local_assertions_packages,
        }
    }
}
