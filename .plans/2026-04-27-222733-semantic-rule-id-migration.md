# Goal

Rename active G3TS and G3RS rule IDs from numeric family IDs to semantic ESLint-style IDs.

Old shape:

```text
g3ts-astro-mdx/no-raw-mdx-images
g3rs-fmt/settings
```

New shape:

```text
g3ts-astro-mdx/no-raw-mdx-images
g3rs-fmt/rustfmt-config-exists
```

# Non-Negotiable Requirements

- No compatibility aliases.
- No dual output.
- No active numeric IDs in runtime output, tests, or active docs.
- Every semantic ID must identify one assertion.
- Rule package prefix must match the owning family or subfamily.
- Rule names must be lowercase kebab-case.
- The migration must cover active `packages/ts`, `apps/guardrail3-ts`, `packages/rs`, and `apps/guardrail3-rs`.
- Archived or legacy code is not an active target unless it is compiled or used by the current CLIs.

# Approach

1. Inventory current numeric IDs from active TS and RS code.
2. Write inventory files under `.plans/rule-id-migration/`.
3. Create a one-to-one mapping from old numeric IDs to semantic IDs.
4. Validate mapping uniqueness and naming quality.
5. Rename constants, rule files, tests, snapshots, reports, and docs that are active.
6. Update golden outputs and command-line assertions.
7. Grep for old active numeric IDs and remove all active occurrences.
8. Run TS and RS workspaces and package guardrails.
9. Run adversarial review against this plan and the final code.
10. Commit the completed migration with a worklog.

# Naming Rules

- Prefix starts with the binary ecosystem and concrete family:
  - `g3ts-astro-setup`
  - `g3ts-astro-content`
  - `g3ts-astro-mdx`
  - `g3ts-astro-seo`
  - `g3ts-astro-state`
  - `g3ts-arch`
  - `g3ts-apparch`
  - `g3ts-jscpd`
  - `g3rs-fmt`
  - `g3rs-toolchain`
  - `g3rs-clippy`
  - `g3rs-deny`
  - `g3rs-cargo`
  - `g3rs-code`
  - `g3rs-apparch`
  - `g3rs-deps`
  - `g3rs-garde`
  - `g3rs-test`
  - `g3rs-release`
- Suffix is the semantic assertion:
  - good: `astro-config-uses-static-output`
  - good: `mdx-component-wrapper-requires-zod-parse`
  - bad: `config-05`
  - bad: `rule-1`

# Files To Produce

- `.plans/rule-id-migration/ts-inventory.md`
- `.plans/rule-id-migration/rs-inventory.md`
- `.plans/rule-id-migration/rule-id-map.toml`
- `.worklogs/<timestamp>-semantic-rule-id-migration.md`

# Verification

- `rg 'TS-[A-Z0-9-]+-[0-9]{2}|RS-[A-Z0-9-]+-[0-9]{2}' packages/ts apps/guardrail3-ts packages/rs apps/guardrail3-rs` returns no active runtime/test matches.
- `cargo test --workspace --offline --locked` in `apps/guardrail3-ts`.
- `cargo test --workspace --offline --locked` in `apps/guardrail3-rs`.
- `g3rs validate` passes on touched Rust packages.
- `g3ts validate` smoke test still prints semantic IDs.
- `g3rs validate` smoke test still prints semantic IDs.
