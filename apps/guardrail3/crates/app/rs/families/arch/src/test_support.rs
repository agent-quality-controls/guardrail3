use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use super::check;
use guardrail3_app_rs_family_mapper::FamilyMapper;
use guardrail3_domain_config::types::GuardrailConfig;
use guardrail3_domain_project_tree::{DirEntry, ProjectTree};
use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_validation_model::{RustFamilySelection, RustValidateFamily};

pub const APP_WORKSPACE_CARGO: &str = "[workspace]\nmembers = []\nresolver = \"2\"\n";
pub const PACKAGE_CARGO: &str = "[package]\nname = \"shared\"\nedition = \"2024\"\n";

pub fn entry(dirs: &[&str], files: &[&str]) -> DirEntry {
    DirEntry {
        dirs: dirs.iter().map(|value| (*value).to_owned()).collect(),
        files: files.iter().map(|value| (*value).to_owned()).collect(),
        symlink_dirs: Vec::new(),
        symlink_files: Vec::new(),
    }
}

pub fn tree(structure: &[(&str, DirEntry)], content: &[(&str, &str)]) -> ProjectTree {
    tree_at("/tmp/arch", structure, content)
}

pub fn tree_at(
    root: &str,
    structure: &[(&str, DirEntry)],
    content: &[(&str, &str)],
) -> ProjectTree {
    ProjectTree {
        root: PathBuf::from(root),
        structure: structure
            .iter()
            .map(|(path, dir_entry)| ((*path).to_owned(), dir_entry.clone()))
            .collect::<BTreeMap<_, _>>(),
        content: content
            .iter()
            .map(|(path, body)| ((*path).to_owned(), (*body).to_owned()))
            .collect::<BTreeMap<_, _>>(),
    }
}

pub fn check_results(tree: &ProjectTree) -> Vec<CheckResult> {
    let route = route(tree);
    check(tree, &route)
}

pub fn error_results<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| result.id == id && result.severity == Severity::Error)
        .collect()
}

pub fn info_results<'a>(results: &'a [CheckResult], id: &str) -> Vec<&'a CheckResult> {
    results
        .iter()
        .filter(|result| result.id == id && result.severity == Severity::Info)
        .collect()
}

pub fn error_files(results: &[CheckResult], id: &str) -> BTreeSet<String> {
    error_results(results, id)
        .into_iter()
        .filter_map(|result| result.file.clone())
        .collect()
}

pub fn info_files(results: &[CheckResult], id: &str) -> BTreeSet<String> {
    info_results(results, id)
        .into_iter()
        .filter_map(|result| result.file.clone())
        .collect()
}

pub fn assert_error_files(results: &[CheckResult], id: &str, expected: &[&str]) {
    let actual = error_files(results, id);
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "unexpected {id} hit set: {results:#?}");
}

pub fn assert_info_files(results: &[CheckResult], id: &str, expected: &[&str]) {
    let actual = info_files(results, id);
    let expected = expected
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    assert_eq!(actual, expected, "unexpected {id} hit set: {results:#?}");
}

fn route(tree: &ProjectTree) -> guardrail3_app_rs_family_mapper::RsArchRoute {
    let scope = guardrail3_app_rs_placement::collect(tree);
    let config = tree
        .file_content("guardrail3.toml")
        .and_then(|content| toml::from_str::<GuardrailConfig>(content).ok());
    let selection = RustFamilySelection::new(BTreeSet::from([RustValidateFamily::Arch]));
    FamilyMapper::new(tree, &scope, config.as_ref(), &selection, None).map_rs_arch()
}
