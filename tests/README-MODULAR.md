# tests/ — nova organização (incremental)

| Pasta nova | Conteúdo atual |
|---|---|
| `tests/unit/` | testes `#[cfg(test)]` dentro de cada crate (`protocol/*`, `firmware/*`) |
| `tests/integration/` | `tests/src/lib.rs` (crate `test-suite`) |
| `tests/interoperability/` | `tests/python/test_*.py` (simulador JSON + `python/openkey_core`) |
| `tests/conformance/` | `tests/python/conformance/` (wire CTAP 2.1 raw CBOR) + `tools/ctaphid_bridge.py` |

Nada foi movido neste incremento; as pastas novas guardam o layout alvo.
