use guardrail3_domain_report::{CheckResult, Severity};

const ID: &str = "HOOK-SHARED-17";

pub fn check(trust_risks: &[String], results: &mut Vec<CheckResult>) {
    if trust_risks.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Warn,
                "no competing hook systems detected".to_owned(),
                "No obvious alternate hook system or shadowing risk was found.".to_owned(),
                None,
                None,
                false,
            )
            .as_inventory(),
        );
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        "competing hook system detected".to_owned(),
        format!(
            "Found alternate hook surfaces that can shadow or confuse hook execution: {}",
            trust_risks.join(", ")
        ),
        None,
        None,
        false,
    ));
}

#[cfg(test)]

mod tests;
