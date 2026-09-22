# OpenKey FIDO2 - Commissioning Guide

## Overview

Commissioning refers to the final factory verification and production line testing performed on assembled hardware units before packaging and distribution.

## Commissioning Checklist

1. **Hardware Verification**:
   - Power rail measurement and USB D+/D- signal integrity.
   - Touch sensor capacitance threshold calibration and sensitivity testing.
   - Status LED PWM testing across all channels (Red, Green, Blue).

2. **Firmware Integrity & Self-Tests**:
   - Cryptographic KATs (Known Answer Tests): AES-CBC, SHA-256, HMAC, ECDSA P-256, Ed25519.
   - Flash filesystem consistency check and wear-leveling allocation verification.
   - Secure Element communication ping and slot access control lockdown.

3. **Production Lockdown**:
   - Flash write-protection and read-out protection (ROP / Debug Port Lockout) activation.
   - JTAG/SWD disablement or secure unlock key registration.
   - Verification of recovery bootloader integrity.
