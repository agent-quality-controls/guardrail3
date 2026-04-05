use g3_deny_content_checks_assertions::rs_deny_14_license_allow_baseline as assertions;

use super::helpers::run_check;

#[test]
fn unexpected_license_produces_error() {
    let results = run_check(
        r#"
[licenses]
allow = [
    "MIT",
    "Apache-2.0",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
    "Unicode-3.0",
    "Zlib",
    "CC0-1.0",
    "OpenSSL",
    "BSL-1.0",
    "MPL-2.0",
    "CDLA-Permissive-2.0",
    "WTFPL",
]
confidence-threshold = 0.8

[licenses.private]
ignore = true
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "unexpected allowed license",
            "`deny.toml` allows unexpected license `WTFPL`.",
            "deny.toml",
            false,
        )],
    );
}
