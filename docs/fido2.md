# OpenKey FIDO2 / WebAuthn Protocol Guide

## Specifications Implemented

- **FIDO CTAP 2.1**: Client to Authenticator Protocol 2.1 standard.
- **W3C Web Authentication (WebAuthn) Level 2 & Level 3**: Browser authentication API.
- **CTAP 2.0 Backward Compatibility**: Full support for legacy FIDO2 authenticators.
- **U2F (CTAP1)**: Backward interoperability for older services.

## Supported Commands

| Command | Opcode | Description |
| :--- | :---: | :--- |
| `authenticatorMakeCredential` | `0x01` | Creates and registers a new credential bound to the RP. |
| `authenticatorGetAssertion` | `0x02` | Generates authentication assertion with signature and counters. |
| `authenticatorGetNextAssertion` | `0x08` | Fetches next credential assertion when multiple match. |
| `authenticatorGetInfo` | `0x04` | Reports authenticator capabilities, versions, algorithms, and limits. |
| `authenticatorClientPIN` | `0x06` | PIN establishment, token exchange, and verification under ECDH. |
| `authenticatorReset` | `0x07` | Factory reset, wiping all stored credentials within 10 seconds of boot. |
| `authenticatorSelection` | `0x0B` | Visual/physical indication for multi-device environments. |
| `authenticatorLargeBlobs` | `0x0C` | Read/write persistent opaque data arrays. |
| `authenticatorConfig` | `0x0D` | Enterprise attestation and configuration toggles. |

## Cryptographic Algorithms

- **COSE Algorithm `-7` (ES256)**: ECDSA with SHA-256 over NIST P-256.
- **COSE Algorithm `-8` (EdDSA)**: Ed25519 (optional/profile dependent).
- **Key Agreement**: ECDH over P-256 (COSE type 2, curve 1) for CTAP2 PIN/UV protocol 1 and protocol 2.
