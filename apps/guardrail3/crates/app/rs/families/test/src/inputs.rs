use super::facts::{
    HookFacts, InputFailureFacts, TestCoverageFacts, TestFileFacts, TestRootFacts, ToolFacts,
};
use super::parse::{CfgTestModuleInfo, ParsedTestFile, TestFunctionInfo};

pub struct ToolTestInput<'a> {
    pub tool: &'a ToolFacts,
}

impl<'a> ToolTestInput<'a> {
    pub const fn new(tool: &'a ToolFacts) -> Self {
        Self { tool }
    }
}

pub struct HookTestInput<'a> {
    pub hook: &'a HookFacts,
}

impl<'a> HookTestInput<'a> {
    pub const fn new(hook: &'a HookFacts) -> Self {
        Self { hook }
    }
}

pub struct RootTestInput<'a> {
    pub root: &'a TestRootFacts,
}

impl<'a> RootTestInput<'a> {
    pub const fn new(root: &'a TestRootFacts) -> Self {
        Self { root }
    }
}

pub struct TestCoverageInput<'a> {
    pub coverage: &'a TestCoverageFacts,
}

impl<'a> TestCoverageInput<'a> {
    pub const fn new(coverage: &'a TestCoverageFacts) -> Self {
        Self { coverage }
    }
}

pub struct TestFileInput<'a> {
    pub file: &'a TestFileFacts,
    pub content: &'a str,
    pub parsed: &'a ParsedTestFile,
}

impl<'a> TestFileInput<'a> {
    pub const fn new(
        file: &'a TestFileFacts,
        content: &'a str,
        parsed: &'a ParsedTestFile,
    ) -> Self {
        Self {
            file,
            content,
            parsed,
        }
    }
}

pub struct TestFunctionInput<'a> {
    pub file: &'a TestFileFacts,
    pub function: &'a TestFunctionInfo,
}

impl<'a> TestFunctionInput<'a> {
    pub const fn new(file: &'a TestFileFacts, function: &'a TestFunctionInfo) -> Self {
        Self { file, function }
    }
}

pub struct TestModuleInput<'a> {
    pub file: &'a TestFileFacts,
    pub module: &'a CfgTestModuleInfo,
}

impl<'a> TestModuleInput<'a> {
    pub const fn new(file: &'a TestFileFacts, module: &'a CfgTestModuleInfo) -> Self {
        Self { file, module }
    }
}

pub struct InputFailureTestInput<'a> {
    pub failure: &'a InputFailureFacts,
}

impl<'a> InputFailureTestInput<'a> {
    pub const fn new(failure: &'a InputFailureFacts) -> Self {
        Self { failure }
    }

    pub fn inline(rel_path: &'a str, message: &'a str) -> Self {
        Self {
            failure: Box::leak(Box::new(InputFailureFacts {
                rel_path: rel_path.to_owned(),
                message: message.to_owned(),
            })),
        }
    }
}
