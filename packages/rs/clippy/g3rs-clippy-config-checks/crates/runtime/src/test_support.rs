use clippy_toml_parser::parse as parse_clippy_toml;
use g3rs_clippy_types::{
    G3RsClippyCargoConfigOverride, G3RsClippyConfigChecksInput, G3RsClippyConfigState,
    G3RsClippyRustPolicyState,
};
use guardrail3_rs_toml_parser::RustProfile;
use guardrail3_check_types::{G3CheckResult, G3Severity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Finding {
    pub(crate) id: String,
    pub(crate) severity: G3Severity,
    pub(crate) title: String,
    pub(crate) message: String,
    pub(crate) file: Option<String>,
    pub(crate) inventory: bool,
}

pub(crate) fn findings(results: &[G3CheckResult]) -> Vec<Finding> {
    results
        .iter()
        .map(|result| Finding {
            id: result.id().to_owned(),
            severity: result.severity(),
            title: result.title().to_owned(),
            message: result.message().to_owned(),
            file: result.file().map(str::to_owned),
            inventory: result.inventory(),
        })
        .collect()
}

pub(crate) fn findings_for(results: &[G3CheckResult], id: &str) -> Vec<Finding> {
    findings(results)
        .into_iter()
        .filter(|finding| finding.id == id)
        .collect()
}

pub(crate) fn assert_findings_for(results: &[G3CheckResult], id: &str, expected: &[Finding]) {
    assert_eq!(findings_for(results, id), expected);
}

pub(crate) fn finding_for(
    id: &str,
    severity: G3Severity,
    title: &str,
    message: &str,
    file: &str,
    inventory: bool,
) -> Finding {
    Finding {
        id: id.to_owned(),
        severity,
        title: title.to_owned(),
        message: message.to_owned(),
        file: Some(file.to_owned()),
        inventory,
    }
}

pub(crate) fn error_for(id: &str, title: &str, message: &str, file: &str, inventory: bool) -> Finding {
    finding_for(id, G3Severity::Error, title, message, file, inventory)
}

pub(crate) fn warn_for(id: &str, title: &str, message: &str, file: &str, inventory: bool) -> Finding {
    finding_for(id, G3Severity::Warn, title, message, file, inventory)
}

pub(crate) fn info_for(id: &str, title: &str, message: &str, file: &str, inventory: bool) -> Finding {
    finding_for(id, G3Severity::Info, title, message, file, inventory)
}

pub(crate) fn baseline_toml(profile: RustProfile, garde_enabled: bool) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "max-struct-bools = {}\n",
        crate::baseline::MAX_STRUCT_BOOLS
    ));
    out.push_str(&format!(
        "max-fn-params-bools = {}\n",
        crate::baseline::MAX_FN_PARAMS_BOOLS
    ));
    out.push_str(&format!(
        "too-many-lines-threshold = {}\n",
        crate::baseline::TOO_MANY_LINES_THRESHOLD
    ));
    out.push_str(&format!(
        "too-many-arguments-threshold = {}\n",
        crate::baseline::TOO_MANY_ARGUMENTS_THRESHOLD
    ));
    out.push_str(&format!(
        "excessive-nesting-threshold = {}\n",
        crate::baseline::EXCESSIVE_NESTING_THRESHOLD
    ));
    out.push_str(&format!(
        "cognitive-complexity-threshold = {}\n",
        crate::baseline::COGNITIVE_COMPLEXITY_THRESHOLD
    ));
    out.push_str(&format!(
        "type-complexity-threshold = {}\n",
        crate::baseline::TYPE_COMPLEXITY_THRESHOLD
    ));
    out.push_str(&format!(
        "avoid-breaking-exported-api = false\nallow-dbg-in-tests = {}\nallow-expect-in-tests = {}\nallow-panic-in-tests = {}\nallow-print-in-tests = {}\nallow-unwrap-in-tests = {}\n",
        crate::baseline::ALLOW_DBG_IN_TESTS,
        crate::baseline::ALLOW_EXPECT_IN_TESTS,
        crate::baseline::ALLOW_PANIC_IN_TESTS,
        crate::baseline::ALLOW_PRINT_IN_TESTS,
        crate::baseline::ALLOW_UNWRAP_IN_TESTS,
    ));

    out.push_str("disallowed-methods = [\n");
    for path in crate::support::expected_method_bans(garde_enabled) {
        out.push_str(&format!("  {{ path = \"{path}\", reason = \"baseline\" }},\n"));
    }
    out.push_str("]\n");

    out.push_str("disallowed-types = [\n");
    for path in crate::support::expected_type_bans(Some(profile), garde_enabled) {
        out.push_str(&format!("  {{ path = \"{path}\", reason = \"baseline\" }},\n"));
    }
    out.push_str("]\n");

    out.push_str("disallowed-macros = [\n");
    for path in crate::baseline::EXPECTED_MACRO_BANS {
        out.push_str(&format!("  {{ path = \"{path}\", reason = \"baseline\" }},\n"));
    }
    out.push_str("]\n");
    out
}

pub(crate) fn parsed_rust_policy(
    rel_path: &str,
    profile: Option<RustProfile>,
    garde_enabled: bool,
) -> G3RsClippyRustPolicyState {
    G3RsClippyRustPolicyState::Parsed {
        rel_path: rel_path.to_owned(),
        profile,
        garde_enabled,
    }
}

pub(crate) fn parse_error_rust_policy(rel_path: &str, reason: &str) -> G3RsClippyRustPolicyState {
    G3RsClippyRustPolicyState::ParseError {
        rel_path: rel_path.to_owned(),
        reason: reason.to_owned(),
    }
}

pub(crate) fn override_facts(
    rel_path: &str,
    parse_error: Option<&str>,
) -> G3RsClippyCargoConfigOverride {
    G3RsClippyCargoConfigOverride {
        rel_path: rel_path.to_owned(),
        parse_error: parse_error.map(str::to_owned),
    }
}

pub(crate) fn input_from_raw(rel_path: &str, raw: &str) -> G3RsClippyConfigChecksInput {
    input_with_raw(
        rel_path,
        raw,
        G3RsClippyRustPolicyState::Missing,
        false,
        Vec::new(),
    )
}

pub(crate) fn input_with_raw(
    rel_path: &str,
    raw: &str,
    rust_policy: G3RsClippyRustPolicyState,
    published_library_policy: bool,
    cargo_config_overrides: Vec<G3RsClippyCargoConfigOverride>,
) -> G3RsClippyConfigChecksInput {
    let clippy = match toml::from_str::<toml::Value>(raw) {
        Ok(raw_value) => G3RsClippyConfigState::Parsed {
            raw: raw_value,
            typed: parse_clippy_toml(raw).map_err(|err| err.to_string()),
        },
        Err(err) => G3RsClippyConfigState::ParseError {
            reason: err.to_string(),
        },
    };

    G3RsClippyConfigChecksInput {
        clippy_rel_path: rel_path.to_owned(),
        clippy,
        rust_policy,
        published_library_policy,
        cargo_config_overrides,
    }
}
