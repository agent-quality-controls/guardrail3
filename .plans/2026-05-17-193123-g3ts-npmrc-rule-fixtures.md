Goal
- Complete CLI-visible fixture coverage for the `g3ts-npmrc` family.
- Keep the fixture count minimal while ensuring every non-inventory npmrc rule emits `Error` or `Warn` in at least one broken fixture.

Approach
- Add `behavior/fixtures/g3ts-rule/npmrc/npmrc-R00-clean-golden`.
  - Contains `pnpm-workspace.yaml` so npmrc policy applies.
  - Contains a root `.npmrc` with exactly the required baseline settings.
  - Expects zero exit and only inventory findings.
- Add `behavior/fixtures/g3ts-rule/npmrc/npmrc-R10-missing-root`.
  - Contains `pnpm-workspace.yaml` but no `.npmrc`.
  - Exposes `g3ts-npmrc/root-exists`.
- Add `behavior/fixtures/g3ts-rule/npmrc/npmrc-R20-parse-error`.
  - Contains a root `.npmrc` with malformed syntax.
  - Exposes `g3ts-npmrc/root-parseable`.
- Add `behavior/fixtures/g3ts-rule/npmrc/npmrc-R30-policy-violations`.
  - Contains a parseable `.npmrc` with duplicate keys, missing required keys, weakened required values, and one extra setting.
  - Exposes `g3ts-npmrc/duplicate-keys`, `g3ts-npmrc/required-settings-present`, and `g3ts-npmrc/required-settings-strong-enough`.
  - Also emits `g3ts-npmrc/extra-settings-inventory` as inventory.
- Mark `npmrc` as completed in `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`.
- Add `g3ts-npmrc/extra-settings-inventory` to `inventory_only_rule` because it intentionally reports `Info`, not failure.
- Refresh `behavior/golden/g3ts-rule/approved.normalized.json` through `fixture3`.

Key decisions
- Do not add rule code. The npmrc family already has six production rules; this slice covers their external CLI behavior.
- Do not create one fixture per rule. `npmrc-R30-policy-violations` combines independent semantic npmrc issues that do not hide each other.
- Keep `extra-settings-inventory` inventory-only. It is designed as `Info`; forcing it to fail would change product behavior, not fixture coverage.

Files to modify
- `.plans/2026-05-17-185551-g3ts-family-rule-fixtures.md.manifest.toml`
- `behavior/fixtures/g3ts-rule/npmrc/**`
- `behavior/golden/g3ts-rule/approved.normalized.json`
