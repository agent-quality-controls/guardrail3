use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-HOOKS-FILETREE-07";

pub(crate) fn check(trust_risks: &[String], results: &mut Vec<G3CheckResult>) {
    if trust_risks.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                "no competing hook systems detected".to_owned(),
                "No obvious alternate hook system or shadowing risk was found.".to_owned(),
                None,
                None,
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "competing hook system detected".to_owned(),
        format!(
            "Found alternate hook surfaces that can shadow or confuse hook execution: {}",
            trust_risks.join(", ")
        ),
        None,
        None,
    ));
}

#[cfg(test)]
#[path = "hook_shared_17_execution_trust_tests/mod.rs"]
mod tests;
