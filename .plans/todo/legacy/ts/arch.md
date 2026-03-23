# TS-ARCH — TypeScript architecture checker (2 check groups)

**Input:** Directory structure + *.ts/*.tsx source files
**Current code:** `ts_arch_checks.rs`, `arch_helpers.rs`

## TS-ARCH-01: Hex arch structure (7 sub-rules)

| Sub-rule | What | Status |
|----------|------|--------|
| rule_01 | src/modules/ must exist for TS service apps | Implemented + 38 tests (3 rounds) |
| rule_02 | modules/ must contain exactly {domain, ports, application, adapters} | Implemented + 36 tests (3 rounds) |
| rule_03 | adapters/ and ports/ must contain exactly {inbound, outbound} | Implemented + 43 tests (2 rounds) |
| rule_04 | Loose files in container dirs (only .gitkeep allowed) | Implemented + 51 tests (3 rounds) |
| rule_05 | Container dirs must not be empty | Implemented + 37 tests (3 rounds) |
| rule_06 | Leaf validity (.ts/.tsx files, .gitkeep, or modules/ hex-in-hex) | Implemented, tests first-draft |
| rule_07 | Hex-in-hex recursion via modules/ | Implemented, tests first-draft |

## TS-ARCH-02: Import boundary violations

| New ID | Old ID | What | Description | Status |
|--------|--------|------|-------------|--------|
| TS-ARCH-02 | T-ARCH-02 | import boundaries | Domain can't import adapters, application can't import adapters, etc. | Implemented, tests in rule_07.rs |

**Note:** T-ARCH-02 is rolled into TS-ARCH-01 rule_07 in the current test structure (same check code file).
