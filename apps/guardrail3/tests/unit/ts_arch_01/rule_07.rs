use super::helpers::{copy_fixture, import_errors, run_import_check, write_file};
use guardrail3_domain_report::Severity;

// ============================================================================
// Rule 07: T-ARCH-02 import boundary violations
// ============================================================================

#[test]
fn golden_no_import_violations() {
    let tmp = copy_fixture();
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert!(
        errors.is_empty(),
        "golden should have 0 T-ARCH-02 errors, got: {errors:#?}"
    );
}

#[test]
fn domain_imports_adapters_fails() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "import { DbAdapter } from '../../adapters/outbound/validator/client';\nexport type User = { id: string };\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 import violation, got {}: {errors:#?}",
        errors.len()
    );
    assert!(
        errors[0].message.contains("domain"),
        "should mention domain layer"
    );
    assert!(
        errors[0].message.contains("adapters"),
        "should mention adapters layer"
    );
}

#[test]
fn domain_imports_application_fails() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "import { validate } from '../../application/commands/validate-live';\nexport type User = { id: string };\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 import violation, got {}: {errors:#?}",
        errors.len()
    );
    assert!(errors[0].message.contains("domain"));
    assert!(errors[0].message.contains("application"));
}

#[test]
fn domain_imports_ports_fails() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "import { UseCase } from '../../ports/inbound/use-cases';\nexport type User = { id: string };\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 import violation, got {}: {errors:#?}",
        errors.len()
    );
    assert!(errors[0].message.contains("domain"));
    assert!(errors[0].message.contains("ports"));
}

#[test]
fn application_imports_domain_ok() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/application/commands/validate-live.ts",
        "import { User } from '../../domain/types';\nexport function validate(u: User) {}\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert!(
        errors.is_empty(),
        "application -> domain should be allowed, got: {errors:#?}"
    );
}

#[test]
fn application_imports_ports_ok() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/application/commands/validate-live.ts",
        "import { UseCase } from '../../ports/inbound/use-cases';\nexport function validate() {}\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert!(
        errors.is_empty(),
        "application -> ports should be allowed, got: {errors:#?}"
    );
}

#[test]
fn application_imports_adapters_fails() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/application/commands/validate-live.ts",
        "import { client } from '../../adapters/outbound/validator/client';\nexport function validate() {}\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 import violation, got {}: {errors:#?}",
        errors.len()
    );
    assert!(errors[0].message.contains("application"));
    assert!(errors[0].message.contains("adapters"));
}

#[test]
fn ports_imports_domain_ok() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/ports/inbound/use-cases/index.ts",
        "import { User } from '../../../domain/types';\nexport interface UseCase { exec(u: User): void; }\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert!(
        errors.is_empty(),
        "ports -> domain should be allowed, got: {errors:#?}"
    );
}

#[test]
fn ports_imports_adapters_fails() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/ports/inbound/use-cases/index.ts",
        "import { client } from '../../../adapters/outbound/validator/client';\nexport interface UseCase {}\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 import violation, got {}: {errors:#?}",
        errors.len()
    );
    assert!(errors[0].message.contains("ports"));
    assert!(errors[0].message.contains("adapters"));
}

#[test]
fn adapters_imports_everything_ok() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/adapters/outbound/validator/client.ts",
        "\
import { User } from '../../../domain/types';
import { UseCase } from '../../../ports/inbound/use-cases';
import { validate } from '../../../application/commands/validate-live';
export function connect() {}
",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert!(
        errors.is_empty(),
        "adapters should be able to import from all layers, got: {errors:#?}"
    );
}

#[test]
fn alias_import_at_modules_detected() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "import { client } from '@/modules/adapters/outbound/validator/client';\nexport type User = { id: string };\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 alias import violation, got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn alias_import_tilde_detected() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "import { client } from '~/modules/adapters/outbound/validator/client';\nexport type User = { id: string };\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 tilde alias import violation, got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn direct_layer_alias_detected() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "import { client } from '@adapters/outbound/validator/client';\nexport type User = { id: string };\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 direct layer alias violation, got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn multiple_violations_in_one_file() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "\
import { client } from '../../adapters/outbound/validator/client';
import { validate } from '../../application/commands/validate-live';
import { UseCase } from '../../ports/inbound/use-cases';
export type User = { id: string };
",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        3,
        "expected 3 violations (domain -> adapters, application, ports), got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn comments_not_flagged() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "\
// import { client } from '../../adapters/outbound/validator/client';
/* import { validate } from '../../application/commands/validate-live'; */
export type User = { id: string };
",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert!(
        errors.is_empty(),
        "commented-out imports should not be flagged, got: {errors:#?}"
    );
}

#[test]
fn line_numbers_reported() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "export type User = { id: string };\nimport { client } from '../../adapters/outbound/validator/client';\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(errors.len(), 1, "expected 1 violation");
    assert_eq!(
        errors[0].line,
        Some(2),
        "expected line 2, got: {:?}",
        errors[0].line
    );
}

#[test]
fn require_syntax_detected() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "const client = require('../../adapters/outbound/validator/client');\nexport type User = { id: string };\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    assert_eq!(
        errors.len(),
        1,
        "expected 1 require() violation, got {}: {errors:#?}",
        errors.len()
    );
}

#[test]
fn all_violations_have_file_field() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "import { client } from '../../adapters/outbound/validator/client';\nexport type User = {};\n",
    );
    let results = run_import_check(tmp.path());
    let errors = import_errors(&results);
    for err in &errors {
        assert!(
            err.file.is_some(),
            "expected file field set, got None: {err:#?}"
        );
        assert!(
            err.file
                .as_deref()
                .unwrap_or("")
                .contains("domain/types/index.ts"),
            "expected file path to contain source file, got: '{}'",
            err.file.as_deref().unwrap_or("")
        );
    }
}

#[test]
fn all_violations_are_errors() {
    let tmp = copy_fixture();
    write_file(
        tmp.path(),
        "apps/admin/src/modules/domain/types/index.ts",
        "import { client } from '../../adapters/outbound/validator/client';\nexport type User = {};\n",
    );
    let results = run_import_check(tmp.path());
    for r in &results {
        if r.id == "T-ARCH-02" {
            assert!(
                matches!(r.severity, Severity::Error),
                "T-ARCH-02 violations should be Error severity, got: {:?}",
                r.severity
            );
        }
    }
}
