use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use super::inputs::RustCodeFileInput;
use super::parse::{
    CfgPredicateTruth, find_path_attrs, path_string_has_parent_segment, same_line_reason,
};

const ID: &str = "RS-CODE-24";

fn is_canonical_test_sidecar_path(input: &RustCodeFileInput<'_>, line: usize, path: &str) -> bool {
    let Some(current_stem) = std::path::Path::new(input.rel_path)
        .file_stem()
        .and_then(std::ffi::OsStr::to_str)
    else {
        return false;
    };
    let expected_mod_name = format!("{current_stem}_tests");
    let expected_path = format!("{expected_mod_name}/mod.rs");
    if path != expected_path {
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
    let expected_mod_line = format!("mod {expected_mod_name};");
    prev_line.trim() == "#[cfg(test)]"
        && (next_line.trim() == "mod tests;" || next_line.trim() == expected_mod_line)
}

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_path_attrs(input.ast) {
        if info.cfg_truth == CfgPredicateTruth::KnownFalse {
            continue;
        }
        if !info.via_cfg_attr && is_canonical_test_sidecar_path(input, info.line, &info.path) {
            continue;
        }
        if path_string_has_parent_segment(&info.path) {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "#[path] escapes parent directory".to_owned(),
                format!(
                    "`#[path = \"{}\"]` escapes the standard module boundary.",
                    info.path
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
                false,
            ));
            continue;
        }

        let Some(reason) = same_line_reason(input.content, info.line) else {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "#[path] without reason".to_owned(),
                format!(
                    "`#[path = \"{}\"]` changes module resolution and requires `// reason:` on the same line.",
                    info.path
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
                false,
            ));
            continue;
        };
        if !reason_text_is_useful(&reason) {
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "#[path] reason too weak".to_owned(),
                format!(
                    "`#[path = \"{}\"]` reason must be specific and at least two words. Weak reason `{reason}` found.",
                    info.path
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
                false,
            ));
            continue;
        }

        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "#[path] usage".to_owned(),
            format!("#[path = \"{}\"] reason: {reason}", info.path),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
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
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test_root,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_24_path_attr_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_24_path_attr_tests;
