pub fn assert_missing_root(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::rs_fmt_filetree_01_exists::rule::assert_findings(
        results,
        &[crate::rs_fmt_filetree_01_exists::rule::error(
            "rustfmt config missing",
            "Expected `rustfmt.toml` at workspace root. Create one with the required formatting settings.",
            "rustfmt.toml",
            false,
        )],
    );
    crate::rs_fmt_filetree_05_per_crate_override::rule::assert_no_findings(results);
    crate::rs_fmt_filetree_08_dual_file_conflict::rule::assert_no_findings(results);
}

pub fn assert_root_conflict(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::rs_fmt_filetree_01_exists::rule::assert_no_findings(results);
    crate::rs_fmt_filetree_05_per_crate_override::rule::assert_no_findings(results);
    crate::rs_fmt_filetree_08_dual_file_conflict::rule::assert_findings(
        results,
        &[crate::rs_fmt_filetree_08_dual_file_conflict::rule::warn(
            "Conflicting rustfmt config files",
            "Both `rustfmt.toml` and `.rustfmt.toml` exist in `.`. Delete `.rustfmt.toml` and keep `rustfmt.toml`.",
            "rustfmt.toml",
            false,
        )],
    );
}

pub fn assert_nested_override(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::rs_fmt_filetree_01_exists::rule::assert_no_findings(results);
    crate::rs_fmt_filetree_05_per_crate_override::rule::assert_findings(
        results,
        &[crate::rs_fmt_filetree_05_per_crate_override::rule::error(
            "Illegal nested rustfmt config",
            "`rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.",
            "crates/api/rustfmt.toml",
            false,
        )],
    );
    crate::rs_fmt_filetree_08_dual_file_conflict::rule::assert_no_findings(results);
}

pub fn assert_combined_all_filetree_findings(results: &[guardrail3_check_types::G3CheckResult]) {
    crate::rs_fmt_filetree_01_exists::rule::assert_findings(
        results,
        &[crate::rs_fmt_filetree_01_exists::rule::error(
            "rustfmt config missing",
            "Expected `rustfmt.toml` at workspace root. Create one with the required formatting settings.",
            "rustfmt.toml",
            false,
        )],
    );
    crate::rs_fmt_filetree_05_per_crate_override::rule::assert_findings(
        results,
        &[crate::rs_fmt_filetree_05_per_crate_override::rule::error(
            "Illegal nested rustfmt config",
            "`rustfmt.toml` below repository root is forbidden; rustfmt policy is root-only. Delete this file and ensure all formatting settings are in the root `rustfmt.toml`.",
            "crates/api/rustfmt.toml",
            false,
        )],
    );
    crate::rs_fmt_filetree_08_dual_file_conflict::rule::assert_findings(
        results,
        &[crate::rs_fmt_filetree_08_dual_file_conflict::rule::warn(
            "Conflicting rustfmt config files",
            "Both `rustfmt.toml` and `.rustfmt.toml` exist in `.`. Delete `.rustfmt.toml` and keep `rustfmt.toml`.",
            "rustfmt.toml",
            false,
        )],
    );
}
