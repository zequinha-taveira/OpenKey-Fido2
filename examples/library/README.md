# examples/ — índice

| Exemplo | Pacote | Mostra |
|---|---|---|
| `basic/` | `basic-example` | Facade `EmbeddedAuthenticator` (padrão de integração) |
| `ccid/` | `ccid-example` | Transporte CCID |
| `crypto/` | `crypto-example` | `CryptoEngine` (Ed25519, ES256, ECIES, HMAC, SHA-256) |
| `ctap/` | `ctap2-example` | CTAP2 direto (`MakeCredential`, `GetAssertion`, `Reset`) |
| `storage/` | `storage-example` | Armazenamento seguro de credenciais |
| `transport/` | `transport-example` | `Transport` customizado (`LoggingTransport` como `Box<dyn Transport>`) |
| `webauthn/` | `webauthn-example` | Registro/autenticação via `WebAuthnAuthenticator` |
| `library/` | — | Este índice: como consumir `core/` como dependência externa (`openkey-*` via `path`/`version`) |

```bash
cargo run -p basic-example
cargo run -p webauthn-example
```
