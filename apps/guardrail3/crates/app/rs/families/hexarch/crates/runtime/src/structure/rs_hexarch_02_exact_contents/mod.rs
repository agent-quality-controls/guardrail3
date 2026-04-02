use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::HexRootInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-02";
const EXPECTED: [&str; 4] = ["adapters", "app", "domain", "ports"];
const OPTIONAL: [&str; 1] = ["macros"];

pub fn check(input: &HexRootInput<'_>, results: &mut Vec<CheckResult>) {
    let before = results.len();
    for expected in EXPECTED {
        if input.dirs.iter().any(|dir| dir == expected)
            && !input.symlink_dirs.iter().any(|dir| dir == expected)
        {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Service `{}` missing {}/{}/ directory",
                input.app_name,
                label(input),
                expected
            ),
            format!(
                "Service `{}` is missing `{}/{}/`. Create it and add a `.gitkeep` if not needed yet.",
                input.app_name,
                label(input),
                expected
            ),
            Some(input.crates_rel_dir.to_owned()),
            None,
            false,
        ));
    }

    for dir in input
        .dirs
        .iter()
        .filter(|dir| !input.symlink_dirs.iter().any(|symlink| symlink == *dir))
    {
        if EXPECTED.contains(&dir.as_str()) || OPTIONAL.contains(&dir.as_str()) {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!(
                "Service `{}` has unexpected directory {}/{}/",
                input.app_name,
                label(input),
                dir
            ),
            format!(
                "Service `{}` has `{}/{}/` which is not part of the hexarch template. Required directories are `{{adapters, app, domain, ports}}`; optional `macros` is also allowed in `{}/`.",
                input.app_name,
                label(input),
                dir,
                label(input)
            ),
            Some(format!("{}/{}", input.crates_rel_dir, dir)),
            None,
            false,
        ));
    }

    let bad_files: Vec<_> = input
        .files
        .iter()
        .filter(|file| file.as_str() != ".gitkeep")
        .cloned()
        .chain(input.symlink_dirs.iter().cloned())
        .chain(input.symlink_files.iter().cloned())
        .collect();
    if !bad_files.is_empty() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("Service `{}` has loose files in {}/", input.app_name, label(input)),
            format!(
                "Service `{}` has files in `{}/` that don't belong: {}. Only `.gitkeep` is allowed alongside the top-level hex directories.",
                input.app_name,
                label(input),
                bad_files.join(", ")
            ),
            Some(input.crates_rel_dir.to_owned()),
            None,
            false,
        ));
    }

    if results.len() == before {
        push_success(
            results,
            ID,
            format!(
                "Service `{}` has exact top-level hex contents",
                input.app_name
            ),
            format!(
                "Service `{}` keeps `{}` aligned with the required top-level hex template.",
                input.app_name, input.crates_rel_dir
            ),
            Some(input.crates_rel_dir.to_owned()),
        );
    }
}

fn label<'a>(input: &'a HexRootInput<'a>) -> &'a str {
    input
        .crates_rel_dir
        .strip_prefix(input.app_rel_dir)
        .unwrap_or(input.crates_rel_dir)
        .trim_start_matches('/')
}

#[cfg(test)]
pub(crate) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]

mod tests;
