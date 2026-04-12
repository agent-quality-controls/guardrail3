use g3rs_toolchain_filetree_checks_types::G3RsToolchainFileTreeChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

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
    toolchain_toml_rel_path: Option<&str>,
    legacy_toolchain_rel_path: Option<&str>,
) -> G3RsToolchainFileTreeChecksInput {
    G3RsToolchainFileTreeChecksInput {
        toolchain_toml_rel_path: toolchain_toml_rel_path.map(str::to_owned),
        legacy_toolchain_rel_path: legacy_toolchain_rel_path.map(str::to_owned),
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
