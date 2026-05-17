# guardrail3

[![license](https://img.shields.io/github/license/agent-quality-controls/guardrail3)](LICENSE)
[![rust](https://img.shields.io/badge/rust-stable-orange)](rust-toolchain.toml)
[![ci](https://img.shields.io/github/actions/workflow/status/agent-quality-controls/guardrail3/ci.yml?branch=main&label=ci)](https://github.com/agent-quality-controls/guardrail3/actions/workflows/ci.yml)
[![issues](https://img.shields.io/github/issues/agent-quality-controls/guardrail3)](https://github.com/agent-quality-controls/guardrail3/issues)

Composable code guardrails for Rust and TypeScript projects.

## Install

```bash
cargo install guardrail3
```

## Quick start

```bash
guardrail3 rs init --profile service .
guardrail3 rs generate
guardrail3 rs validate .
```

Run `guardrail3 --help` for all commands, profiles, topology conventions, config reference, and the full check inventory. Or `guardrail3 dump-guide` to produce a comprehensive Markdown guide.

## More

- [Philosophy](https://github.com/agent-quality-controls/guardrail3/wiki/Philosophy) — why banned-by-default, why self-validating, why Rust.
- [Comparison](https://github.com/agent-quality-controls/guardrail3/wiki/Comparison) — guardrail3 vs ArchUnit, clippy, cargo-deny, dependency-cruiser, ts-prune.
- [Checks](https://github.com/agent-quality-controls/guardrail3/wiki/Checks) — what guardrail3 checks (config, source scan, dependency allowlists, apparch, release readiness, test quality, validation).
- [Quick start](https://github.com/agent-quality-controls/guardrail3/wiki/Quick-Start) — Rust and TypeScript bootstrap walkthroughs.
- [Layout](https://github.com/agent-quality-controls/guardrail3/wiki/Layout) — active workspace layout and legacy archive.
- [Thanks](https://github.com/agent-quality-controls/guardrail3/wiki/Thanks) — primary inspiration, wrapped tools, and acknowledgments.
- [Contributing](.github/CONTRIBUTING.md) — open a detailed issue first; PRs must pass the G3RS pre-commit gate.

---

Part of [Agent Quality Controls](https://github.com/agent-quality-controls).

## License

MIT
