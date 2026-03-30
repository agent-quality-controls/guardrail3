use super::super::copy_fixture;
use super::super::run_family;
use guardrail3_app_rs_family_code_assertions::rs_code_26_lib_glob_reexport::assert_no_hits;
use test_support::write_file;

#[test]
fn skips_non_library_profiles_and_non_glob_reexports() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let worker_rel = "apps/worker/crates/domain/jobs/src/lib.rs";
    let worker_content = test_support::read_file(root, worker_rel);

    write_file(
        root,
        worker_rel,
        &format!(
            "{worker_content}\n\npub mod internal {{ pub struct Visible; }}\npub use internal::Visible;\n"
        ),
    );

    let results = run_family(root);
    assert_no_hits(&results);
}
