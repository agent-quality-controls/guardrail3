# g3-cargo-content-checks TODO

## Boundary contract

The package input is parsed-file only:

- policy-root `Cargo.toml`
- member `Cargo.toml` files
- optional normalized cargo policy profile
- normalized cargo lint-allow waiver entries

The app still owns:

- file discovery
- owned-root classification
- missing-member detection
- parse-failure routing
- parsing and normalizing the root-local cargo policy config

## Rule split

Package-owned content rules:

- `RS-CARGO-01`
- `RS-CARGO-02`
- `RS-CARGO-03`
- `RS-CARGO-04`
- `RS-CARGO-05`
- `RS-CARGO-06`
- `RS-CARGO-07`
- `RS-CARGO-08`
- `RS-CARGO-09`
- `RS-CARGO-11`
- `RS-CARGO-12`
- `RS-CARGO-13`
- `RS-CARGO-15`

App-owned structural rules:

- `RS-CARGO-10`
- `RS-CARGO-14`

## Next step

After the package boundary is stable, extract the first cargo content rule set into small rule modules instead of wiring the whole legacy app runtime at once.
