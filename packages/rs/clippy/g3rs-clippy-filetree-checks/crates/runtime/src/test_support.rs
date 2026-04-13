use g3rs_clippy_filetree_checks_types::{
    G3RsClippyFileTreeChecksInput, G3RsClippyShadowedConfig,
};
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
    preferred_root_config_rel_path: Option<&str>,
    shadowed_same_root_configs: &[(&str, &str)],
) -> G3RsClippyFileTreeChecksInput {
    G3RsClippyFileTreeChecksInput {
        preferred_root_config_rel_path: preferred_root_config_rel_path.map(str::to_owned),
        shadowed_same_root_configs: shadowed_same_root_configs
            .iter()
            .map(|(rel_path, preferred_rel_path)| G3RsClippyShadowedConfig {
                rel_path: (*rel_path).to_owned(),
                preferred_rel_path: (*preferred_rel_path).to_owned(),
            })
            .collect(),
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
