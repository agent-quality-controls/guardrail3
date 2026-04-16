use guardrail3_rs_app_types::{SUPPORTED_FAMILIES, SupportedFamily, ValidateRequest};

#[must_use]
pub const fn family_cli_name(family: SupportedFamily) -> &'static str {
    match family {
        SupportedFamily::Topology => "topology",
        SupportedFamily::Toolchain => "toolchain",
        SupportedFamily::Fmt => "fmt",
        SupportedFamily::Cargo => "cargo",
        SupportedFamily::Clippy => "clippy",
        SupportedFamily::Deny => "deny",
        SupportedFamily::Code => "code",
        SupportedFamily::Arch => "arch",
        SupportedFamily::Deps => "deps",
        SupportedFamily::Garde => "garde",
        SupportedFamily::Test => "test",
        SupportedFamily::Release => "release",
        SupportedFamily::Hooks => "hooks",
        SupportedFamily::Apparch => "apparch",
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
