# Goal

Extract the non-file-tree `arch` and `hexarch` lanes from `apps/guardrail3` into `packages/rs` using the current package architecture:

- family-wide ingestion package
- `source-checks` package for source/content rules
- `config-checks` package for config/dependency-manifest rules

The end state is:

- `packages/rs/arch/*` exists and runs real source + config lanes
- `packages/rs/hexarch/*` exists and runs real source + config lanes
- package tests cover the extracted rules
- no rule logic is invented; package behavior matches the current app family behavior for the extracted lanes

# Approach

1. Build `arch` first because it is smaller and proves the mixed-family pattern.
   - Create:
     - `packages/rs/arch/g3rs-arch-source-checks`
     - `packages/rs/arch/g3rs-arch-config-checks`
     - `packages/rs/arch/g3rs-arch-ingestion`
   - Source slice:
     - `RS-ARCH-01`, `02`, `03`, `04`, `08`, `09`
   - Config slice:
     - `RS-ARCH-05`, `06`, `07`
   - Reuse existing fact builders from the app family where possible, but keep public package boundaries typed and package-local.

2. Build `hexarch` second using the same pattern.
   - Create:
     - `packages/rs/hexarch/g3rs-hexarch-source-checks`
     - `packages/rs/hexarch/g3rs-hexarch-config-checks`
     - `packages/rs/hexarch/g3rs-hexarch-ingestion`
   - Source slice:
     - `RS-HEXARCH-22`, `23`
   - Config slice:
     - `RS-HEXARCH-08`, `10`, `11`, `13`-`21`, `24`-`26`
   - Keep structural/file-tree rules out.

3. Add package-level pipeline tests in each ingestion package.
   - `crawl -> ingest_for_source_checks -> source-checks::check`
   - `crawl -> ingest_for_config_checks -> config-checks::check`

4. Verify each new family workspace independently.

# Key decisions

- Keep the lane names as `source`, `config`, `file-tree`.
  - Rejected: reviving `ast` naming.

- Extract only non-file-tree rules.
  - Rejected: pulling structural shape checks into these packages in the same pass.

- Match current app behavior before redesigning semantics.
  - Rejected: policy rewrites during extraction.

# Files to modify

- `.plans/2026-04-10-202849-arch-hexarch-package-extraction.md`
- `.worklogs/...-arch-hexarch-package-extraction.md`
- `packages/rs/arch/**`
- `packages/rs/hexarch/**`
- any workspace manifests or scripts that register the new package directories
