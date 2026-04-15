use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Clone, Debug, PartialEq, Eq)]
struct Finding<'a> {
    id: &'a str,
    severity: G3Severity,
    title: &'a str,
    message: &'a str,
    file: Option<&'a str>,
    inventory: bool,
}

fn finding<'a>(
    id: &'a str,
    severity: G3Severity,
    title: &'a str,
    message: &'a str,
    file: &'a str,
    inventory: bool,
) -> Finding<'a> {
    Finding {
        id,
        severity,
        title,
        message,
        file: Some(file),
        inventory,
    }
}

fn findings<'a>(results: &'a [G3CheckResult]) -> Vec<Finding<'a>> {
    let mut findings = results
        .iter()
        .map(|result| Finding {
            id: result.id(),
            severity: result.severity(),
            title: result.title(),
            message: result.message(),
            file: result.file(),
            inventory: result.inventory(),
        })
        .collect::<Vec<_>>();
    findings.sort_by(|left, right| {
        (
            left.id,
            format!("{:?}", left.severity),
            left.title,
            left.message,
            left.file,
            left.inventory,
        )
            .cmp(&(
                right.id,
                format!("{:?}", right.severity),
                right.title,
                right.message,
                right.file,
                right.inventory,
            ))
    });
    findings
}

pub fn assert_missing_root(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![finding(
            "RS-FMT-FILETREE-01",
            G3Severity::Error,
            "rustfmt config missing",
            "Expected `rustfmt.toml` at workspace root. Create one with the required formatting settings.",
            "rustfmt.toml",
            false,
        )],
    );
}

pub fn assert_nested_override_and_dual_conflict(results: &[G3CheckResult]) {
    assert_eq!(
        findings(results),
        vec![
            finding(
                "RS-FMT-FILETREE-05",
                G3Severity::Error,
                "Illegal nested rustfmt config",
                "`.rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.",
                "crates/api/.rustfmt.toml",
                false,
            ),
            finding(
                "RS-FMT-FILETREE-05",
                G3Severity::Error,
                "Illegal nested rustfmt config",
                "`rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.",
                "crates/api/rustfmt.toml",
                false,
            ),
            finding(
                "RS-FMT-FILETREE-08",
                G3Severity::Warn,
                "Conflicting rustfmt config files",
                "Both `rustfmt.toml` and `.rustfmt.toml` exist in `crates/api`. Delete `.rustfmt.toml` and keep `rustfmt.toml`.",
                "crates/api/rustfmt.toml",
                false,
            ),
        ],
    );
}

pub fn assert_nightly_rustfmt_keys_on_stable(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-03"
                && result.title() == "nightly-only rustfmt setting `group_imports` on stable"
                && result.file() == Some("rustfmt.toml")
        }),
        "{results:#?}"
    );
}

pub fn assert_nightly_key_blocker_when_toolchain_is_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-03"
                && result.title() == "rust-toolchain.toml missing"
                && result.file() == Some("rust-toolchain.toml")
        }),
        "{results:#?}"
    );
}

pub fn assert_edition_blocker_when_cargo_is_missing(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-04"
                && result.title() == "Cargo.toml missing"
                && result.file() == Some("Cargo.toml")
        }),
        "{results:#?}"
    );
}

pub fn assert_rustfmt_parse_error(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-01"
                && result.title() == "rustfmt config parse error"
                && result.file() == Some("rustfmt.toml")
        }),
        "{results:#?}"
    );
}

pub fn assert_rustfmt_ignore_waiver(results: &[G3CheckResult]) {
    assert!(
        results.iter().any(|result| {
            result.id() == "RS-FMT-CONFIG-07"
                && result.title() == "rustfmt ignore waiver"
                && result.file() == Some("rustfmt.toml")
        }),
        "{results:#?}"
    );
}

pub fn assert_root_dot_rustfmt_toml_for_config_checks(results: &[G3CheckResult]) {
    assert_eq!(
        results
            .iter()
            .filter(|result| result.id() == "RS-FMT-CONFIG-03")
            .map(|result| (result.title().to_owned(), result.file().map(str::to_owned)))
            .collect::<Vec<_>>(),
        vec![(
            "nightly-only rustfmt setting `group_imports` on stable".to_owned(),
            Some(".rustfmt.toml".to_owned()),
        )],
        "{results:#?}"
    );
}

pub fn assert_keeps_config_01_active_when_cargo_is_parse_error(results: &[G3CheckResult]) {
    assert_eq!(
        results
            .iter()
            .map(|result| {
                (
                    result.id().to_owned(),
                    result.title().to_owned(),
                    result.file().map(str::to_owned),
                )
            })
            .collect::<Vec<_>>(),
        vec![
            (
                "RS-FMT-CONFIG-01".to_owned(),
                "rustfmt max_width wrong".to_owned(),
                Some("rustfmt.toml".to_owned()),
            ),
            (
                "RS-FMT-CONFIG-04".to_owned(),
                "Cargo.toml parse error".to_owned(),
                Some("Cargo.toml".to_owned()),
            ),
        ],
        "{results:#?}"
    );
}
