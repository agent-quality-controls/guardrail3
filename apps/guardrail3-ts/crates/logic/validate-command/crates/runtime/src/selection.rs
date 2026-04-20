use guardrail3_ts_app_types::{SUPPORTED_FAMILIES, SupportedFamily, ValidateRequest};

#[must_use]
pub const fn family_cli_name(family: SupportedFamily) -> &'static str {
    match family {
        SupportedFamily::Eslint => "eslint",
    }
}

#[must_use]
pub fn selected_families(request: &ValidateRequest) -> Vec<SupportedFamily> {
    if request.families.is_empty() {
        return SUPPORTED_FAMILIES.to_vec();
    }

    SUPPORTED_FAMILIES
        .into_iter()
        .filter(|family| request.families.contains(family))
        .collect()
}

#[cfg(test)]
#[path = "selection_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod selection_tests;
