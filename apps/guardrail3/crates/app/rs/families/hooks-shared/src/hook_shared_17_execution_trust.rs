use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-17";

pub fn check(trust_risks: &[String], results: &mut Vec<CheckResult>) {
    if trust_risks.is_empty() {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "no competing hook systems detected".to_owned(),
                message: "No obvious alternate hook system or shadowing risk was found.".to_owned(),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "competing hook system detected".to_owned(),
        message: format!(
            "Found alternate hook surfaces that can shadow or confuse hook execution: {}",
            trust_risks.join(", ")
        ),
        file: None,
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "hook_shared_17_execution_trust_tests.rs"]
mod tests;
