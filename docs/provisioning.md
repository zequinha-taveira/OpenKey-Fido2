# OpenKey FIDO2 - Provisioning Guide

## Overview

Device provisioning is the process of generating or injecting root attestation keys, batch certificates (AAGUID, X.509 certificates), and manufacturing configuration parameters into the authenticator before or during initial deployment.

## Provisioning Stages

1. **Entropy & Seed Initialization**:
   - TRNG calibration and entropy source validation.
   - Master key derivation and persistent root secret injection.

2. **Attestation Key & Certificate Enrollment**:
   - Injection of the device batch attestation private key (ECDSA P-256) into secure storage / Secure Element (SE).
   - Embedding corresponding X.509 attestation certificate and AAGUID.

3. **Factory Defaults Configuration**:
   - Initial PIN state (not configured).
   - Reset counter and signature counter initialization (`signCount = 0`).
   - Feature flags enabling/disabling (NFC, CCID, FIDO2, PIN length policy).

## Provisioning CLI & Tools

The `openkey-manager` suite will provide an automated provisioning workflow (currently under active development).
