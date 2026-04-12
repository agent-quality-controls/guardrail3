use g3rs_deps_filetree_checks_types::G3RsDepsFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_rs_toml_parser::RustProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Finding {
    pub id: String,
    pub severity: G3Severity,
    pub title: String,
    pub message: String,
    pub file: Option<String>,
    pub inventory: bool,
}

pub(crate) fn input(
    profile: Option<RustProfile>,
    cargo_lock_exists: bool,
    cargo_lock_ignored: bool,
    gitignore_rel_path: Option<&str>,
) -> G3RsDepsFileTreeChecksInput {
    G3RsDepsFileTreeChecksInput {
        profile,
        cargo_lock_rel_path: "Cargo.lock".to_owned(),
        cargo_lock_exists,
        cargo_lock_ignored,
        gitignore_rel_path: gitignore_rel_path.map(str::to_owned),
    }
}

pub(crate) fn findings(results: &[G3CheckResult]) -> Vec<Finding> {
    let mut findings = results
        .iter()
        .map(|result| Finding {
            id: result.id().to_owned(),
            severity: result.severity(),
            title: result.title().to_owned(),
            message: result.message().to_owned(),
            file: result.file().map(str::to_owned),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            left.id.as_str(),
            format!("{:?}", left.severity),
            left.title.as_str(),
            left.message.as_str(),
            left.file.as_deref(),
            left.inventory,
        )
            .cmp(&(
                right.id.as_str(),
                format!("{:?}", right.severity),
                right.title.as_str(),
                right.message.as_str(),
                right.file.as_deref(),
                right.inventory,
            ))
    });
    findings
}
