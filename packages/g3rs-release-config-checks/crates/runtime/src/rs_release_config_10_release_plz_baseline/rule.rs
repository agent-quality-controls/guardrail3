use g3rs_release_config_checks_types::G3RsReleaseConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

use crate::support::{info, warn};

/// Check ID for release-plz baseline configuration.
const ID: &str = "RS-RELEASE-CONFIG-10";

/// Verify baseline release-plz.toml settings.
pub(crate) fn check(input: &G3RsReleaseConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    let release_plz = match input.release_plz.as_ref() {
        Some(r) => r,
        None => return,
    };

    let file = input
        .release_plz_rel_path
        .as_deref()
        .unwrap_or("release-plz.toml");

    let mut issues = 0usize;

    // Check workspace section exists.
    let workspace = release_plz.workspace.as_ref();
    if workspace.is_none() {
        issues = issues.saturating_add(1);
        results.push(warn(
            ID,
            "release-plz: missing [workspace] section".to_owned(),
            "release-plz.toml should have a [workspace] section.".to_owned(),
            file,
        ));
    }

    // Check changelog_config == "cliff.toml".
    let changelog_ok = workspace
        .and_then(|w| w.changelog_config.as_deref())
        .is_some_and(|c| c == "cliff.toml");
    if !changelog_ok {
        issues = issues.saturating_add(1);
        results.push(warn(
            ID,
            "release-plz: changelog_config should be \"cliff.toml\"".to_owned(),
            "Set changelog_config = \"cliff.toml\" in [workspace].".to_owned(),
            file,
        ));
    }

    // Check git_release_enable == true.
    let git_release_ok = workspace
        .and_then(|w| w.git_release_enable)
        .is_some_and(|v| v);
    if !git_release_ok {
        issues = issues.saturating_add(1);
        results.push(warn(
            ID,
            "release-plz: git_release_enable should be true".to_owned(),
            "Set git_release_enable = true in [workspace].".to_owned(),
            file,
        ));
    }

    // Check release_always == false.
    let release_always_ok = workspace
        .and_then(|w| w.release_always)
        .is_some_and(|v| !v);
    if !release_always_ok {
        issues = issues.saturating_add(1);
        results.push(warn(
            ID,
            "release-plz: release_always should be false".to_owned(),
            "Set release_always = false in [workspace].".to_owned(),
            file,
        ));
    }

    if issues == 0 {
        results.push(info(
            ID,
            "release-plz: baseline configuration correct".to_owned(),
            String::new(),
            file,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
