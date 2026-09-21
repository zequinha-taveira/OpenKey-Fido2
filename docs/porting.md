# OpenKey FIDO2 - Porting Guide

## Overview

OpenKey FIDO2 uses a strict Hardware Abstraction Layer (`hal`) decoupling the core authenticator, cryptography, and protocol handling from specific microcontrollers.

## Steps to Port to a New Target MCU

1. **Implement HAL Traits**:
   Create a new directory under `targets/<target-mcu>/` and implement the traits from `hal`:
   - `HardwareRng`: TRNG source with sufficient hardware entropy.
   - `FlashStorage`: Sector erase, page read, and write operations.
   - `HardwareTimer`: Monotonic timer and precision delays.
   - `InputPin` / `OutputPin`: Touch sensor / button inputs and LED indicators.
   - `SecureElement` *(Optional)*: Hardware secure cryptoprocessor if available.

2. **USB HID Stack Integration**:
   - Provide a CTAPHID compliant endpoint (64-byte IN/OUT interrupt transfers).
   - Wire packet transmission directly to `services/transport`.

3. **Linker & Memory Configuration**:
   - Configure RAM allocations (recommended >= 32KB SRAM for buffers and CBOR decode).
   - Reserve flash sectors for credential storage, state flags, and persistent counters.

4. **Verify Conformance**:
   - Run test suite under `tests/` with the target emulator or test harness.
