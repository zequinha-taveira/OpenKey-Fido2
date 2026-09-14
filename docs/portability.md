# Portability

`crates/openkey-core`, `crates/openkey-hal` e `crates/openkey-transport` são
`no_std + alloc`:

- `cfg_attr(not(feature = "std"), no_std)` em `crypto`, `ctap2`, `webauthn`,
  `storage`, `authenticator`, `transport`.
- `cargo check -p <crate> --no-default-features` no host simula o firmware.
- Targets: `thumbv8m.main-none-eabihf` (RP2350) e `thumbv7em-none-eabihf`
  (nRF52840/STM32L4); heap via `embedded-alloc` no `main.rs` do target.
- `FileStorageBackend` e `InsecureHostStorage` apenas com `feature = "std"`.
- `ring` + `SystemRandom` no host; no ARM sem OS o target registra
  `getrandom(custom)`. O `[patch.crates-io] ring` vendored vive apenas no
  target RP2350, não na raiz.
- `just check-targets` valida `transport --features embedded --no-default-features`
  nos dois targets.
