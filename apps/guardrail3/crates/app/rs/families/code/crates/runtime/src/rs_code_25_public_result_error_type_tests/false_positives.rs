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
