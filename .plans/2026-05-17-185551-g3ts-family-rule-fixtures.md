# G3TS Family Rule Fixtures

## Goal

Build the TypeScript-side equivalent of the current G3RS fixture set.

The target is external CLI behavior only:

- `g3ts validate --path <fixture-repo> --family <family> --rules-only --inventory`
- `g3ts validate-repo --path <fixture-repo>`
- stdout
- stderr
- exit code
- emitted rule IDs
- inventory output

The fixture set must not test internal ingestion structs, rule input structs, helper functions, assertion helper crates, or private package boundaries.

## Current State

G3TS has no active `fixture3` suites.

Current `fixture3.yaml` suites are all Rust-only:

- `g3rs-rule-fixtures`
- `g3rs-validate-repo`
- `g3rs-cli-output`

Current TS test/assertion surface still exists:

- `120` test/assertion directories under `packages/ts` and `apps/guardrail3-ts`
- `252` test/assertion files
- `11927` test/assertion lines

Current active TS source surface:

- `816` Rust source files under `packages/ts` and `apps/guardrail3-ts`
- `53520` Rust source lines

Current active G3TS rule IDs discovered from production TS packages:

- `apparch`: `9`
- `arch`: `7`
- `astro-content`: `17`
- `astro-i18n`: `11`
- `astro-mdx`: `14`
- `astro-media`: `14`
- `astro-seo`: `23`
- `astro-setup`: `19`
- `astro-state`: `2`
- `eslint`: `17`
- `fmt`: `8`
- `hooks`: `25`
- `jscpd`: `6`
- `npmrc`: `6`
- `package`: `10`
- `spelling`: `8`
- `style`: `13`
- `topology`: `1`
- `tsconfig`: `5`
- `typecov`: `7`

Total discovered production G3TS rule IDs: `222`.

## Fixture Layout

Add one rule fixture suite:

```text
behavior/fixtures/g3ts-rules/<family>/<fixture-id>/fixture.toml
behavior/fixtures/g3ts-rules/<family>/<fixture-id>/repo/...
behavior/golden/g3ts-rule-fixtures/approved.normalized.json
```

Add one repo-level suite:

```text
behavior/fixtures/g3ts-validate-repo/<fixture-id>/fixture.toml
behavior/fixtures/g3ts-validate-repo/<fixture-id>/repo/...
behavior/golden/g3ts-validate-repo/approved.normalized.json
```

Add one CLI-output suite:

```text
behavior/fixtures/g3ts-cli-output/<fixture-id>/fixture.toml
behavior/fixtures/g3ts-cli-output/<fixture-id>/repo/...
behavior/golden/g3ts-cli-output/approved.normalized.json
```

Do not create ingestion fixture suites.

Do not create package-internal fixture suites.

Do not serialize internal TS family structs for fixture comparison.

## Family Rule Fixture Contract

Each family folder must contain exactly one clean fixture:

```text
behavior/fixtures/g3ts-rules/<family>/<family>-R00-clean-golden/
```

Each family folder must contain the minimum number of broken fixtures needed to expose every CLI-visible rule in that family.

Fixture metadata must include:

```toml
id = "eslint-R10-missing-and-invalid-config"
tool = "g3ts"
run_from = "repo"
commands = [
  ["validate", "--path", ".", "--family", "eslint", "--rules-only", "--inventory"],
]
expected_exit = "nonzero"
level = "family_rule_policy"

rule_family = "eslint"
target_rules = [
  "g3ts-eslint/exists",
  "g3ts-eslint/parseable",
]
expected_findings = [
  "g3ts-eslint/exists",
  "g3ts-eslint/parseable",
]
```

Required fields:

- `id`
- `tool`
- `run_from`
- `commands`
- `expected_exit`
- `level`
- `rule_family`
- `target_rules`
- `expected_findings`

Allowed `expected_exit` values:

- `zero`
- `nonzero`

Allowed fixture levels:

- `family_rule_clean_golden`
- `family_rule_policy`
- `family_rule_input_failure`
- `family_rule_filetree`
- `family_rule_source`

## Grouping Rule

Use the smallest fixture count that preserves rule visibility.

Merge rules into one broken fixture when all of these are true:

- one repo state can trigger every target rule
- no target finding prevents another target finding from being evaluated
- the fixture output remains readable
- the fixture does not rely on a parse failure that hides deeper checks

Split fixtures when any of these are true:

- missing required file prevents parse-dependent checks
- invalid config prevents semantic checks
- missing package prevents plugin-wiring checks
- source parse failure prevents source policy checks
- toolchain-gate failure replaces static family output

## G3TS Suite Wiring

Add these suites to `fixture3.yaml`:

```yaml
g3ts-rule-fixtures:
  fixtures:
    - "behavior/fixtures/g3ts-rules/*/*/fixture.toml"
  command:
    argv:
      - "python3"
      - "scripts/behavior/fixture3-g3ts-replay.py"
      - "--manifest"
      - ".plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml"
      - "{fixtures}"

g3ts-validate-repo:
  fixtures:
    - "behavior/fixtures/g3ts-validate-repo/*/fixture.toml"
  command:
    argv:
      - "python3"
      - "scripts/behavior/fixture3-g3ts-replay.py"
      - "--manifest"
      - ".plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml"
      - "{fixtures}"

g3ts-cli-output:
  fixtures:
    - "behavior/fixtures/g3ts-cli-output/*/fixture.toml"
  command:
    argv:
      - "python3"
      - "scripts/behavior/fixture3-g3ts-replay.py"
      - "--manifest"
      - ".plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml"
      - "{fixtures}"
```

The G3TS replay script should mirror `scripts/behavior/fixture3-g3rs-replay.py`, with only the tool name and schema version changed.

## Verification Scripts

Add:

```text
scripts/behavior/fixture3-g3ts-replay.py
scripts/behavior/verify-g3ts-family-rule-fixtures.py
scripts/behavior/verify-g3ts-rule-coverage.py
```

`verify-g3ts-family-rule-fixtures.py` must:

- read the manifest
- discover active G3TS production rule IDs under `packages/ts`
- ignore `target`, `tests`, `contract_tests`, `assertions`, and `*_tests`
- load every `behavior/fixtures/g3ts-rules/*/*/fixture.toml`
- require exactly one `family_rule_clean_golden` fixture per completed family
- require every `target_rules` entry to exist in active production rule IDs
- require every `target_rules` entry to also be listed in `expected_findings`
- require every `expected_findings` entry to appear in approved fixture stdout
- require every non-clean fixture target rule to emit `Error` or `Warn`
- fail on duplicate fixture IDs

`verify-g3ts-rule-coverage.py` must:

- read the manifest
- discover all active G3TS production rule IDs
- read approved G3TS fixture outputs
- classify each rule ID as:
  - broken by fixture
  - inventory-only
  - CLI-unreachable
  - missing coverage
- fail if any active production rule ID has no classification
- fail if any manifest-listed rule ID does not exist in production source

## Implementation Order

Implement in this order:

1. Replay harness and verifier scripts.
2. `fixture3.yaml` suite wiring.
3. G3TS CLI-output fixtures.
4. G3TS validate-repo fixtures.
5. Core package/config families:
   - `package`
   - `npmrc`
   - `tsconfig`
   - `eslint`
   - `jscpd`
   - `fmt`
   - `spelling`
   - `typecov`
   - `style`
6. Structure families:
   - `topology`
   - `arch`
   - `apparch`
   - `hooks`
7. Astro families:
   - `astro-setup`
   - `astro-content`
   - `astro-mdx`
   - `astro-i18n`
   - `astro-media`
   - `astro-seo`
   - `astro-state`
8. Reduce broken fixtures with `fixture3 reduce`.
9. Delete G3TS unit tests and assertion helper crates only after fixture coverage proves every CLI-visible rule.

## Deletion Rule

Do not delete any G3TS test or assertion code until:

- `fixture3 check --suite g3ts-rule-fixtures --json` passes
- `fixture3 check --suite g3ts-validate-repo --json` passes
- `fixture3 check --suite g3ts-cli-output --json` passes
- `python3 scripts/behavior/verify-g3ts-family-rule-fixtures.py` passes
- `python3 scripts/behavior/verify-g3ts-rule-coverage.py` passes
- every active G3TS production rule ID is covered, inventory-only, or CLI-unreachable

## Non-Goals

- Do not port Rust fixtures to TypeScript by text replacement.
- Do not create fixtures for internal ingestion structs.
- Do not create one fixture per rule when multiple rules can be exposed together.
- Do not keep broad composite fixtures as final coverage.
- Do not delete TS tests during the first fixture build pass.
