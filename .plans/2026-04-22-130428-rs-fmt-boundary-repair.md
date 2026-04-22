Goal
- Repair the `rs/fmt` config seam so `g3rs-fmt-config-checks` consumes ingestion-owned family facts instead of parser-owned TOML documents and parser-type adapters.
- Keep the repair scoped to the config lane.

Approach
- Add a red proving test in `g3rs-fmt-config-checks` that populates only prebound fmt config facts and proves `run.rs` dispatches from those facts instead of parser-owned states.
- Replace parser-leaking config states in `g3rs-fmt-types/src/types.rs` with family-owned facts:
  - rustfmt parse state plus explicit rustfmt baseline fields, explicit keys, nightly-key inventory, and ignore inventory
  - cargo parse state plus resolved edition fact
  - toolchain parse state plus resolved channel fact
  - keep rust-policy waivers as family-owned data
- Move fact derivation into `g3rs-fmt-ingestion/crates/runtime/src/run.rs` at the architecturally correct parse-once boundary.
- Remove parser-type adapter logic from `g3rs-fmt-config-checks/crates/runtime/src/inputs.rs` and update rules to read only family facts.
- Rewire test support and rule tests to construct family facts directly instead of parser-owned TOML values.

Key decisions
- Choose `rs/fmt` before `rs/cargo`.
  - Why: `fmt` has the same parser-leak defect with a much smaller surface. It is the next clean seam repair, while `cargo` would be a broader family rewrite.
  - Rejected: starting with `cargo`. The defect is real there, but the scope is too wide for the next incremental repair.
- Keep this repair config-only.
  - Why: the confirmed defect is in `g3rs-fmt-config-checks` and its public types.
  - Rejected: mixing file-tree or source changes into the same commit.
- Preserve existing rule IDs and behavior.
  - Why: this is a boundary repair, not a policy rewrite.
  - Rejected: redesigning the fmt rule inventory while changing the seam.

Files to modify
- `packages/rs/fmt/g3rs-fmt-types/src/types.rs`
- `packages/rs/fmt/g3rs-fmt-types/src/lib.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/run.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/inputs.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_01_settings/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_02_extra_settings/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_03_nightly_keys_on_stable/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_04_edition_mismatch/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/rs_fmt_config_07_ignore_escape_hatch/rule.rs`
- `packages/rs/fmt/g3rs-fmt-config-checks/crates/test_support/src/input.rs`
- `packages/rs/fmt/g3rs-fmt-ingestion/crates/runtime/src/run.rs`
- new `run_tests` files under `packages/rs/fmt/g3rs-fmt-config-checks/crates/runtime/src/`
