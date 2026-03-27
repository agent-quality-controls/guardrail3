use guardrail3_app_rs_family_code_assertions::rs_code_14_unwrap_expect::assert_attacks_unwrap_and_expect_calls_across_real_owned_files_with_exact_metadata;
use super::super::run_family;
use super::super::copy_fixture;
use test_support::write_file;

#[test]
fn attacks_unwrap_and_expect_calls_across_real_owned_files_with_exact_metadata() {
    let fixture = copy_fixture();
    let root = fixture.path();

    let backend_rel = "apps/backend/crates/app/queries/src/lib.rs";
    let worker_rel = "apps/worker/crates/adapters/outbound/sqs/src/lib.rs";

    let backend_content =
        test_support::read_file(root, backend_rel);
    let worker_content =
        test_support::read_file(root, worker_rel);

    let backend_new = format!(
        "{backend_content}\nfn unwrap_probe() {{ let _ = Some(1).unwrap(); }}\nfn expect_probe() {{ let _ = Some(1).expect(\"backend\"); }}\n"
    );
    let worker_new = format!(
        "{worker_content}\nmod nested_method_probe {{\n    pub fn run() {{\n        let _ = Some(1).expect(\"queue\");\n        let _ = Some(1).unwrap();\n    }}\n}}\n"
    );

    write_file(root, backend_rel, &backend_new);
    write_file(root, worker_rel, &worker_new);

    let backend_unwrap_line = backend_new
        .lines()
        .position(|line| line.contains("fn unwrap_probe()")).map(|index| index + 1).unwrap_or_default();
    let backend_expect_line = backend_new
        .lines()
        .position(|line| line.contains("fn expect_probe()")).map(|index| index + 1).unwrap_or_default();
    let worker_expect_line = worker_new
        .lines()
        .position(|line| line.contains("let _ = Some(1).expect(\"queue\");")).map(|index| index + 1).unwrap_or_default();
    let worker_unwrap_line = worker_new
        .lines()
        .position(|line| line.contains("let _ = Some(1).unwrap();")).map(|index| index + 1).unwrap_or_default();

    assert_attacks_unwrap_and_expect_calls_across_real_owned_files_with_exact_metadata(
        &run_family(root),
        backend_rel,
        worker_rel,
        backend_unwrap_line,
        backend_expect_line,
        worker_expect_line,
        worker_unwrap_line,
    );
}
