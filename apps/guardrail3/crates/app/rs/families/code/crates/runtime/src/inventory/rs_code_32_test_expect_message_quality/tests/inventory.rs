use super::helpers::copy_fixture;
use super::helpers::run_family;
use guardrail3_app_rs_family_code_assertions::inventory::rs_code_32_test_expect_message_quality::{
    RuleFinding, Severity, assert_findings,
};
use test_support::{line_number, read_file, write_file};

#[test]
fn attacks_weak_test_expect_messages_across_owned_test_surfaces() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let test_rel = "apps/backend/tests/weak_expect_probe.rs";
    let cfg_rel = "apps/backend/crates/adapters/inbound/rest/src/lib.rs";

    write_file(
        root,
        test_rel,
        "#[test]\nfn weak_expect_probe() {\n    let _ = Some(1).expect(\"ok\");\n}\n",
    );

    let cfg_content = read_file(root, cfg_rel);
    write_file(
        root,
        cfg_rel,
        &format!(
            "{cfg_content}\n#[cfg(test)]\nmod expect_probe {{\n    #[test]\n    fn parse_probe() {{\n        let message = \"backend fixture should parse\";\n        let _ = Some(1).expect(message);\n    }}\n}}\n"
        ),
    );

    let test_line = line_number(&read_file(root, test_rel), "expect(\"ok\")");
    let cfg_line = line_number(&read_file(root, cfg_rel), "expect(message)");

    let results = run_family(root);
    assert_findings(
        &results,
        &[
            RuleFinding::new(
                Severity::Error,
                "test expect message must be literal",
                "Test `expect(...)` message must be a useful string literal or `concat!` of string literals, not an indirect expression: `let _ = Some(1).expect(message);`.",
                Some(cfg_rel),
                Some(cfg_line),
                false,
            ),
            RuleFinding::new(
                Severity::Error,
                "test expect message too weak",
                "Test `expect(...)` message must explain the failure clearly. Weak message `ok` found in `let _ = Some(1).expect(\"ok\");`.",
                Some(test_rel),
                Some(test_line),
                false,
            ),
        ],
    );
}
