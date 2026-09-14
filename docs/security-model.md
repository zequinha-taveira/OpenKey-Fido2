# Security Model

- Sem `unsafe` sem ADR; sem log de chaves, seeds, PINs.
- Chaves privadas com `zeroize` no drop; comparação em tempo constante
  (`constant_time_eq`); nonces via `SystemRandom`.
- Encryption at rest (ChaCha20-Poly1305); `CredRandomWithUV/WithoutUV` só para
  requests autenticados; flag UV (0x04) apenas com verificador real.
- PIN: retries persistentes, bloqueio após 3 falhas (`PIN_AUTH_BLOCKED`),
  `pinUvAuthParam` com `permissions/rpId`; Reset exige presença física e
  invalida tokens/segredos.
- `LargeBlobs` com bounds contra DoS; `hmac-secret` conforme CTAP 2.1 §12.5
  (saltAuth verificado, IV fresco).
- Ver `SECURITY.md` e `docs/adr/ADR-0006-side-channel-mitigation.md`.
