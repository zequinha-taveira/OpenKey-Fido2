# Porting Guide

1. Add the board-specific implementation under `targets/<board>/`.
2. Implement or adapt the contracts from `crates/openkey-hal`.
3. Keep protocol and credential logic in `crates/openkey-core`.
4. Select transports through `crates/openkey-transport` without importing a
   concrete MCU into the core crates.
5. Add a host-testable board profile before adding target-only startup code.

Hardware register access, linker scripts, interrupt handlers, and flash drivers
belong only in a target crate. Secure storage and randomness must be injected
through HAL or storage contracts rather than accessed from the core.
