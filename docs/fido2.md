# OpenKey FIDO2 / WebAuthn — matriz de estado e evidências

Este documento é a referência de **status documental** do suporte FIDO2. Ele separa
quatro afirmações que não são equivalentes:

1. **Implementado** — há código, interface ou adaptador correspondente no
   repositório. Isso não prova interoperabilidade.
2. **Validado no host** — o comportamento foi exercitado por testes Rust/Python,
   mocks, simulador ou UHID virtual. Não envolve uma placa nem um rádio real.
3. **Validado em hardware** — o fluxo foi executado e passou em uma placa física,
   leitor/radio e caminho de transporte reais, com evidência registrada.
4. **Validado pelo FIDO Conformance Tool** — a suíte oficial da FIDO Alliance foi
   executada e o relatório foi arquivado no repositório.

> **Regra de honestidade:** o simulador, `python-fido2`, `opensc-tool`, a ponte
> UHID e `tools/hardware_check.py` são instrumentos de desenvolvimento/diagnóstico.
> Nenhum deles transforma um teste de simulador em conformance oficial.

## Resumo atual

- O núcleo CTAP2, o armazenamento de host e os adaptadores de framing têm
  implementação e cobertura de host no repositório.
- A cobertura de host inclui o simulador JSON/raw-CBOR, mocks e UHID virtual. Ela
  não é uma validação de USB, NFC, BLE, CCID ou UV físico.
- **UV real, NFC, BLE e a validação física ponta a ponta permanecem pendentes.**
  O runbook registra uma sondagem física parcial (HID FIDO não enumerado e CCID
  sem ATR), não um resultado aprovado.
- **Nenhuma execução do FIDO Conformance Tool está validada ou arquivada.** Não
  existe relatório oficial em `docs/conformance/`.

## Matriz objetiva

| Área / protocolo | Implementado | Validado no host | Validado em hardware | Validado pelo FIDO Conformance Tool | Evidência e comando de referência |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **CTAP2 / CTAP2.1-alvo** — GetInfo, MakeCredential, GetAssertion, GetNextAssertion, ClientPIN, Reset, Credential Management, LargeBlobs, `authenticatorConfig` | **Sim, como handlers e wire path do projeto** | **Sim, como suíte interna do projeto**; isso não certifica a especificação completa | **Não**; o runbook físico ainda tem gates abertos | **Não — pendente** | Implementação: `crates/openkey-core/src/ctap2/` e `core/ctap2/`. Host: `cargo test --workspace`; `PYTHONPATH=targets/simulator/python:tests/python python -m pytest tests/python -v`; `python -m pytest tests/python/conformance/ -v`. A ressalva de wire format está em `docs/adr/ADR-0013-conformidade-ctap2-codigos-erro-e-chaves-inteiras.md`. |
| **WebAuthn / cerimônia no navegador** | **Sim, núcleo de cerimônia e estruturas** | **Parcial**: há fluxo virtual/ponte e testes de núcleo; o checklist manual de navegador ainda está aberto | **Não** | **Não — pendente** | Implementação: `core/webauthn/` e `crates/openkey-core/src/webauthn/`. Ponte: `tools/ctaphid_bridge.py --self-test`. Checklist de navegador: `docs/hardware/Linux-UHID.md` (seção 3). |
| **CTAPHID / USB-HID** | **Sim**: framing no transporte e ponte UHID | **Sim, virtual**: fragmentação, `INIT`, `PING`/CBOR e GetInfo | **Não**; a última sondagem física documentada não encontrou HID FIDO enumerado | **Não — pendente** | Implementação: `crates/openkey-transport/src/hid/` e `tools/ctaphid_bridge.py`. Testes: `python -m pytest tests/python/test_ctaphid_bridge.py -v`; `PYTHONIOENCODING=utf-8 python tools/ctaphid_bridge.py --self-test`. Hardware: `python tools/hardware_check.py --json` e `docs/hardware/rp2350-zero-validation.md` (seções 5–6). |
| **CCID / ISO 7816-4 / applets** | **Sim, em adaptadores, roteador e applets do projeto** | **Sim, no simulador/mocks**; cobre APDU, applets e persistência host | **Não**; não há aprovação física de ATR, PC/SC e SELECT ponta a ponta | **Não — não é o escopo de conformance CTAP** | Implementação: `crates/openkey-transport/src/ccid/`, `crates/openkey-transport/src/embedded/usb_ccid*` e `firmware/authenticator/`. Host: `python -m pytest tests/python/test_piv_openpgp_e2e.py tests/python/test_ctap2_commands.py`; `cargo test --workspace`. Diagnóstico físico: `python tools/hardware_check.py --json`, `opensc-tool -l` e `opensc-tool -a`; evidência atual no runbook registra `SCARD_W_REMOVED_CARD`/ATR nulo. |
| **NFC / ISO 14443** | **Parcial**: framing/APDU e contrato `NfcDevice`; não há stack de rádio no repositório | **Sim, somente para adaptador/mocks**, quando o crate é testado com a feature correspondente | **Não — rádio e validação física pendentes** | **Não — pendente** | Implementação: `crates/openkey-transport/src/nfc/` e `crates/openkey-transport/src/embedded/nfc.rs`. Host: `cargo test -p openkey-transport` (incluindo a feature aplicável). Limite explícito: `crates/openkey-transport/src/nfc/framed_nfc.rs` declara “Host-only: sem rádio”. |
| **BLE GATT** | **Parcial**: framing e contrato `BleGattDevice`; não há SoftDevice/NimBLE nem rádio concreto | **Sim, somente para framing/adaptador/mocks** | **Não — stack e validação física pendentes** | **Não — pendente** | Implementação: `crates/openkey-transport/src/bluetooth/` e `crates/openkey-transport/src/embedded/ble_gatt.rs`. Host: `cargo test -p openkey-transport` (incluindo a feature aplicável). O módulo `framed_ble.rs` identifica o modelo como host-only, sem SoftDevice/NimBLE. |
| **PIN / `pinUvAuth` / UV** | **PIN e protocolos 1/2 implementados; UV embutido real não** | **Sim para PIN e rejeições de UV no simulador** | **Não**; não existe sensor biométrico/UV real validado | **Não — pendente** | Implementação: `crates/openkey-core/src/ctap2/client_pin.rs` e `hmac_secret.rs`. Host: `python -m pytest tests/python/conformance/test_client_pin.py tests/python/conformance/test_hmac_secret.py -v`. O runbook confirma que `uv` não é anunciado e que `getUVRetries` segue indisponível. |
| **Armazenamento / persistência** | **Sim** para backend simulado/host e adaptador QSPI RP2350 | **Sim para power-loss e reinício no host** | **Não**; persistência após power-cycle real continua no checklist | **Não — pendente** | Implementação: `crates/openkey-storage/` e `examples/rp2350-firmware/src/qspi_flash.rs`. Host: `cargo test --workspace` e `python -m pytest tests/python/conformance/test_persistence_restart.py -v`. Hardware: `docs/hardware/rp2350-zero-validation.md` (seção 7), ainda sem todos os itens marcados. |

### Como interpretar a matriz

“**Implementado**” pode coexistir com “**Validado no host**” e “**Não validado
em hardware**”. Do mesmo modo, um adaptador NFC/BLE pode ter testes de framing
sem que exista uma pilha de rádio pronta. Um teste verde do simulador demonstra
uma propriedade do código executado no host; não demonstra presença física,
latência, enumeração USB, ATR/PC-SC, campo NFC, conexão BLE, toque/biometria ou
conformidade normativa.

## Evidências por estado

### 1. Implementado

Os pontos de entrada principais são:

- CTAP2/WebAuthn: `crates/openkey-core/src/ctap2/`, `core/ctap2/` e
  `crates/openkey-core/src/webauthn/`.
- CTAPHID: `crates/openkey-transport/src/ctaphid/` e
  `crates/openkey-transport/src/hid/`.
- CCID/NFC/BLE: `crates/openkey-transport/src/ccid/`, `src/nfc/` e
  `src/bluetooth/`; os módulos host-only deixam explícito quando só há framing
  ou contrato de dispositivo.
- Simulador: `targets/simulator/`, com protocolo JSON por linhas e
  `--raw-cbor`.
- Decisões e limites: `docs/adr/ADR-0009-ctaphid-framing-e-hardware-transports.md`,
  `docs/adr/ADR-0012-fido-conformance-e-raw-cbor-interface.md` e
  `docs/adr/ADR-0021-authnr-config-e-gates-de-hardware.md`.

### 2. Validado no host

Com dependências instaladas, os comandos de referência são:

```sh
cargo test --workspace
cargo build -p fido2-simulator

PYTHONPATH=targets/simulator/python:tests/python \
  python -m pytest tests/python -v
python -m pytest tests/python/conformance/ -v

python -m pytest tests/python/test_ctaphid_bridge.py -v
PYTHONIOENCODING=utf-8 python tools/ctaphid_bridge.py --self-test
```

Esses comandos exercitam código no host, simulador, mocks ou UHID virtual. A
suíte `tests/python/conformance/` é uma suíte **interna de desenvolvimento**:
ela não substitui a ferramenta oficial e deve ser lida junto com a ressalva do
ADR-0013 sobre chaves CBOR/códigos de erro ainda em revisão.

### 3. Validado em hardware

O procedimento físico está em
[`docs/hardware/rp2350-zero-validation.md`](hardware/rp2350-zero-validation.md).
Os comandos de evidência, que só devem ser classificados como “passou” quando
executados em uma placa e com a saída arquivada, incluem:

```sh
# diagnóstico pós-flash (HID FIDO + CTAP2 + CCID/PC-SC + SELECT)
python tools/hardware_check.py --json

# ferramentas neutras no caminho CCID/ISO 7816
opensc-tool -l
opensc-tool -a
opensc-tool -s 00:a4:04:00:07:a0:00:00:05:27:21:01

# identidade/enumeração opcional do build privado 1050:0407
ykman list --serials
```

No estado documental atual, o runbook ainda marca os passos físicos como
pendentes. A tabela de diagnóstico registra uma execução parcial com
`HID FIDO: nenhum enumerado`, `ATR: null` e `connect_error: 0x80100066`
(`SCARD_W_REMOVED_CARD`). Isso é evidência de uma falha/gap diagnosticado, não
um “hardware validado”. Permanecem pendentes, em particular:

- enumeração e resposta CTAPHID em USB-HID físico;
- ATR, PC/SC e APDU SELECT em CCID físico;
- persistência após power-cycle real;
- campo/leitor NFC e rádio BLE;
- **UV real** (sensor/biometria e validação física do fluxo).

Não alterar `tools/hardware_check.py` para transformar uma sonda em aprovação;
o script é uma ferramenta de diagnóstico e o próprio runbook é a evidência
primária do gate físico.

### 4. Validado pelo FIDO Conformance Tool

**Estado: pendente.** A ponte prevista para uma futura execução é:

```sh
cargo build -p fido2-simulator
sudo python tools/ctaphid_bridge.py \
  --simulator ./target/debug/fido2-simulator
```

Depois seria necessário obter acesso à ferramenta oficial da FIDO Alliance,
configurá-la para o autenticador HID virtual/real, executar a suíte pertinente
e arquivar o relatório em `docs/conformance/`. A seção 4 de
[`docs/hardware/Linux-UHID.md`](hardware/Linux-UHID.md) é um **runbook de
integração**, não um relatório de execução. Até que exista esse relatório:

- não chamar a suíte Python de “conformance oficial”;
- não chamar o simulador de autenticador certificado;
- não declarar CTAP2/WebAuthn, USB, CCID, NFC, BLE ou UV como aprovados no
  FIDO Conformance Tool.

## Referências de manutenção

- `README.md` — resumo público e comandos de desenvolvimento.
- `STATUS_ATUAL_PROJETO.md` e `TODO.md` — histórico de critérios e gates.
- `docs/adr/ADR-0013-conformidade-ctap2-codigos-erro-e-chaves-inteiras.md` —
  ressalvas de wire format que impedem equiparar testes internos a conformance.
- `docs/adr/ADR-0016-flash-simulada-e-gates-de-release.md` — separação entre
  flash simulada, validação física e gate de release.
- `docs/hardware/Linux-UHID.md` — ponte virtual e passos futuros para o Tool.
- `docs/hardware/rp2350-zero-validation.md` — checklist e diagnóstico físico.
