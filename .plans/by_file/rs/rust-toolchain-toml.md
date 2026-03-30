# rust-toolchain.toml

> Historical research note: the current live `RS-TOOLCHAIN` family is a routed
> policy-root family, not a validation-root family. See
> [`.plans/by_family/rs/toolchain.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/by_family/rs/toolchain.md)
> and
> [`apps/guardrail3/crates/app/rs/families/toolchain/README.md`](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/toolchain/README.md)
> for the current contract.

## Location

**Where Rust looks:** Walks UP from CWD toward filesystem root. Nearest
`rust-toolchain.toml` or `rust-toolchain` wins. No merging. If both filenames
exist in the same directory, `rust-toolchain` wins for backward compatibility.

**In steady-parent:**
- `apps/validator-rust/rust-toolchain.toml` — `channel = "stable"`, `components = ["rustfmt", "clippy"]`
- NO root rust-toolchain.toml
- NO substack-publisher rust-toolchain.toml

**Scoping question:** Should this be per-workspace or repo-root?

That question has since been resolved in favor of routed policy-root
ownership. The current family validates one local `rust-toolchain.toml`,
optional local legacy `rust-toolchain`, and one local `Cargo.toml` MSRV source
for each owned workspace root. Any nested toolchain beneath that workspace root
and any toolchain outside all governed workspace roots is a violation. Shared
Rust exclusions such as `target/`, `tests/fixtures/`, `tests/snapshots/`, and
`.claude/worktrees/` stay out of that placement surface.

The older reasoning below is preserved only as historical context for the
pre-routing discussion:
- Extractability — if you pull an app out of the monorepo, it brings its
  toolchain spec
- A workspace with rust-toolchain.toml is self-contained
- If someone wants repo-root-only, they just put it at root and don't put
  per-app ones (rustup walks up, finds root)

## Contents

Only 3 possible keys under `[toolchain]`:

| Key | guardrail3 value | Description |
|---|---|---|
| channel | "stable" | Toolchain channel. Could be "stable", "nightly", "1.80.0", etc. |
| components | ["clippy", "rustfmt"] | Required components. User might add "rust-src", "llvm-tools", "miri" |
| targets | (not set) | Cross-compilation targets. User might add "wasm32-unknown-unknown" |
| profile | (not set) | Toolchain profile. Usually default. |

## Category: Merge-managed

**guardrail3 ensures:**
- `channel` exists (default "stable")
- `components` contains "clippy" and "rustfmt" (required for guardrail3 to work)

**User owns:**
- `channel` value (project might need nightly or a pinned version like "1.80.0")
- Extra components beyond clippy/rustfmt
- `targets` array
- `profile`

## Algorithm

### On `generate` (existing file):
```
1. Parse with toml_edit
2. channel: if missing ADD "stable". If present LEAVE (project choice — validate warns if nightly without reason)
3. components: if missing ADD ["clippy", "rustfmt"]. If present, ensure "clippy" and "rustfmt" are in the array — ADD if missing, LEAVE existing entries
4. targets: LEAVE
5. profile: LEAVE
6. Write back
```

### On `generate` (new file):
```
1. Write [toolchain] with channel = "stable", components = ["clippy", "rustfmt"]
```

## Override mechanism

None needed. User edits the file directly. guardrail3 only ensures clippy and rustfmt are in components.

## Edge cases

1. **Legacy `rust-toolchain` file (no .toml):** Old format, just contains the channel string. Warn — recommend migrating to .toml format.
2. **Nightly pinning:** Some projects use `channel = "nightly-2024-01-15"`. guardrail3 should leave this alone — it's a deliberate pin.
3. **Per-crate rust-toolchain.toml:** Unlike clippy/deny, rustup resolves
   rust-toolchain.toml by walking up from CWD, not from crate manifest. The
   current guardrail family models ownership at routed policy roots rather than
   raw rustup walk-up behavior.

## Parser

`toml_edit`. Simple structure: `[toolchain]` section with 2-4 keys. Components is a TOML array of strings — need to check membership and append if missing.
