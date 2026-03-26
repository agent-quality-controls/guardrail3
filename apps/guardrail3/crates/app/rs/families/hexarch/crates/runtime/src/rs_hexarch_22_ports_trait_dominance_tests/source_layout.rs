use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_22_ports_trait_dominance as assertions;
use super::{copy_fixture, write_file};

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

    let results = super::run_family(tmp.path());
    let _warnings = assertions::warning_results(&results, "");

    assertions::assert_warning_summary(
        &results,
        "",
        1,
        &["apps/backend/crates/ports/outbound/repo"],
        Some(Some("apps/backend/crates/ports/outbound/repo")),
        Some("Ports crate `backend-ports-outbound-repo` has 4 impl blocks and 3 public traits"),
        None,
        &[],
    );
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

    let results = super::run_family(tmp.path());
    assertions::assert_no_error(&results, "");
}
