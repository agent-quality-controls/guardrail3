use super::super::check_source;
use guardrail3_app_rs_family_code_assertions::rs_code_21_fs_glob_import::{
    RuleFinding, assert_findings,
};

#[test]
fn errors_on_direct_std_fs_glob() {
    let results = check_source("src/foo.rs", "use std::fs::*;\nfn main() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_grouped_std_fs_glob_import() {
    let results = check_source("src/foo.rs", "use std::{fs::*, io};\nfn main() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_public_std_fs_glob_import() {
    let results = check_source("src/lib.rs", "pub use std::fs::*;\nfn main() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/lib.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_fs_glob_inside_group_with_self() {
    let results = check_source("src/foo.rs", "use std::fs::{self, *};\nfn main() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_fs_glob_inside_group_with_named() {
    let results = check_source("src/foo.rs", "use std::fs::{File, *};\nfn main() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_alias_plus_glob_inside_group() {
    let results = check_source(
        "src/foo.rs",
        "use std::fs::{File as Alias, *};\nfn main() {}",
        false,
    );

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_nested_grouped_fs_glob_import() {
    let results = check_source("src/foo.rs", "use std::fs::{{*}};\nfn main() {}", false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(1),
            false,
        )],
    );
}

#[test]
fn errors_on_glob_inside_inline_module() {
    let content = "mod internal {\n    use std::fs::*;\n    pub fn read_it() {}\n}\nfn main() {}";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_grouped_glob_inside_inline_module() {
    let content =
        "mod internal {\n    use std::{fs::*, io};\n    pub fn read_it() {}\n}\nfn main() {}";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_function_local_glob_import() {
    let content =
        "fn main() {\n    use std::fs::*;\n    let _ = read_to_string(\"Cargo.toml\");\n}";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_extern_crate_std_alias_glob_import() {
    let content = "extern crate std as s;\nuse s::fs::*;\nfn main() {}";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn errors_on_use_std_alias_then_glob_import() {
    let content = "use std as s;\nuse s::fs::*;\nfn main() {}";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[RuleFinding::new(
            guardrail3_domain_report::Severity::Error,
            "std::fs glob import",
            "Direct `use std::fs::*` glob import bypasses clippy method bans.",
            Some("src/foo.rs"),
            Some(2),
            false,
        )],
    );
}

#[test]
fn multiple_glob_imports_in_same_file() {
    let content = "use std::fs::*;\nmod helpers {\n    use std::fs::{self, *};\n}\nfn main() {}";
    let results = check_source("src/foo.rs", content, false);

    assert_findings(
        &results,
        &[
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "std::fs glob import",
                "Direct `use std::fs::*` glob import bypasses clippy method bans.",
                Some("src/foo.rs"),
                Some(1),
                false,
            ),
            RuleFinding::new(
                guardrail3_domain_report::Severity::Error,
                "std::fs glob import",
                "Direct `use std::fs::*` glob import bypasses clippy method bans.",
                Some("src/foo.rs"),
                Some(3),
                false,
            ),
        ],
    );
}
