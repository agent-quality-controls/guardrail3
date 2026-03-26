# RS-TEST Compliance Handoff: `rs/arch`

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch`

Current state:
- historical starting point for this handoff:
  - `RS-ARCH`: passed cleanly on the family root
  - `RS-TEST`: `18 errors, 13 warnings`
  - dominant failures:
    - `RS-TEST-02`
    - `RS-TEST-03`
    - `RS-TEST-07`
- current refactor status on this branch:
  - `families/arch` is now a local workspace with:
    - `crates/runtime`
    - `crates/assertions`
    - `test_support`
  - production code moved under `crates/runtime/src`
  - semantic result assertions moved into `crates/assertions`
  - generic tree/fixture helpers moved into `test_support`
  - local proof passes:
    - `cargo test --manifest-path apps/guardrail3/crates/app/rs/families/arch/Cargo.toml --workspace --quiet`
  - product-level `guardrail3 rs validate ... --family {arch,test}` proof is still blocked by unrelated broken manifests in other families under the main `apps/guardrail3` workspace

Read first:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/crates/assertions/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/arch/test_support/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`

Do:
1. Turn `families/arch/Cargo.toml` into a workspace with members:
   - `crates/runtime`
   - `crates/assertions`
   - `test_support`
2. Move all production code from `src/` into `crates/runtime/src/`.
3. Keep one rule file per rule and one `*_tests/mod.rs` directory per rule under `crates/runtime/src/`.
4. Create `crates/assertions/src/lib.rs` plus one assertion module per rule as needed.
5. Split current `src/test_support.rs`:
   - keep only generic tree/fixture/route helpers in `test_support`
   - move semantic result assertions out of `test_support` into `crates/assertions`
6. Rewrite sidecar tests so proof-bearing assertions live in the assertions crate, not inline in the tests.
7. Update `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/Cargo.toml` so `guardrail3-app-rs-family-arch` points at `families/arch/crates/runtime`.

Expected result:
- family tests pass
- `RS-ARCH` on the family root stays clean
- `RS-TEST` on the family root becomes `0 errors, 0 warnings, 0 info`

Verify with:
```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-arch
cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/arch --family arch --inventory --format json
cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/arch --family test --inventory --format json
```
