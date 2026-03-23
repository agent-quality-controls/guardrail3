use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::domain::project_tree::{DirEntry, ProjectTree};
use crate::ports::outbound::ToolChecker;

use super::facts::{
    HookFacts, InputFailureFacts, TestCoverageFacts, TestFileFacts, TestRootFacts, TestRootKind,
    ToolFacts,
};
use super::inputs::{
    HookTestInput, InputFailureTestInput, RootTestInput, TestCoverageInput, TestFileInput,
    TestFunctionInput, TestModuleInput, ToolTestInput,
};
use super::parse::{CfgTestModuleInfo, ParsedTestFile, TestFunctionInfo};

pub fn dir_entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|value| (*value).to_owned()).collect(),
        files: files.iter().map(|value| (*value).to_owned()).collect(),
    }
}

pub fn project_tree(structure: Vec<(&str, DirEntry)>, content: Vec<(&str, &str)>, root: PathBuf) -> ProjectTree {
    ProjectTree {
        root,
        structure: structure
            .into_iter()
            .map(|(rel, entry)| (rel.to_owned(), entry))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .into_iter()
            .map(|(rel, body)| (rel.to_owned(), body.to_owned()))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos()
    );
    std::env::temp_dir().join(unique)
}

pub fn write_file(root: &Path, rel_path: &str, content: &str) {
    let abs = root.join(rel_path);
    if let Some(parent) = abs.parent() {
        std::fs::create_dir_all(parent).expect("create parent");
    }
    std::fs::write(abs, content).expect("write file");
}

pub struct StubToolChecker {
    installed: bool,
}

impl StubToolChecker {
    pub const fn new(installed: bool) -> Self {
        Self { installed }
    }
}

impl ToolChecker for StubToolChecker {
    fn is_installed(&self, tool: &str) -> bool {
        tool == "cargo-mutants" && self.installed
    }

    fn run_cargo_publish_dry_run(&self, _path: &Path) -> Option<String> {
        None
    }
}

pub fn tool_input(installed: bool) -> ToolTestInput<'static> {
    ToolTestInput::new(Box::leak(Box::new(ToolFacts { installed })))
}

pub fn hook_input(matching_files: &[&str]) -> HookTestInput<'static> {
    HookTestInput::new(Box::leak(Box::new(HookFacts {
        matching_files: matching_files.iter().map(|value| (*value).to_owned()).collect(),
    })))
}

pub fn root_input(
    kind: TestRootKind,
    mutants_exists: bool,
    has_mutants_profile: bool,
    tokio_present: bool,
    nextest_exists: bool,
    nextest_toml: Option<&str>,
    mutants_toml: Option<&str>,
) -> RootTestInput<'static> {
    let nextest_parsed = nextest_toml.map(|body| toml::from_str(body).expect("parse nextest"));
    let mutants_parsed = mutants_toml.map(|body| toml::from_str(body).expect("parse mutants"));
    RootTestInput::new(Box::leak(Box::new(TestRootFacts {
        rel_dir: String::new(),
        kind,
        cargo_rel_path: "Cargo.toml".to_owned(),
        mutants_rel_path: ".cargo/mutants.toml".to_owned(),
        mutants_exists,
        mutants_parsed,
        nextest_rel_path: ".config/nextest.toml".to_owned(),
        nextest_exists,
        nextest_parsed,
        nextest_parse_error: None,
        tokio_present,
        has_mutants_profile,
    })))
}

pub fn coverage_input(
    has_any_tests: bool,
    public_fn_count: usize,
    test_fn_count: usize,
    integration_test_exists: bool,
) -> TestCoverageInput<'static> {
    TestCoverageInput::new(Box::leak(Box::new(TestCoverageFacts {
        root_rel_dir: String::new(),
        has_any_tests,
        public_fn_count,
        test_fn_count,
        integration_test_exists,
    })))
}

pub fn file_input(
    rel_path: &str,
    is_src_file: bool,
    is_integration_test_file: bool,
    is_test_sidecar_file: bool,
    content: &str,
    parsed: ParsedTestFile,
) -> TestFileInput<'static> {
    let file = Box::leak(Box::new(TestFileFacts {
        rel_path: rel_path.to_owned(),
        root_rel_dir: String::new(),
        is_src_file,
        is_integration_test_file,
        is_test_sidecar_file,
    }));
    let content = Box::leak(content.to_owned().into_boxed_str());
    let parsed = Box::leak(Box::new(parsed));
    TestFileInput::new(file, content, parsed)
}

pub fn function_input(
    rel_path: &str,
    function: TestFunctionInfo,
) -> TestFunctionInput<'static> {
    let file = Box::leak(Box::new(TestFileFacts {
        rel_path: rel_path.to_owned(),
        root_rel_dir: String::new(),
        is_src_file: false,
        is_integration_test_file: true,
        is_test_sidecar_file: false,
    }));
    let function = Box::leak(Box::new(function));
    TestFunctionInput::new(file, function)
}

pub fn module_input(rel_path: &str, name: &str, has_body: bool) -> TestModuleInput<'static> {
    let file = Box::leak(Box::new(TestFileFacts {
        rel_path: rel_path.to_owned(),
        root_rel_dir: String::new(),
        is_src_file: true,
        is_integration_test_file: false,
        is_test_sidecar_file: false,
    }));
    let module = Box::leak(Box::new(CfgTestModuleInfo {
        line: 3,
        name: name.to_owned(),
        has_body,
    }));
    TestModuleInput::new(file, module)
}

pub fn failure_input(rel_path: &str, message: &str) -> InputFailureTestInput<'static> {
    InputFailureTestInput::new(Box::leak(Box::new(InputFailureFacts {
        rel_path: rel_path.to_owned(),
        message: message.to_owned(),
    })))
}
