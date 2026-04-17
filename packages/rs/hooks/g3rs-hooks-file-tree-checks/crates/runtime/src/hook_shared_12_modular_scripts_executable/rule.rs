use g3rs_hooks_types::G3RsHooksScriptFileFact;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-HOOKS-FILETREE-06";

pub(crate) fn check(scripts: &[G3RsHooksScriptFileFact], results: &mut Vec<G3CheckResult>) {
    for script in scripts {
        match script.executable {
            Some(true) => results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "modular hook script is executable".to_owned(),
                    "Modular hook script has the executable bit set.".to_owned(),
                    Some(script.rel_path.clone()),
                    None,
                )
                .into_inventory(),
            ),
            Some(false) => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                "modular hook script is not executable".to_owned(),
                "Modular hook script exists but does not have the executable bit set.".to_owned(),
                Some(script.rel_path.clone()),
                None,
            )),
            None => {}
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rule_tests;
