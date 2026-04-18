## Goal

Create the first real `guardrail3-rs.toml` inside a package workspace and validate that workspace through the new `guardrail3-rs` CLI.

## Approach

- Use `packages/rs/garde/g3rs-garde-source-checks` as the first package workspace.
- Derive the workspace's real external dependency set from `cargo metadata` instead of guessing.
- Add a minimal, meaningful `guardrail3-rs.toml`:
  - `profile = "library"`
  - `allowed_deps` matching the actual external dependencies of that workspace
- Run `guardrail3-rs validate --path packages/rs/garde/g3rs-garde-source-checks` before and after adding the file.
- Confirm that the `deps` family no longer errors on missing config and that the remaining findings are the real package findings.

## Key decisions

- Chosen workspace: `g3rs-garde-source-checks`
  - Reason: it is a real standalone package workspace and already has a strong package boundary.
- Chosen profile: `library`
  - Reason: this workspace is a publishable library package set, not an app/service.
- Keep the file minimal.
  - Do not add parser-supported but currently non-meaningful fields like `checks`, `waivers`, or `excluded_paths`.

## Files to modify

- `packages/rs/garde/g3rs-garde-source-checks/guardrail3-rs.toml`
