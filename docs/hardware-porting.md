# Hardware Porting

Para levar `device/` a um novo MCU (ex. `targets/novo-board/`):

1. Implemente `crates/openkey-hal`: `FlashDevice`, `UsbHidDevice`/`UsbCcidDevice`/`NfcDevice`/`BleGattDevice`, `UserPresenceButton`, `UserVerificationDevice`.
2. Registre heap (`embedded-alloc`), clocks e `getrandom(custom)` no `main.rs`.
3. Crie o perfil em `crates/openkey-device/src/profile` (AAGUID, USB VID:PID, transports, `BoardDefinition`).
4. Componha via `EmbeddedAuthenticator::new_with_board` ou
   `new_with_profile_and_transport(Box::new(seu_transport))`.
5. Valide: `cargo check --target <thumbv*>`, `just build-<board>`, enumeração USB,
   `GetInfo`, `hardware_check.py`, runbook em `docs/hardware/`.
6. Referências: `docs/hardware/rp2350-zero-validation.md`, `ADR-0011`
   (targets/no_std), `ADR-0009` (CTAPHID), `ADR-0016` (flash/gates de release).
