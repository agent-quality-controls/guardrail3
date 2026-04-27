pub fn assert_missing_graph_section(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_config_checks_assertions::advisories::graph_no_default_features::rule::assert_findings(
        results,
        &[g3rs_deny_config_checks_assertions::advisories::graph_no_default_features::rule::error(
            "[graph] section missing",
            "`deny.toml` must contain `[graph]` coverage settings.",
            "deny.toml",
            false,
        )],
    );
}

pub fn assert_missing_deny_config(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[
            g3rs_deny_filetree_checks_assertions::coverage::error_no_file(
                "workspace root uncovered by deny config",
                "workspace root `.` is not covered by any allowed deny config.",
                false,
            ),
        ],
    );
}

pub fn assert_selected_root_deny_config(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[g3rs_deny_filetree_checks_assertions::coverage::info(
            "workspace root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        )],
    );
    g3rs_deny_filetree_checks_assertions::shadowing::assert_no_findings(results);
}

pub fn assert_same_root_conflicts(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[g3rs_deny_filetree_checks_assertions::coverage::info(
            "workspace root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        )],
    );
    g3rs_deny_filetree_checks_assertions::shadowing::assert_findings(
        results,
        &[g3rs_deny_filetree_checks_assertions::shadowing::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .cargo/deny.toml, .deny.toml, deny.toml.",
            ".cargo/deny.toml",
            false,
        )],
    );
}

pub fn assert_selected_deny_parse_failures(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[
            g3rs_deny_filetree_checks_assertions::coverage::error(
                "deny input failure",
                "Failed to parse root deny config `deny.toml` for deny checks: invalid deny.toml: TOML parse error at line 1, column 12\n  |\n1 | [advisories\n  |            ^\ninvalid table header\nexpected `.`, `]`\n",
                "deny.toml",
                false,
            ),
            g3rs_deny_filetree_checks_assertions::coverage::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

pub fn assert_rust_policy_parse_failures(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[
            g3rs_deny_filetree_checks_assertions::coverage::error(
                "deny rust policy is not parseable",
                "Failed to parse root Rust policy `guardrail3-rs.toml` for deny profile resolution: invalid guardrail3-rs.toml: TOML parse error at line 1, column 11\n  |\n1 | profile = \"invalid\"\n  |           ^^^^^^^^^\nunknown variant `invalid`, expected `service` or `library`\n",
                "guardrail3-rs.toml",
                false,
            ),
            g3rs_deny_filetree_checks_assertions::coverage::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

pub fn assert_unreadable_selected_deny_file(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[
            g3rs_deny_filetree_checks_assertions::coverage::error(
                "deny input failure",
                "Failed to read root deny config `deny.toml` for deny checks: file is not readable",
                "deny.toml",
                false,
            ),
            g3rs_deny_filetree_checks_assertions::coverage::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

pub fn assert_unreadable_rust_policy(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[
            g3rs_deny_filetree_checks_assertions::coverage::error(
                "deny rust policy is not parseable",
                "Failed to parse root Rust policy `guardrail3-rs.toml` for deny profile resolution: file is not readable",
                "guardrail3-rs.toml",
                false,
            ),
            g3rs_deny_filetree_checks_assertions::coverage::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}

pub fn assert_shadowed_root_parse_failures(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[
            g3rs_deny_filetree_checks_assertions::coverage::error(
                "deny input failure",
                "Failed to parse root deny config `.deny.toml` for deny checks: invalid deny.toml: TOML parse error at line 1, column 12\n  |\n1 | [advisories\n  |            ^\ninvalid table header\nexpected `.`, `]`\n",
                ".deny.toml",
                false,
            ),
            g3rs_deny_filetree_checks_assertions::coverage::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
    g3rs_deny_filetree_checks_assertions::shadowing::assert_findings(
        results,
        &[g3rs_deny_filetree_checks_assertions::shadowing::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .deny.toml, deny.toml.",
            ".deny.toml",
            false,
        )],
    );
}

pub fn assert_shadowed_root_unreadable_failures(results: &[guardrail3_check_types::G3CheckResult]) {
    g3rs_deny_filetree_checks_assertions::coverage::assert_findings(
        results,
        &[
            g3rs_deny_filetree_checks_assertions::coverage::error(
                "deny input failure",
                "Failed to read root deny config `.deny.toml` for deny checks: file is not readable",
                ".deny.toml",
                false,
            ),
            g3rs_deny_filetree_checks_assertions::coverage::info(
                "workspace root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
    g3rs_deny_filetree_checks_assertions::shadowing::assert_findings(
        results,
        &[g3rs_deny_filetree_checks_assertions::shadowing::error(
            "multiple deny configs at one policy root",
            "`.` has multiple accepted deny configs: .deny.toml, deny.toml.",
            ".deny.toml",
            false,
        )],
    );
}
