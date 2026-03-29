use super::super::check_source;
use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_25_public_result_error_type::assert_no_hits;
use test_support::write_file;

#[test]
fn skips_non_library_files_and_typed_public_errors() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";
    let worker_content = test_support::read_file(root, worker_rel);

    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\n\npub fn typed_job_probe() -> Result<Job, JobError> {{\n    Err(JobError::MissingTenantSlug)\n}}\n"
        ),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}

#[test]
fn skips_public_tokens_that_are_not_reachable_public_api() {
    let private_module_content = "mod internal { pub fn parse() -> Result<(), String> { Ok(()) } }";
    let private_type_content =
        "struct Hidden; impl Hidden { pub fn parse(&self) -> Result<(), String> { Ok(()) } }";
    let private_trait_content =
        "mod internal { pub trait Service { fn parse(&self) -> Result<(), String>; } }";
    let same_name_collision_content = "pub mod api { pub struct Service; }\nmod internal { struct Service; impl Service { pub fn parse(&self) -> Result<(), String> { Ok(()) } } }";

    assert_no_hits(&check_source("src/lib.rs", private_module_content, false));
    assert_no_hits(&check_source("src/lib.rs", private_type_content, false));
    assert_no_hits(&check_source("src/lib.rs", private_trait_content, false));
    assert_no_hits(&check_source(
        "src/lib.rs",
        same_name_collision_content,
        false,
    ));
}
