use g3rs_fmt_config_checks::G3RsFmtConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_app_rs_family_view::FamilyView;
use guardrail3_domain_report::{CheckResult, Severity};

use crate::facts::{collect, file_name_kind};
use crate::inputs::{RustfmtDualConflictInput, RustfmtExtraConfigInput, RustfmtRootInput};

pub fn check(
    surface: &FamilyView,
    route: &guardrail3_app_rs_family_mapper::RsFmtRoute,
) -> Vec<CheckResult> {
    let tree = surface;
    let facts = collect(tree, route);
    let mut results = Vec::new();

    let root = RustfmtRootInput::from_facts(&facts);
    crate::rs_fmt_01_exists::check(&root, &mut results);
    run_content_checks(&root, &mut results);
    crate::rs_fmt_07_ignore_escape_hatch::check(&root, &mut results);

    for config_rel in &facts.extra_config_rels {
        let input = RustfmtExtraConfigInput {
            config_rel: config_rel.clone(),
            config_kind: file_name_kind(config_rel),
        };
        crate::rs_fmt_05_per_crate_override::check(&input, &mut results);
    }

    for dir_rel in &facts.dual_file_conflict_dirs {
        let input = RustfmtDualConflictInput {
            dir_rel: dir_rel.clone(),
        };
        crate::rs_fmt_08_dual_file_conflict::check(&input, &mut results);
    }

    results
}

fn run_content_checks(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    let Some(config_rel) = input.config_rel.as_deref() else {
        return;
    };
    let Some(rustfmt) = input.parsed.as_ref() else {
        if input.parse_error.is_some() {
            results.push(CheckResult::from_parts(
                "RS-FMT-CONFIG-01".to_owned(),
                Severity::Error,
                "rustfmt config parse error".to_owned(),
                "rustfmt config exists but could not be parsed as a TOML table".to_owned(),
                Some(config_rel.to_owned()),
                None,
                false,
            ));
        }
        return;
    };

    if input.cargo.is_none() && rustfmt.edition.is_some() {
        push_cargo_blocker(input, results);
    }
    if input.toolchain.is_none() && uses_nightly_rustfmt_keys(rustfmt) {
        push_toolchain_blocker(input, results);
    }

    let (Some(cargo), Some(toolchain)) = (input.cargo.clone(), input.toolchain.clone()) else {
        return;
    };

    let package_input = G3RsFmtConfigChecksInput {
        rustfmt_rel_path: config_rel.to_owned(),
        rustfmt: rustfmt.clone(),
        cargo_rel_path: input.cargo_rel_path.clone(),
        cargo,
        toolchain_rel_path: input.toolchain_rel_path.clone(),
        toolchain,
    };
    let package_results = g3rs_fmt_config_checks::check(&package_input);
    results.extend(package_results.into_iter().map(convert_check_result));
}

fn push_cargo_blocker(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    let (title, message) = match input.cargo_parse_error.as_deref() {
        Some(_) => (
            "Cargo.toml parse error",
            "rustfmt edition checks require a parseable root Cargo.toml.".to_owned(),
        ),
        None => (
            "Cargo.toml missing",
            "rustfmt edition checks require a root Cargo.toml with workspace or package edition."
                .to_owned(),
        ),
    };
    results.push(CheckResult::from_parts(
        "RS-FMT-CONFIG-04".to_owned(),
        Severity::Error,
        title.to_owned(),
        message,
        Some(input.cargo_rel_path.clone()),
        None,
        false,
    ));
}

fn push_toolchain_blocker(input: &RustfmtRootInput, results: &mut Vec<CheckResult>) {
    let (title, message) = match input.toolchain_parse_error.as_deref() {
        Some(_) => (
            "rust-toolchain.toml parse error",
            "Nightly-only rustfmt settings require a parseable root rust-toolchain.toml."
                .to_owned(),
        ),
        None => (
            "rust-toolchain.toml missing",
            "Nightly-only rustfmt settings require a root rust-toolchain.toml to verify the channel."
                .to_owned(),
        ),
    };
    results.push(CheckResult::from_parts(
        "RS-FMT-CONFIG-03".to_owned(),
        Severity::Error,
        title.to_owned(),
        message,
        Some(input.toolchain_rel_path.clone()),
        None,
        false,
    ));
}

fn uses_nightly_rustfmt_keys(rustfmt: &rustfmt_toml_parser::RustfmtToml) -> bool {
    let table = toml::Value::try_from(rustfmt.clone())
        .expect("typed RustfmtToml should serialize")
        .as_table()
        .cloned()
        .expect("typed RustfmtToml should serialize as table");
    [
        "group_imports",
        "imports_granularity",
        "format_code_in_doc_comments",
        "format_strings",
        "overflow_delimited_expr",
        "normalize_comments",
        "normalize_doc_attributes",
        "wrap_comments",
        "format_macro_matchers",
        "format_macro_bodies",
        "condense_wildcard_suffixes",
    ]
    .into_iter()
    .any(|key| table.contains_key(key))
}

fn convert_check_result(result: G3CheckResult) -> CheckResult {
    CheckResult::from_parts(
        result.id().to_owned(),
        convert_severity(result.severity()),
        result.title().to_owned(),
        result.message().to_owned(),
        result.file().map(str::to_owned),
        result.line(),
        result.inventory(),
    )
}

fn convert_severity(severity: G3Severity) -> Severity {
    match severity {
        G3Severity::Error => Severity::Error,
        G3Severity::Warn => Severity::Warn,
        G3Severity::Info => Severity::Info,
    }
}
