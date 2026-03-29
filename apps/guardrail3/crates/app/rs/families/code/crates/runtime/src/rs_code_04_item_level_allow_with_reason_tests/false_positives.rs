use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_04_item_level_allow_with_reason::{Severity, 
    RuleFinding, assert_findings, assert_no_hits,
};
use test_support::write_file;

#[test]
fn skips_undocumented_and_malformed_reason_near_misses() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let undocumented_rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let empty_reason_rel = "apps/worker/crates/ports/outbound/queue/src/lib.rs";
    let wrong_key_rel = "apps/devctl/crates/ports/outbound/traits/src/lib.rs";
    let block_comment_rel = "apps/backend/crates/adapters/outbound/queue/src/lib.rs";

    let undocumented_content = test_support::read_file(root, undocumented_rel);
    let empty_reason_content = test_support::read_file(root, empty_reason_rel);
    let wrong_key_content = test_support::read_file(root, wrong_key_rel);
    let block_comment_content = test_support::read_file(root, block_comment_rel);

    write_file(
        root,
        undocumented_rel,
        &format!(
            "{undocumented_content}\n#[allow(clippy::unwrap_used)]\npub fn undocumented_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        empty_reason_rel,
        &format!(
            "{empty_reason_content}\n#[allow(clippy::panic)] // reason:\npub fn empty_reason_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        wrong_key_rel,
        &format!(
            "{wrong_key_content}\n#[allow(clippy::expect_used)] // because: temporary seam\npub fn wrong_key_probe() {{}}\n"
        ),
    );
    write_file(
        root,
        block_comment_rel,
        &format!(
            "{block_comment_content}\n#[allow(clippy::unwrap_used)] /* reason: block comment */\npub fn block_comment_probe() {{}}\n"
        ),
    );

    let results = run_family(root);

    assert_no_hits(&results);
}

#[test]
fn inventories_accepted_reason_variants_and_other_item_kinds() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let upper_rel = "apps/backend/crates/ports/outbound/repo/src/lib.rs";
    let tight_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";
    let mod_rel = "apps/devctl/crates/app/core/src/lib.rs";

    let upper_content = test_support::read_file(root, upper_rel);
    let tight_content = test_support::read_file(root, tight_rel);
    let mod_content = test_support::read_file(root, mod_rel);

    let upper_new = format!(
        "{upper_content}\n#[allow(clippy::unwrap_used)] // REASON: uppercase accepted\npub fn uppercase_reason_probe() {{}}\n"
    );
    let tight_new = format!(
        "{tight_content}\n#[allow(clippy::panic)] //reason: tight spacing accepted\npub fn tight_reason_probe() {{}}\n"
    );
    let mod_new = format!(
        "{mod_content}\n#[allow(clippy::expect_used)] // reason: module boundary shim\npub mod documented_module_probe {{\n    pub fn helper() {{}}\n}}\n"
    );

    write_file(root, upper_rel, &upper_new);
    write_file(root, tight_rel, &tight_new);
    write_file(root, mod_rel, &mod_new);

    let upper_line = upper_new
        .lines()
        .position(|line| {
            line.contains("#[allow(clippy::unwrap_used)] // REASON: uppercase accepted")
        })
        .map(|index| index + 1)
        .unwrap_or_default();
    let tight_line = tight_new
        .lines()
        .position(|line| line.contains("#[allow(clippy::panic)] //reason: tight spacing accepted"))
        .map(|index| index + 1)
        .unwrap_or_default();
    let mod_line = mod_new
        .lines()
        .position(|line| {
            line.contains("#[allow(clippy::expect_used)] // reason: module boundary shim")
        })
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_findings(
        &run_family(root),
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::unwrap_used)] reason: uppercase accepted",
                file: Some(upper_rel),
                line: Some(upper_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::expect_used)] reason: module boundary shim",
                file: Some(mod_rel),
                line: Some(mod_line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::panic)] reason: tight spacing accepted",
                file: Some(tight_rel),
                line: Some(tight_line),
                inventory: true,
            },
        ],
    );
}

#[test]
fn inventories_only_documented_allows_in_mixed_same_file_case() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let content = test_support::read_file(root, rel);

    let new_content = format!(
        "{content}\n#[allow(clippy::unwrap_used)] // reason: documented surface\npub fn documented_probe() {{}}\n#[allow(clippy::expect_used)]\npub fn undocumented_probe() {{}}\n"
    );
    write_file(root, rel, &new_content);

    let documented_line = new_content
        .lines()
        .position(|line| {
            line.contains("#[allow(clippy::unwrap_used)] // reason: documented surface")
        })
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_findings(
        &run_family(root),
        &[RuleFinding {
            severity: Severity::Info,
            title: "item-level allow with reason",
            message: "#[allow(clippy::unwrap_used)] reason: documented surface",
            file: Some(rel),
            line: Some(documented_line),
            inventory: true,
        }],
    );
}

#[test]
fn preserves_reason_text_with_url_like_content() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let content = test_support::read_file(root, rel);

    let new_content = format!(
        "{content}\n#[allow(clippy::unwrap_used)] // reason: compatibility note see https://example.com/policy\npub fn documented_probe() {{}}\n"
    );
    write_file(root, rel, &new_content);

    let line = new_content
        .lines()
        .position(|source| {
            source.contains(
                "#[allow(clippy::unwrap_used)] // reason: compatibility note see https://example.com/policy",
            )
        })
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_findings(
        &run_family(root),
        &[RuleFinding {
            severity: Severity::Info,
            title: "item-level allow with reason",
            message: "#[allow(clippy::unwrap_used)] reason: compatibility note see https://example.com/policy",
            file: Some(rel),
            line: Some(line),
            inventory: true,
        }],
    );
}

#[test]
fn inventories_multiline_allow_with_reason_on_closing_line() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let rel = "apps/backend/crates/ports/inbound/api/src/lib.rs";
    let content = test_support::read_file(root, rel);

    let new_content = format!(
        "{content}\n#[allow(\n    clippy::unwrap_used,\n    clippy::expect_used\n)] // reason: multiline adapter seam\npub fn documented_probe() {{}}\n"
    );
    write_file(root, rel, &new_content);

    let line = new_content
        .lines()
        .position(|source| source.contains(")] // reason: multiline adapter seam"))
        .map(|index| index + 1)
        .unwrap_or_default();

    assert_findings(
        &run_family(root),
        &[
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::expect_used)] reason: multiline adapter seam",
                file: Some(rel),
                line: Some(line),
                inventory: true,
            },
            RuleFinding {
                severity: Severity::Info,
                title: "item-level allow with reason",
                message: "#[allow(clippy::unwrap_used)] reason: multiline adapter seam",
                file: Some(rel),
                line: Some(line),
                inventory: true,
            },
        ],
    );
}
