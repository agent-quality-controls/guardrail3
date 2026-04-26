# Summary

Installed the current local `g3ts` binary after the Astro policy changes and committed the missed `apps/guardrail3-ts/Cargo.lock` dependency updates. The installed binary now reports the new Astro strict policy rule on the landing app.

# Decisions

- Used `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force` so the installed CLI exactly matches the checked lockfile.
- Kept the lockfile update because recent Astro ingestion/config checks added `globset` and the shared `guardrail3-rs-toml-parser` dependency to the G3TS workspace.

# Key Files

- `apps/guardrail3-ts/Cargo.lock`
- `apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime/Cargo.toml`

# Verification

- `cargo install --path apps/guardrail3-ts/crates/io/inbound/cli/crates/runtime --locked --force`
- `g3ts validate --path /Users/tartakovsky/Projects/websmasher/websmasher/apps/landing --family astro --inventory`

# Next Steps

- Every future G3TS source change should finish with local `g3ts` reinstall before reporting results.
