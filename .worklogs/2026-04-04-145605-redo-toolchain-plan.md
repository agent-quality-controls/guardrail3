# Redo toolchain extraction plan — content checks only

**Date:** 2026-04-04 14:56

## Summary
Rewrote toolchain extraction plan. Content checks only (rules 02, 03)
move to package. Filetree checks (rules 01, 04) stay in app. Input
uses parsed types (toml::Value for toolchain, cargo_toml::Manifest
for Cargo.toml). No raw strings, no booleans, no pre-extracted fields.
