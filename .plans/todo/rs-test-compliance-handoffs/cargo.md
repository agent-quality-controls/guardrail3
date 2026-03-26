# RS-TEST Compliance Handoff: `rs/cargo`

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/cargo`

Current state:
- `RS-ARCH`: passes cleanly on the family root
- `RS-TEST`: `31 errors, 0 warnings`
- dominant failures:
  - `RS-TEST-02`
  - `RS-TEST-03`

Read first:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/cargo/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/cargo/src/test_support.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`

Do:
1. Turn `families/cargo/Cargo.toml` into a workspace with members:
   - `crates/runtime`
   - `crates/assertions`
   - `test_support`
2. Move all production code from `src/` into `crates/runtime/src/`.
3. Keep one rule file per rule and one `*_tests/mod.rs` directory per rule under `crates/runtime/src/`.
4. Create `crates/assertions/src/lib.rs` and add assertion modules for the family’s test helpers.
5. Split current `src/test_support.rs`:
   - keep generic fixture constants, tree builders, and route helpers in `test_support`
   - move semantic result assertions out of `test_support` into `crates/assertions`
6. Update `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/Cargo.toml` so `guardrail3-app-rs-family-cargo` points at `families/cargo/crates/runtime`.

Expected result:
- family tests pass
- `RS-ARCH` on the family root stays clean
- `RS-TEST` on the family root becomes `0 errors, 0 warnings, 0 info`

Verify with:
```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-cargo
cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/cargo --family arch --inventory --format json
cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/cargo --family test --inventory --format json
```
