# Contributing

- Keep MCU-specific code under `targets/`.
- Keep protocol and domain code independent of hardware crates.
- Add or update tests for behavior changes.
- Run `cargo fmt --all -- --check` and the relevant Cargo tests before opening a change.
- Never commit secrets, private keys, or generated credentials.
