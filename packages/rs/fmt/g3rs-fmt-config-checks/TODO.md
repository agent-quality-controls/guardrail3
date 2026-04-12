# g3rs-fmt-config-checks TODO

- Keep the package lane-pure:
  - config content only
  - no file discovery
  - no placement logic
- Preserve blocker-state coverage for:
  - malformed root rustfmt config
  - missing or malformed `Cargo.toml`
  - missing or malformed `rust-toolchain.toml`
