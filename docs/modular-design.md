# Modular Design

The repository is split into MCU-independent crates and hardware targets:

```text
openkey-device
    ├── openkey-core       (FIDO2, CTAP2, WebAuthn, domain state)
    ├── openkey-transport  (HID, CCID, NFC, BLE, CTAPHID)
    ├── openkey-storage    (credential and secure storage)
    ├── openkey-crypto     (cryptographic primitives)
    └── openkey-hal        (hardware contracts)
             │
             └── targets/* (generic, RP2040, RP2350, STM32, simulator)
```

The dependency direction is `openkey-device` to the core, storage, transport,
and HAL crates. Target crates may know a board and its registers, but
`openkey-core` must not know an MCU. Hardware access is injected through HAL,
storage, and transport contracts.

## Extension Points

- Add protocol behavior in `crates/openkey-core`.
- Add cryptographic providers or algorithms in `crates/openkey-crypto`.
- Add persistence backends in `crates/openkey-storage`.
- Add framing or physical transports in `crates/openkey-transport`.
- Add MCU adapters only in `targets/<board>` through `openkey-hal` contracts.
