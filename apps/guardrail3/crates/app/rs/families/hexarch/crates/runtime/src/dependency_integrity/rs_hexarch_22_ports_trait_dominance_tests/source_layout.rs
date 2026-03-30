use super::{copy_fixture, write_file};
use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;

#[test]
fn public_inherent_methods_inside_trait_modules_warn() {
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

    let results = super::run_family(tmp.path());
    assertions::assert_warning_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/ports/outbound/repo"],
        Some(Some("apps/backend/crates/ports/outbound/repo")),
        Some("public inherent method"),
        None,
        &[],
    );
}

#[test]
fn trait_only_split_modules_with_passive_types_stay_clean() {
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

impl std::fmt::Display for ApiDto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
