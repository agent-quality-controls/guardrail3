use g3rs_code_source_checks_assertions::rs_code_ast_31_public_struct_named_fields::rule::{
    ExpectedRuleResult, G3Severity, assert_rule_results,
};

#[test]
fn shared_plain_record_struct_is_allowed() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub struct Input { pub rel_path: String, pub profile: Option<String> }",
        false,
        true,
    );

    assert_rule_results(&results, &[]);
}

#[test]
fn shared_struct_with_inherent_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\nimpl Input { pub fn validate(&self) -> bool { self.mode } }",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(1),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_inherent_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    impl Input { pub fn validate(&self) -> bool { self.mode } }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_crate_qualified_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    impl crate::api::Input { pub fn validate(&self) -> bool { self.mode } }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_self_qualified_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    impl self::Input { pub fn validate(&self) -> bool { self.mode } }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_super_qualified_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    mod nested {\n        impl super::Input { pub fn validate(&self) -> bool { self.mode } }\n    }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_super_super_qualified_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    pub mod nested {\n        pub mod deeper {\n            impl super::super::Input { pub fn validate(&self) -> bool { self.mode } }\n        }\n    }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_super_imported_name_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    mod nested {\n        use super::Input;\n        impl Input { pub fn validate(&self) -> bool { self.mode } }\n    }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_super_aliased_name_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    mod nested {\n        use super::Input as Alias;\n        impl Alias { pub fn validate(&self) -> bool { self.mode } }\n    }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_chained_alias_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    mod nested {\n        use super::Input as Alias;\n        use Alias as Alias2;\n        impl Alias2 { pub fn validate(&self) -> bool { self.mode } }\n    }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_super_glob_import_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    mod nested {\n        use super::*;\n        impl Input { pub fn validate(&self) -> bool { self.mode } }\n    }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_crate_glob_import_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    mod nested {\n        use crate::api::*;\n        impl Input { pub fn validate(&self) -> bool { self.mode } }\n    }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_nested_public_struct_with_absolute_crate_qualified_impl_still_errors() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub mod api {\n    pub struct Input { pub rel_path: String, pub profile: Option<String>, pub raw: String, pub flags: usize, pub mode: bool }\n    mod nested {\n        impl ::crate::api::Input { pub fn validate(&self) -> bool { self.mode } }\n    }\n}",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Error),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 5 named `pub` fields and also defines inherent methods. Keep shared public fields only on plain data structs. Make the fields private or move the behavior out, so shared crates stay as transport data instead of mixing data and API logic.",
            ),
            line: Some(2),
        }],
    );
}

#[test]
fn shared_struct_with_mixed_visibility_still_warns() {
    let results = super::super::check_source_with_shared(
        "src/types.rs",
        "pub struct Input { pub rel_path: String, profile: Option<String> }",
        false,
        true,
    );

    assert_rule_results(
        &results,
        &[ExpectedRuleResult {
            severity: Some(G3Severity::Warn),
            title: Some("public struct exposes named public fields"),
            file: Some("src/types.rs"),
            inventory: Some(false),
            message: Some(
                "Shared-crate struct `Input` exposes 1 named `pub` fields but also hides some fields. In shared crates, either make this a plain data struct with all fields `pub`, or make the fields private and expose an API. Mixed visibility hides part of the shared data contract.",
            ),
            line: Some(1),
        }],
    );
}
