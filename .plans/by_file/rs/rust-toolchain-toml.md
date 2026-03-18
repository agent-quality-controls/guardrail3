# rust-toolchain.toml

## Location

**Where Rust looks:** Walks UP from CWD toward filesystem root. Nearest `rust-toolchain.toml` or `rust-toolchain` wins. No merging.

**In steady-parent:**
- `apps/validator-rust/rust-toolchain.toml` — `channel = "stable"`, `components = ["rustfmt", "clippy"]`
- NO root rust-toolchain.toml
- NO substack-publisher rust-toolchain.toml

**Scoping question:** Should this be per-workspace or repo-root?

Per-workspace means each app pins its own toolchain. This makes sense if different apps need different Rust versions (app-A needs nightly for a feature, app-B uses stable). But in practice, monorepos almost always use one Rust version everywhere — CI and developer machines have one toolchain.

Repo-root means one file covers everything. Simpler, matches common practice.

**Decision: per-workspace (same as clippy/rustfmt/deny).** Reasoning:
- Extractability — if you pull an app out of the monorepo, it brings its toolchain spec
- A workspace with rust-toolchain.toml is self-contained
- If someone wants repo-root-only, they just put it at root and don't put per-app ones (rustup walks up, finds root)

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
3. **Per-crate rust-toolchain.toml:** Unlike clippy/deny, rustup resolves rust-toolchain.toml by walking up from CWD, not from crate manifest. So a per-crate file is unusual but not dangerous in the same way. Still, one per workspace is sufficient.

## Parser

`toml_edit`. Simple structure: `[toolchain]` section with 2-4 keys. Components is a TOML array of strings — need to check membership and append if missing.
