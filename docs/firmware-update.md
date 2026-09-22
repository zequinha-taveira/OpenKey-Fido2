# OpenKey FIDO2 - Firmware Update Guide

## Overview

Firmware updates on OpenKey authenticators must be secure, atomic, and authenticated to prevent unauthorized code execution or downgrade attacks.

## Update Modes

### 1. DFU / BOOTSEL Mode (RP2040 / RP2350)
- The device can be rebooted into ROM/UF2 bootloader mode via `openkey-manager` or physical button hold during insertion.
- The new signed binary package is mounted as a mass-storage drive or streamed via vendor-specific HID commands.

### 2. Dual-Bank A/B Swap
- For targets with dual-bank internal flash (e.g. STM32, RP2350 with secure boot):
  - Bank A runs active firmware.
  - Bank B receives staged update image.
  - Image signature and version monotonicity are validated before booting the new slot.

## Security Constraints

- **Rollback Prevention**: Monotonically increasing firmware version counters prevent downgrading to versions with known vulnerabilities.
- **Credential Preservation**: Sensitive keys and resident credentials stored in protected flash/SE sectors remain intact during upgrades.
- **Cryptographic Signature**: Only firmware signed by authorized OpenKey release keys will be accepted by the bootloader.
