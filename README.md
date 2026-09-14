# openkey-fido2

Open-source FIDO2 / WebAuthn authenticator firmware in Rust for embedded devices.

Portable `no_std`-ready core (CTAP2.1, crypto, credential storage) plus pluggable
transports (USB-HID, CCID, NFC, BLE), board profiles, host simulator, and
conformance test suites. No hardware required to develop and test.

- Version: `0.1.1`
- License: `MIT OR Apache-2.0` (`LICENSE-MIT`, `LICENSE-APACHE`)
- Rust: `1.85+`
- Repo: `https://github.com/zequinha-taveira/openkey-fido2`

## Features

- **CTAP 2.1**: MakeCredential / GetAssertion / GetInfo, ClientPIN (protocols 1+2),
  Credential Management (`0x0A`), LargeBlobs (`0x0C`), `authenticatorConfig` (`0x0D`),
  Reset, Selection, GetNextAssertion, `hmac-secret`, `credProtect`, `credBlob`,
  `minPinLength`, Enterprise Attestation
- **Crypto**: Ed25519, ES256, ES384, PS256, RS256, HMAC-SHA256, SHA-256,
  ChaCha20-Poly1305 (encryption at rest), ECIES (X25519 + HKDF + ChaCha20-Poly1305),
  constant-time comparisons, `zeroize` on key material
- **Storage**: encrypted credentials, monotonic sign counters, PIN hash,
  large blobs, flash-simulated backend with power-loss recovery, wear-leveling
  counters, LRU pruning
- **Transports**: unified `Transport` trait, CTAPHID framing/fragmentation,
  USB-HID backend via `usb-device`, CCID/NFC/BLE framed adapters
  (concrete radio/NFC stacks still need hardware validation)
- **Boards**: NRF52840, STM32L4, ESP32C3, RP2350, RP2350-Zero, GENERIC profiles
  with AAGUID, capabilities, and security features; BOOTSEL-as-user-presence
  on RP2350
- **Testing**: Rust unit/integration tests, Python E2E over simulator
  (JSON line protocol + `--raw-cbor` wire mode), CTAPHID bridge tooling

## Repository layout

```text
crates/       openkey-core, openkey-crypto, openkey-storage, openkey-transport,
              openkey-hal, openkey-device
targets/      generic, rp2040, rp2350, stm32, simulator
tests/        Rust suite + Python pytest (incl. conformance/)
examples/     basic, crypto, ctap, storage, transport, webauthn
tools/        ctaphid_bridge.py and helpers
docs/         architecture.md, modular-design.md, security-model.md, adr/
fuzz/         cargo-fuzz harness (decode_cbor, ctap2_dispatch, ctaphid_framing)
```

See `docs/architecture.md` and `docs/modular-design.md` for dependency rules
and extension points.

## Quickstart

Prerequisites: Rust `1.85+` (`rustup target add thumbv8m.main-none-eabihf`
for RP2350, `thumbv7em-none-eabihf` for STM32), Python `3.9+`
with `pytest` + `fido2` for E2E.

```sh
# Host build + tests
cargo build --workspace
cargo test --workspace

# Lint / format
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check

# Simulator (JSON line protocol, no hardware)
cargo run -p fido2-simulator

# Raw CBOR wire mode (used by conformance suite / bridge)
cargo run -p fido2-simulator -- --raw-cbor

# Python E2E + conformance
python -m pytest tests/python -v
python -m pytest tests/python/conformance -v

# Unified build script (Linux/macOS/WSL)
./build_openkey_fido2.sh --help
./build_openkey_fido2.sh --all
```

Windows: `build_openkey_fido2.bat` mirrors the `.sh` script.

## Firmware targets

```sh
# RP2350 (ELF + UF2 via picotool / elf2uf2-rs)
./build_openkey_fido2.sh --rp2350
./build_openkey_fido2.sh --rp2350-uf2 --release

# Embedded crate checks
cargo check -p openkey-transport --target thumbv8m.main-none-eabihf \
  --features embedded --no-default-features
cargo check -p openkey-transport --target thumbv7em-none-eabihf \
  --features embedded --no-default-features
```

Default USB identity is pid.codes `1209:0001`. A YubiKey-identity build
(`1050:0407`) exists behind `yubikey5-identity` for private testing only
(third-party VID/PID, not for distribution). Physical validation runbook:
`docs/hardware/rp2350-zero-validation.md`.

## Simulator protocol

`fido2-simulator` speaks newline-delimited JSON on stdin/stdout by default,
and length-prefixed binary CTAP2 (`--raw-cbor`) for wire-accurate testing.
Python helpers live in `tests/python/conformance/ctap2_transport.py`;
`tools/ctaphid_bridge.py` exposes the simulator as a virtual USB-HID
device (Linux/UHID) for browsers and conformance tools.

## Security notes

- Private keys and PIN material are never logged; `Debug` impls redact secrets.
- PIN retries: 8 initial, block after 3 consecutive failures, power-cycle recovery.
- Reset requires user presence and wipes credentials, PIN, blobs, and sessions.
- UV bit (`0x04`) is set only on real PIN/UV authentication, never on request alone.
- `docs/security-model.md` and `docs/adr/` (notably ADR-0006, ADR-0008,
  ADR-0016, ADR-0017, ADR-0021) record threat model and known hardware gaps:
  no real biometric sensor, no audited secure element, artifact signing and
  FIDO conformance-tool runs still pending.

## Docs

- `docs/architecture.md` — layers, contracts, MakeCredential/GetAssertion flows
- `docs/modular-design.md` — workspace crate map
- `docs/security-model.md`, `docs/portability.md`, `docs/porting-guide.md`
- `docs/adr/` — design decisions (side-channels, ECIES, flash, ClientPIN, …)
- `TODO.md` — build status and remaining hardware-gated items
- Per-layer notes: `docs/modular-design.md`, `targets/README.md`,
  `tests/python/README.md`

## Contributing

One logical change per commit/PR. New or modified code requires tests
(Rust unit + Python E2E where applicable), `cargo fmt` / `clippy` clean,
and docs updates for public API or protocol changes. Never commit secrets
or log sensitive material. See `TODO.md` conventions before starting.
