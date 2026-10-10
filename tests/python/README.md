# Testes Python do workspace openkey-fido2

Suíte `pytest` que cobre, em camadas crescentes de fidelidade ao wire format:

| Arquivo | O que testa |
|---------|-------------|
| `test_examples.py` | Executa os binários `basic-example` e `ccid-example` (exit code, logs, AAGUID). |
| `test_firmware_sim.py` | Ciclos completos make/assert/verify contra o `fido2-simulator` (protocolo JSON por linha). |
| `test_algorithms.py`, `test_client_pin.py`, `test_extensions.py` | Casos específicos via simulador. |
| `test_virtual_authenticator.py` | Autenticador virtual em processo (`openkey-core`), falando **CTAP2 real sobre CBOR**. |

## Como rodar

A partir da raiz do workspace, a suíte completa usa o simulador Rust, o
wheel `openkey_core` e dois diretórios no `PYTHONPATH`:

```bash
python -m pip install -r tests/python/requirements.txt
python -m pip install maturin
python -m maturin build --release \
  --manifest-path python/openkey_core/Cargo.toml \
  --interpreter python --out /tmp/openkey-wheels
python -m pip install --force-reinstall /tmp/openkey-wheels/*.whl

cargo build -p fido2-simulator
PYTHONPATH=targets/simulator/python:tests/python \
  python -m pytest tests/python -v
```

No Windows, use `set PYTHONPATH=targets\simulator\python;tests\python` ou o
equivalente do PowerShell. Em shells Unix, `PYTHONPATH` é separado por `:`;
no Windows, por `;`.

O comando `cargo build -p fido2-simulator` é necessário para os testes de
CTAP2/conformance que iniciam o simulador local. Sem ele, a coleta pode
terminar com `FileNotFoundError` mesmo que o módulo `openkey_core` esteja
instalado. O baseline observado no Sandbox Linux foi **332 passed, 5 skipped**.

## Autenticador virtual (`openkey-core`)

Os testes de `test_virtual_authenticator.py` dependem do wheel
`openkey_core`, que embrulha `EmbeddedAuthenticator` (o mesmo núcleo Rust
que compila para firmware) via pyo3/maturin. Para (re)compilar e instalar
após mudanças em Rust:

```bash
python -m maturin build --manifest-path python/openkey_core/Cargo.toml --interpreter python
python -m pip install --force-reinstall python/openkey_core/target/wheels/openkey_core-*.whl
```

`maturin develop` exige um venv; neste ambiente use `maturin build` + `pip
install` do wheel. A ponte `virtualauthenticator.py` encoda requests CTAP2
com `fido2.cbor`, chama `process_command(cmd, data)` e decodifica as
respostas com os helpers de `fido2.webauthn` (`AttestationObject`,
`AuthenticatorData`).

## Observações

- Os exemplos são pacotes do workspace (`basic-example` e `ccid-example`),
  não alvos `--example`; `cargo run --example basic` não funciona aqui. Os
  testes executam o binário compilado em `target/debug/<pacote>.exe`
  (opção preferida) e, se não existir, compilam com
  `cargo build -p basic-example -p ccid-example` (fixture de sessão).
- Os logs dos exemplos vão para o stderr; a execução define `RUST_LOG=info`.
- O AAGUID é validado pela representação Debug dos bytes (ex. `[170, 187,
  17, ...]`), pois os exemplos imprimem o array, não o hex literal.
- No Windows com Smart App Control ativo, binários sem assinatura podem ser
  bloqueados pelo SO; nesse caso os testes são pulados (skip) com uma
  mensagem indicando o motivo.
