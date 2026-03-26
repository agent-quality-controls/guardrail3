use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;
use guardrail3_domain_report::Severity;
use crate::test_support::{copy_fixture, write_file};

#[test]
fn nested_file_and_inline_modules_still_count_toward_impl_heaviness() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        r#"
pub mod api;
mod internal;

pub trait RepoPort {
    fn fetch(&self);
}

pub(crate) trait HiddenRepoPort {
    fn hidden(&self);
}

pub mod inline_ports {
    pub trait InlinePort {
        fn list(&self);
    }

    mod detail {
        struct Detail;

        impl Detail {
            fn new() -> Self {
                Self
            }
        }
    }
}
"#,
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/api.rs",
        r#"
pub trait ApiPort {
    fn read(&self);
}

pub struct ApiAdapter;

impl ApiAdapter {
    pub fn new() -> Self {
        Self
    }
}
"#,
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/internal.rs",
        r#"
struct InternalWorker;

impl InternalWorker {
    fn new() -> Self {
        Self
    }
}

impl InternalWorker {
    fn close(&self) {}
}
"#,
    );

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();

    assert_eq!(
        warnings.len(),
        1,
        "expected one impl-heavy warning for the split ports crate: {warnings:#?}"
    );
    assert_eq!(warnings[0].severity, Severity::Warn);
    assert_eq!(
        warnings[0].file.as_deref(),
        Some("apps/backend/crates/ports/outbound/repo")
    );
    assert!(warnings[0].message.contains(
        "Ports crate `backend-ports-outbound-repo` has 4 impl blocks and 3 public traits"
    ));
}

#[test]
fn trait_only_split_modules_with_pub_crate_traits_stay_clean() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/lib.rs",
        r#"
pub mod api;
mod internal;

pub trait RepoPort {
    fn fetch(&self);
}

pub(crate) trait HiddenRepoPort {
    fn hidden(&self);
}

pub mod inline_ports {
    pub trait InlinePort {
        fn list(&self);
    }
}
"#,
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/api.rs",
        r#"
pub trait ApiPort {
    fn read(&self);
}

pub struct ApiDto {
    pub id: String,
}
"#,
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/ports/outbound/repo/src/internal.rs",
        r#"
struct InternalWorker;

pub struct InternalDto {
    pub label: String,
}
"#,
    );

    let results = assertions::run_family(tmp.path());
    let warnings: Vec<_> = results
        .iter()
        .filter(|result| result.id == "RS-HEXARCH-22")
        .collect();

    assert!(
        warnings.is_empty(),
        "expected multi-module DTO-only ports layout to stay clean, got: {warnings:#?}"
    );
}
