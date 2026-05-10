## Goal

Build a separate `guardrail3-ts` Rust CLI app that exposes only `validate`, and migrate the first four TypeScript validation families into `packages/ts/{family}`:

- `ts/eslint`
- `ts/tsconfig`
- `ts/npmrc`
- `ts/package`

End state for this phase:

- `apps/guardrail3-ts` exists as a validate-only app.
- It is structurally as close as practical to `apps/guardrail3-rs`.
- The four TS families exist as Rust package groups under `packages/ts`.
- Those Rust packages themselves pass Rust guardrails.
- No generation, init, diff, check, or write path exists yet.

## Approach

1. Clone the active app shape from `guardrail3-rs` into `guardrail3-ts`.
   - Mirror the package split and CLI wiring patterns.
   - Replace Rust family orchestration with TS family orchestration only.
   - Keep the command surface minimal: `Validate` only.

2. Create `packages/ts/{family}` package groups using the same package discipline as `packages/rs/{family}`.
   - Root `Cargo.toml`, `README.md`, `clippy.toml`, `deny.toml`, `rustfmt.toml`, `rust-toolchain.toml`, `guardrail3-rs.toml`.
   - Inner Rust crates follow the same sibling-crate shape used in the Rust families.

3. Migrate family-by-family from the legacy grouped TS validator under:
   - `legacy/apps/guardrail3-current/crates/app/ts/validate`
   - Parse once in orchestrators.
   - Give pure rules the smallest typed local input.
   - Keep shared config-file handoff as whole typed files across family boundaries.

4. Start with the four config-policy families that give the highest validation leverage.
   - `ts/package`
   - `ts/npmrc`
   - `ts/tsconfig`
   - `ts/eslint`

5. Keep family boundaries strict from day one.
   - `ts/eslint` owns ESLint config and baseline plugin/rule policy.
   - `ts/tsconfig` owns TypeScript config strictness and inheritance.
   - `ts/npmrc` owns root `.npmrc` policy only.
   - `ts/package` owns `package.json` manifest policy only.
   - Do not re-create the legacy cross-family blob.

## Key Decisions

- Separate app, not a second mode inside `guardrail3-rs`.
  - Reason: the user explicitly wants two different projects with the same idea and structure.

- Validate only for now.
  - Reason: this matches the successful Rust cutover sequence and keeps scope tight.

- Near-copy the `guardrail3-rs` app shape.
  - Reason: the Rust app is the current working reference for package boundaries and CLI flow.

- Treat the TS families as Rust package groups.
  - Reason: these packages are implemented in Rust and must satisfy Rust guardrails themselves.

- Shared config files cross family boundaries as whole typed files, not central slices.
  - Reason: slicing re-creates the same central-knowledge problem already identified on the Rust side.

- Package ownership must be explicit where config surfaces overlap.
  - Example: `ts/package` may expose a typed `PackageJson` to `ts/eslint`, but it should not own ESLint package-presence rules that belong to `ts/eslint`.

## Latest Ideas, Not Final Decisions

- `guardrail3-ts` should be almost an exact structural copy of `guardrail3-rs`, not a custom one-off app.
- `package.json` should remain a shared typed file that multiple TS families can read, with field ownership staying explicit per family.
- `ts/npmrc` may need to narrow versus the older plan because current pnpm and npm docs no longer support treating root `.npmrc` as a giant general policy surface.
- `ts/eslint` should likely target modern flat config plus `typescript-eslint` typed linting from the start, instead of carrying forward the old string-match assumptions.

## Files To Modify

- `apps/guardrail3-ts/**`
- `packages/ts/eslint/**`
- `packages/ts/tsconfig/**`
- `packages/ts/npmrc/**`
- `packages/ts/package/**`
- top-level workspace manifests as needed to include the new app and package groups

## First Implementation Order

1. Scaffold `apps/guardrail3-ts` as a validate-only copy of the Rust app shape.
2. Build `packages/ts/package`.
3. Build `packages/ts/npmrc`.
4. Build `packages/ts/tsconfig`.
5. Build `packages/ts/eslint`.
6. Wire the four families into `guardrail3-ts validate`.

## Source Inputs

- `.plans/todo/checks/ts/README.md`
- `.plans/todo/checks/ts/eslint.md`
- `.plans/todo/checks/ts/tsconfig.md`
- `.plans/todo/checks/ts/npmrc.md`
- `.plans/todo/checks/ts/package.md`
- `.plans/by_family/ts/eslint.md`
- `.plans/by_family/ts/package.md`
- `.plans/by_file/ts/eslint-config-mjs.md`
- `.plans/by_file/ts/tsconfig.md`
- `.plans/by_file/ts/npmrc.md`
- `legacy/apps/guardrail3-current/crates/app/ts/validate/**`
- `apps/guardrail3-rs/**`
