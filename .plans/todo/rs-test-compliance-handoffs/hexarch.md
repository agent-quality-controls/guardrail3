# RS-TEST Compliance Handoff: `rs/hexarch`

Owner root: `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch`

Current state:
- `RS-ARCH`: passes cleanly on the family root
- `RS-TEST`: `58 errors, 50 warnings`
- dominant failures:
  - `RS-TEST-02`
  - `RS-TEST-03`
  - `RS-TEST-07`

Read first:
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/src/test_support.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/hexarch/src/dependency_facts.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/Cargo.toml`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/crates/assertions/src/lib.rs`
- `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/families/test/test_support/src/lib.rs`

Do:
1. Turn `families/hexarch/Cargo.toml` into a workspace with members:
   - `crates/runtime`
   - `crates/assertions`
   - `test_support`
2. Move all production code from `src/` into `crates/runtime/src/`.
3. Keep one rule file per rule and one `*_tests/mod.rs` directory per rule under `crates/runtime/src/`.
4. Create `crates/assertions/src/lib.rs` and add proof-bearing assertion modules for rule-specific tests.
5. Split current `src/test_support.rs`:
   - keep generic real-filesystem fixture helpers, tempdir helpers, and tree builders in `test_support`
   - move semantic result assertions and proof sites out of `test_support` into `crates/assertions`
6. Keep `dependency_facts.rs` in runtime if it is runtime discovery/facts logic; only move test-only semantics out.
7. Update `/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/Cargo.toml` so `guardrail3-app-rs-family-hexarch` points at `families/hexarch/crates/runtime`.

Expected result:
- family tests pass
- `RS-ARCH` on the family root stays clean
- `RS-TEST` on the family root becomes `0 errors, 0 warnings, 0 info`

Verify with:
```bash
cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-hexarch
cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/hexarch --family arch --inventory --format json
cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/hexarch --family test --inventory --format json
```
