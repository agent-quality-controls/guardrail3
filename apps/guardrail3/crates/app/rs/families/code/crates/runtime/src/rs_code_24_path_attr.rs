use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_path_attrs, same_line_reason};

const ID: &str = "RS-CODE-24";

fn is_canonical_test_sidecar_path(input: &RustCodeFileInput<'_>, line: usize, path: &str) -> bool {
    let Some(mod_name) = path.strip_suffix("/mod.rs") else {
        return false;
    };
    if !mod_name.ends_with("_tests") {
        return false;
    }
    let lines = input.content.lines().collect::<Vec<_>>();
    let attr_index = line.saturating_sub(1);
    let Some(prev_line) = attr_index.checked_sub(1).and_then(|index| lines.get(index)) else {
        return false;
    };
    let Some(next_line) = lines.get(attr_index + 1) else {
        return false;
    };
    prev_line.trim() == "#[cfg(test)]" && next_line.trim() == format!("mod {mod_name};")
}

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_path_attrs(input.ast) {
        if is_canonical_test_sidecar_path(input, info.line, &info.path) {
            continue;
        }
        if info.path.contains("..") {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "#[path] escapes parent directory".to_owned(),
                message: format!(
                    "`#[path = \"{}\"]` escapes the standard module boundary.",
                    info.path
                ),
                file: Some(input.rel_path.to_owned()),
                line: Some(info.line),
                inventory: false,
            });
            continue;
        }

        let Some(reason) = same_line_reason(input.content, info.line) else {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "#[path] without reason".to_owned(),
                message: format!(
                    "`#[path = \"{}\"]` changes module resolution and requires `// reason:` on the same line.",
                    info.path
                ),
                file: Some(input.rel_path.to_owned()),
                line: Some(info.line),
                inventory: false,
            });
            continue;
        };

        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "#[path] usage".to_owned(),
            message: format!("#[path = \"{}\"] reason: {reason}", info.path),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
    }
}


#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content).unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_24_path_attr_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_24_path_attr_tests;
