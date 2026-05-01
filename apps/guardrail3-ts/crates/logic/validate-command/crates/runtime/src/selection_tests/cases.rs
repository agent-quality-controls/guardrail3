use guardrail3_ts_app_types::{SupportedFamily, ValidateRequest};
use guardrail3_ts_validate_command_assertions::selection as assertions;

#[test]
fn selected_families_follow_canonical_order() {
    let request = ValidateRequest {
        workspace_root: "ignored".into(),
        families: vec![SupportedFamily::Eslint],
        include_inventory: false,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        &[SupportedFamily::Eslint],
    );
}

#[test]
fn selected_families_default_to_all_supported_families_when_filter_is_empty() {
    let request = ValidateRequest {
        workspace_root: "ignored".into(),
        families: Vec::new(),
        include_inventory: true,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        &[
            SupportedFamily::Eslint,
            SupportedFamily::AstroSetup,
            SupportedFamily::AstroContent,
            SupportedFamily::AstroMdx,
            SupportedFamily::AstroI18n,
            SupportedFamily::AstroMedia,
            SupportedFamily::AstroSeo,
            SupportedFamily::AstroState,
            SupportedFamily::Arch,
            SupportedFamily::Apparch,
            SupportedFamily::Tsconfig,
            SupportedFamily::Package,
            SupportedFamily::Npmrc,
            SupportedFamily::Jscpd,
            SupportedFamily::Style,
            SupportedFamily::Fmt,
            SupportedFamily::Hooks,
        ],
    );
}

#[test]
fn selected_families_deduplicate_repeated_entries() {
    let request = ValidateRequest {
        workspace_root: "ignored".into(),
        families: vec![SupportedFamily::Eslint, SupportedFamily::Eslint],
        include_inventory: false,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        &[SupportedFamily::Eslint],
    );
}

#[test]
fn selected_families_keep_canonical_order_with_arch_in_mixed_filter() {
    let request = ValidateRequest {
        workspace_root: "ignored".into(),
        families: vec![
            SupportedFamily::Jscpd,
            SupportedFamily::Arch,
            SupportedFamily::AstroSetup,
            SupportedFamily::Eslint,
        ],
        include_inventory: false,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        &[
            SupportedFamily::Eslint,
            SupportedFamily::AstroSetup,
            SupportedFamily::Arch,
            SupportedFamily::Jscpd,
        ],
    );
}

#[test]
fn selected_families_keep_canonical_order_with_astro_in_mixed_filter() {
    let request = ValidateRequest {
        workspace_root: "ignored".into(),
        families: vec![
            SupportedFamily::Jscpd,
            SupportedFamily::Arch,
            SupportedFamily::AstroSetup,
        ],
        include_inventory: false,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        &[
            SupportedFamily::AstroSetup,
            SupportedFamily::Arch,
            SupportedFamily::Jscpd,
        ],
    );
}

#[test]
fn selected_families_keep_canonical_order_with_apparch_in_mixed_filter() {
    let request = ValidateRequest {
        workspace_root: "ignored".into(),
        families: vec![
            SupportedFamily::Jscpd,
            SupportedFamily::Apparch,
            SupportedFamily::Arch,
        ],
        include_inventory: false,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        &[
            SupportedFamily::Arch,
            SupportedFamily::Apparch,
            SupportedFamily::Jscpd,
        ],
    );
}

#[test]
fn selected_families_keep_canonical_order_with_jscpd_in_mixed_filter() {
    let request = ValidateRequest {
        workspace_root: "ignored".into(),
        families: vec![
            SupportedFamily::Jscpd,
            SupportedFamily::Eslint,
            SupportedFamily::Jscpd,
        ],
        include_inventory: false,
    };

    assertions::assert_selected_families(
        &super::super::selected_families(&request),
        &[SupportedFamily::Eslint, SupportedFamily::Jscpd],
    );
}
