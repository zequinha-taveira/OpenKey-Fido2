# Loop de Melhoria de Segurança — Backlog da Próxima Iteração

## 1. Escopo e regra de evidência

Este documento consolida as seis auditorias independentes fornecidas para o repositório `/home/ubuntu/OpenKey-Fido2`. Ele é um **backlog**, não um relatório de conclusão.

**Regra deste ciclo:** nenhuma tarefa abaixo deve ser marcada como concluída somente porque existe código, um mock, uma declaração de perfil, um teste histórico ou uma instrução de runbook. Um item só pode mudar para `CONCLUÍDO` quando houver a evidência indicada no seu critério de aceite, com commit, ambiente, comando, saída e artefatos preservados.

### Situação de evidência na abertura do ciclo

- Não foi apresentada nenhuma falha adicional de auditoria (`Falhas de auditoria: []`). Isso não significa que os gates estejam aprovados.
- As auditorias são parcialmente divergentes quanto à disponibilidade de Rust: uma análise UV encontrou o toolchain 1.97.1 em caminho explícito e registrou testes focalizados aprovados; outras não encontraram `cargo`, `rustc` ou `rustup` no `PATH`. Portanto, **não há uma execução workspace-wide única e reproduzível a ser tratada como validação atual**.
- A análise UV registrou, especificamente, `cargo test -p openkey-core --lib uv` (28 passados), `cargo test -p openkey-device --lib uv` (2 passados) e `cargo test -p openkey-hal --lib` (3 passados), usando caminho explícito para o toolchain. Esses resultados são evidência focalizada daquela execução, não aprovação do workspace, hardware ou FIDO Conformance.
- Não há placa, leitor NFC, central BLE, debugger, fixture elétrico ou sensor biométrico UV conectado nas auditorias. Não houve validação física.
- Não houve execução autorizada do FIDO Conformance Tool nem relatório oficial. A suíte local e o simulador são regressão/interoperabilidade host, não certificação.
- `STATUS_ATUAL_PROJETO.md` já estava não rastreado antes das auditorias e não faz parte deste incremento.

### Estados usados neste backlog

| Estado | Significado |
|---|---|
| `PLANEJADO` | Tarefa priorizada, ainda sem evidência de execução. |
| `EM IMPLEMENTAÇÃO` | Há uma alteração em andamento, mas o critério de aceite ainda não foi demonstrado. |
| `BLOQUEADO-TOOLCHAIN` | Depende de Rust/Python/ferramentas instaladas e de uma execução reproduzível. |
| `BLOQUEADO-HARDWARE` | Depende de placa, periférico, instrumento ou fixture físico. |
| `BLOQUEADO-FIDO` | Depende de acesso/autorização e execução do FIDO Conformance Tool. |
| `CONCLUÍDO` | Reservado para quando o critério de aceite e a evidência exigida estiverem arquivados. Nenhum item deste documento está nesse estado na abertura. |

---

## 2. Tabela de prioridade

Os caminhos na tabela são relativos à raiz do repositório.

| ID | Prioridade | Frente | Tarefa | Arquivos candidatos | Estado inicial | Critério de aceite verificável | Bloqueios/dependências |
|---|---|---|---|---|---|---|---|
| P0-01 | P0 | UV host-only | Ampliar o `MockBoardUserVerification` somente em `cfg(test)` para simular sucesso, falha e quantidade de retries; testar a ponte board→CTAP2 sem inventar sensor. | `crates/openkey-device/src/authenticator/authenticator.rs` | `PLANEJADO` | Teste comprova que falha do board vira `Ctap2Error::UvBlocked`, `retries()` é encaminhado e `uv_support(true)` sem dispositivo não anuncia UV. Nenhum perfil de produção é habilitado. | Toolchain Rust para executar; decisão futura sobre erro de verificação versus indisponibilidade e fonte única de retries continua aberta. |
| P0-02 | P0 | CCID host-only | Fechar o contrato do adaptador legado e cobrir APDU curta/estendida, truncamento, `Le`, `SW` e resposta encadeada; adicionar regressão end-to-end com `MockUsbBus` e `CardRouter`. | `crates/openkey-transport/src/ccid/framed_ccid.rs`; `crates/openkey-transport/src/embedded/usb_ccid_backend.rs`; `crates/openkey-transport/src/iso7816.rs` | `PLANEJADO` | Teste sem hardware prova `IccPowerOn → ATR T=0 → SELECT AID → XfrBlock`, `SW 9000/6A82`, eco de `bSlot/bSeq`, multipacote de 64 bytes e preservação de erros. O caminho RP2350 permanece `CompositeUsbDevice/CcidClass + CardRouter`, sem instanciar backend USB concorrente. | Toolchain Rust; não confundir cobertura MockUsbBus com enumeração USB/PCSC física. |
| P0-03 | P0 | Diagnóstico e aceitação | Transformar `hardware_check.py` em contrato tri-state/quad-state por placa: `PASS`, `FAIL`, `BLOCKED`, `NOT_RUN`, com `diagnostic_code`, `expected`, `observed` e exit codes definidos. Filtrar dispositivo por `--board`, `--vid`, `--pid` e/ou `--path`, nunca usar silenciosamente o primeiro. | `tools/hardware_check.py`; `tests/python/test_hardware_check.py`; `tests/python/test_hardware_acceptance.py` (novo) | `PLANEJADO` | Fixtures cobrem ausência, múltiplos dispositivos, timeout, VID/PID errado, GetInfo divergente, ATR inválido, erro PC/SC, `SW` incorreto e cada exit code: `0` somente todos os obrigatórios `PASS`; `1` falha observada; `2` pré-condição bloqueada; `3` configuração inválida. Mock nunca gera `PASS` físico. | Python/pytest para executar; contrato de perfil e manifests ainda precisam ser definidos. |
| P0-04 | P0 | Build e runbook | Corrigir os caminhos obsoletos e os falsos sucessos dos scripts; usar `--locked`, `--features firmware` no RP2350, falhar em target/ELF/UF2 ausente e alinhar recipes `just`. | `build_openkey_fido2.sh`; `build_openkey_fido2.bat`; `justfile`; `docs/hardware/rp2350-zero-validation.md` | `PLANEJADO` | `bash -n`/checagem equivalente passa; dry-run e fixtures falham quando artefato/target não existe; build RP2350 aponta para `targets/rp2350`, exige a feature correta e valida existência, formato, tamanho e SHA-256 do artefato. Runbook não referencia `examples/rp2350-firmware` inexistente nem recipes não definidos. | Toolchain e targets para validar build real; a capacidade de flash ainda requer medição JEDEC/`FLASH_DEVINFO`, não aliasing. |
| P0-05 | P0 | Documentação de segurança | Substituir a linguagem ambígua por matriz separada `IMPLEMENTADO`, `HOST-VALIDADO`, `HARDWARE-VALIDADO` e `CONFORMANCE OFICIAL`; rotular testes históricos como não reproduzidos neste ciclo. | `docs/fido2.md`; `README.md`; `TODO.md`; `docs/hardware/rp2350-zero-validation.md` | `PLANEJADO` | Cada alegação aponta para comando, commit e log quando for host; placa/periférico/firmware/evidência quando for hardware; versão/perfil/relatório quando for conformance. CTAP1/U2F, WebAuthn L2/L3, UV biométrico, NFC/BLE reais e certificação permanecem não comprovados. | Nenhum hardware é necessário para editar, mas os dados de validação devem continuar ausentes até existirem. |
| P0-06 | P0 | Contrato de evidência | Criar manifest JSON por placa/variante com revisão, MCU, firmware/UF2 SHA-256, VID/PID, interfaces, serial, método de flash, versões de ferramentas, timestamp, resultados, logs e fotos quando aplicável. | `docs/hardware/` (novo manifest/schema/runbook); `tools/hardware_check.py`; `tools/flash_rp2350.py`; `tests/python/test_hardware_acceptance.py` | `PLANEJADO` | Schema valida campos obrigatórios e rejeita capacidade/identidade ambígua. O relatório sempre conserva `expected`, `observed`, `status` e caminho dos artefatos; ausência de dispositivo ou ferramenta fica `BLOCKED`, nunca `PASS`. | Definição do board exato e execução física para preencher valores observados. |
| P1-01 | P1 | CI reprodutível | Fixar toolchain/ações/CLI em versões auditáveis; usar `--locked`; tornar `cargo-audit` bloqueante; auditar explicitamente `tests/python/requirements.txt` com constraints/hashes; reduzir permissões do bot. | `.github/workflows/ci.yml`; `.github/workflows/security.yml`; `.github/workflows/security-bot.yml`; `.github/workflows/codeql.yml`; `.github/dependabot.yml`; `tests/python/requirements.txt` | `PLANEJADO` | Workflow executa em ambiente limpo, falha com vulnerabilidade/lock inconsistente e não concede `issues: write` sem necessidade. O pin por SHA e versões são documentados e atualizáveis por Dependabot. | Execução no GitHub Actions; rede, toolchains e dependências pinadas. |
| P1-02 | P1 | Fuzz e release | Criar fuzz finito dos três harnesses e release dry-run separado de assinatura/publicação; gerar checksums, SBOM/provenance e verificar cada artefato. Assinatura só em job protegido com aprovação e secrets isolados. | `.github/workflows/fuzz.yml` (novo); `.github/workflows/release.yml` (novo); `fuzz/README.md`; workflows de CI/security | `PLANEJADO` | Fuzz não usa hardware/secrets e preserva crash input somente em falha. Dry-run valida ELF/UF2, `SHA256SUMS`, SBOM/provenance e verificação de hash; publish/sign não roda em PR e falha se `cosign verify` ou hash divergirem. | Toolchain/nightly/cargo-fuzz; configuração protegida e `cosign` só para a etapa posterior; nenhum segredo deve ser criado para validar o dry-run. |
| P1-03 | P1 | Transportes | Manter NFC/BLE sem driver fictício; ampliar somente testes/contratos de limites e documentar que aliases UUID, `Nrf52840Nfc` e `Nrf52840BleGatt` são stubs/referências, não interoperabilidade FIDO. Corrigir comentário CCID stale. | `crates/openkey-transport/src/nfc/nfc.rs`; `crates/openkey-transport/src/bluetooth/ble_gatt.rs`; `crates/openkey-transport/src/embedded/nrf52840.rs`; `targets/rp2350/src/composite.rs`; `README.md`/docs | `PLANEJADO` | Testes host cobrem overflow, estado após erro de init, timeout, MTU e desconexão; documentação não anuncia advertising/GATT/NFCT real. Nenhum teste mock é chamado de validação de rádio. | Escolha de placa e stack real para qualquer implementação funcional futura. |
| P1-04 | P1 | Matriz de perfis | Registrar explicitamente que `nRF52840` no simulador não é firmware/board e que STM32L4 ainda é só `BoardDefinition`; não promover perfil declarativo a integração física. | `targets/simulator/python/board/profiles.py`; `targets/simulator/python/board/board.py`; `targets/stm32/src/lib.rs`; `build_openkey_fido2.sh`; docs de hardware | `PLANEJADO` | Build script não retorna sucesso por target ausente; cada perfil tem `IMPLEMENTADO` separado de `HOST-VALIDADO` e `HARDWARE-VALIDADO`; a matriz lista runner, flash e periféricos reais ou `NÃO DEFINIDO`. | Decisão de hardware/board para nRF52840 e STM32L4. |
| P2-01 | P2 | Revisão de segurança | Depois dos gates host, revisar fontes de verdade de retries UV, mapeamento de indisponibilidade/I/O, rollback/contador e política de falha fechada com o contrato do sensor real. | `crates/openkey-hal/src/board_generic.rs`; `crates/openkey-device/src/authenticator/authenticator.rs`; `crates/openkey-core/src/ctap2/client_pin.rs`; `crates/openkey-core/src/ctap2/ctap2.rs`; ADR UV | `BLOQUEADO-HARDWARE` | ADR aprovado define tipos de erro, quem decrementa/persiste retries, sem consumo indevido em I/O, comportamento após reboot e testes de falha. Só então adaptar driver real; não fazer sincronização especulativa agora. | Sensor/driver e especificação do fornecedor ainda inexistentes. |
| P2-02 | P2 | Conformance oficial | Preparar perfil e checklist para o FIDO Conformance Tool, solicitar/acessar a ferramenta autorizada e executar somente contra artefato/firmware identificados. | `docs/fido2.md`; `README.md`; `TODO.md`; `tests/python/conformance/`; novo relatório em `docs/conformance/` | `BLOQUEADO-FIDO` | Relatório oficial contém versão do tool, perfil, commit/firmware, configuração, casos, falhas, logs e resultado. Só após isso a coluna `CONFORMANCE OFICIAL` pode ser `PASS`. | Registro/acesso de participante, tool oficial e hardware correspondente; nenhum desses recursos foi demonstrado. |

---

## 3. Mudanças de código e documentação seguras sem hardware

Estas tarefas podem ser preparadas em branch/PR sem uma placa conectada. O aceite final ainda exige os testes indicados na seção 4 quando eles dependerem de cargo/pytest. **Não adicionar driver fictício, sensor simulado em produção, identidade de fabricante ou PASS físico sintético.**

### 3.1 UV: fechar o wiring de teste sem habilitar capacidade real

1. Alterar apenas o módulo de testes do adaptador em `crates/openkey-device/src/authenticator/authenticator.rs`.
2. Tornar o mock configurável para sucesso, falha e retries reportados.
3. Adicionar caso de falha que confirme `Ctap2Error::UvBlocked` e caso de encaminhamento de `retries()`.
4. Preservar o teste que mostra que `uv_support(true)` sem verificador não anuncia `options.uv`.
5. Não mudar perfis de produção, não declarar sensor, não alegar biometria, não resolver ainda a disputa entre contador do core e contador do dispositivo.

O próximo incremento de hardware deverá separar, na API, falha de verificação de indisponibilidade/I/O antes de escolher a fonte única de retries.

### 3.2 CCID: tornar o contrato do caminho legado explícito

- Alinhar ou documentar conscientemente `FramedCcidTransport` com o parser canônico `iso7816::Apdu`.
- Adicionar testes para formas 2E/3E/4E, truncamento, `Le`, `SW`, GET RESPONSE e respostas multipacote.
- Adicionar regressão end-to-end no `MockUsbBus` existente, incluindo `IccPowerOn`, ATR T=0, SELECT de AIDs, `XfrBlock`, `bSlot`, `bSeq` e `SW` esperado.
- Corrigir o comentário stale em `targets/rp2350/src/composite.rs` que chama CCID de stub, sem criar uma segunda classe USB nem conectar `UsbCcidBackendDevice` em paralelo ao `CompositeUsbDevice`.

### 3.3 Aceitação e ferramentas: implementar um contrato que não mascara bloqueios

- Introduzir schema/fixtures para `PASS`, `FAIL`, `BLOCKED` e `NOT_RUN`.
- Fazer `hardware_check.py` aceitar o dispositivo explicitamente selecionado e validar expectativas do perfil, em vez de testar somente o primeiro HID.
- Separar ferramenta neutra de compatibilidade opcional de fabricante. A presença de `ykman`, nome Yubico ou VID/PID de terceiro nunca pode ser critério de aceitação.
- Fazer ausência de `fido2`, PC/SC, OpenSC, dispositivo ou target ser diagnosticada com código próprio, não confundida com sucesso.
- Ajustar `flash_rp2350.py` para não retornar sucesso após `found=false`; exigir readback/hash quando o fluxo físico for ativado.

### 3.4 Documentação e automação

- Atualizar a matriz nos três documentos de produto (`docs/fido2.md`, `README.md`, `TODO.md`).
- Corrigir caminhos e receitas do runbook RP2350; remover afirmações de diagnóstico medido que não tenham log/manifest correspondente.
- Criar runbooks nRF52840 e STM32L4 apenas como **bloqueados/planejados** até board, part, runner, pinout, stack e método de flash serem fixados.
- Adicionar `just` recipes somente se realmente implementados; não documentar comandos inexistentes.
- Preparar workflows de CI/fuzz/release em modo revisável, sem secrets e sem publicar artefatos como release oficial.

---

## 4. Validações dependentes de cargo/toolchain

Esta seção é um gate separado do hardware. Um resultado host não prova rádio, USB físico, UV biométrico, segurança elétrica ou conformance oficial.

### 4.1 Preparação obrigatória

Executar em um ambiente declarado e reproduzível, com commit registrado:

```sh
command -v cargo rustc rustup python3
cargo --version
rustc --version
rustup show active-toolchain
python3 --version
python3 -m pytest --version
```

Se `cargo` não estiver no `PATH`, instalar/ativar o toolchain documentado ou usar um caminho explícito **registrado no log**. A execução focalizada com Rust 1.97.1 encontrada em uma auditoria não substitui a confirmação do ambiente atual.

Registrar pelo menos: `git rev-parse HEAD`, `git status --short`, versões, sistema operacional, comandos completos, exit codes e logs. Não incluir secrets nem material de chave.

### 4.2 Gate host Rust/Python

Após a preparação, executar e arquivar saídas:

```sh
cargo fmt --all -- --check
cargo build --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
PYTHONPATH=targets/simulator/python:tests/python python3 -m pytest tests/python
```

Executar também os focos ligados a este backlog:

```sh
cargo test -p openkey-core --lib uv -- --nocapture
cargo test -p openkey-device --lib uv -- --nocapture
cargo test -p openkey-hal --lib -- --nocapture
cargo test -p openkey-transport --features embedded,usb-device
cargo test -p openkey-transport framed_nfc
cargo test -p openkey-transport framed_ble
cargo test -p openkey-transport framed_ccid
cargo test -p openkey-transport usb_ccid_backend
cargo test -p openkey-device profile
```

Os nomes/filtros devem ser ajustados se o workspace reportar outro nome de teste; o log deve conservar o comando efetivamente executado. Os três números históricos da auditoria UV podem ser comparados, mas não devem ser repetidos como resultado novo sem saída atual.

### 4.3 Gate embedded, scripts, segurança e fuzz

- Confirmar que os targets e features existem antes de alegar cross-build:

```sh
cargo check -p openkey-transport --target thumbv8m.main-none-eabihf --features embedded --no-default-features
cargo check -p openkey-transport --target thumbv7em-none-eabihf --features embedded --no-default-features
```

- Compilar o RP2350 com o manifest real de `targets/rp2350`, feature `firmware`, `--locked` e target correto; verificar ELF/UF2, tamanho e SHA-256. Só marcar `HOST-VALIDADO` se o artefato existir e o log estiver arquivado.
- Executar `bash -n build_openkey_fido2.sh` e testes de argumentos do script; no Windows, executar a verificação equivalente em ambiente suportado.
- Rodar `cargo audit` de modo bloqueante e `pip-audit -r tests/python/requirements.txt` (ou equivalente com constraints), sem `continue-on-error` para vulnerabilidades.
- Executar cada harness de fuzz por tempo/iterações finitos, sem hardware, e preservar crash input/log somente se houver falha.

**Critério de encerramento deste bloco:** existe um relatório host reproduzível e todos os checks requeridos passam no commit indicado. Isso não altera os estados `HARDWARE-VALIDADO` ou `CONFORMANCE OFICIAL`.

### 4.4 Bloqueios conhecidos deste bloco

- `cargo`, `rustc` e `rustup` não estavam disponíveis no `PATH` em várias auditorias.
- `pytest`, `fido2`, `probe-rs`, `picotool`, `lsusb`, `opensc-tool` e `ykman` não estavam disponíveis na análise física; alguns são opcionais, outros são pré-condições de fluxos específicos.
- Não existe atualmente target de firmware nRF52840 no checkout; um warning/retorno zero de script não é build.
- Os manifests reais não coincidem com os caminhos `examples/rp2350-firmware` e `examples/nrf52840-firmware` citados em scripts/runbook antigos.

---

## 5. Validações dependentes de placas, periféricos e acesso FIDO

Nenhuma destas validações pode ser simulada por `MockUsbBus`, perfil Python, alias UUID, nome de fabricante, foto sem log ou teste contra o próprio simulador.

### 5.1 RP2350 / RP2350-Zero

Antes de iniciar, fixar variante exata, revisão da placa, MCU, método de flash e VID/PID esperados. O manifest deve registrar capacidade autoritativa por JEDEC/`FLASH_DEVINFO`; a discrepância documentada de 2 MiB versus 4 MiB é `BLOCKED` até ser medida.

Gate físico mínimo:

1. Gravar o firmware identificado, registrar ferramenta, comando, UF2/ELF e SHA-256.
2. Fazer readback/hash e verificar região de storage dentro da capacidade medida.
3. Resetar e confirmar enumeração simultânea das interfaces USB HID FIDO e CCID, descritores, endpoints e identidade esperada.
4. Executar CTAPHID `INIT`/`PING`, `GetInfo` contra o perfil (AAGUID, versões, algoritmos, opções e firmwareVersion), MakeCredential/GetAssertion e reset conforme o runbook.
5. Para CCID, confirmar PowerOn/ATR parseável, estado `PRESENT` e SELECT dos AIDs Management/OATH/PIV/OpenPGP com payload e `SW` esperados.
6. Confirmar user presence tocando BOOT quando aplicável, inclusive rejeição sem toque, sem afirmar UV biométrica.
7. Verificar persistência de credencial/contador após pelo menos três power-cycles reais; registrar cada ciclo e recuperação.
8. Medir/documentar secure boot, OTP, debug lock, RDP/WRP somente quando realmente observados; propriedades do perfil não contam como medição.

Resultado aceitável: relatório por item com `PASS/FAIL/BLOCKED`, manifest, logs, hash, identificação de placa e evidência de enumeração. Sem isso, manter `BLOQUEADO-HARDWARE`.

### 5.2 nRF52840

O repositório tem perfil/simulador, mas não demonstrou um target de firmware nRF52840, placa, runner, rádio ou stack NFCT/BLE. Primeiro escolher a placa física e registrar part, bootloader, runner, stack e pinout. Não aceitar o nome `NRF` do simulador como evidência.

Depois, validar independentemente:

- USB HID/CTAPHID: enumeração, descritores, INIT/PING, GetInfo e operações CTAP.
- BLE FIDO GATT: advertising, UUID/serviço, conexão, CCCD/subscription, MTU, fragmentação, timeout, desconexão e troca com central real.
- NFC Type 4/ISO-DEP: campo, ATS, APDU, timeout e fragmentação com leitor real.
- Presença do usuário, armazenamento após reset/power-cycle, brownout, consumo e estado de debug.

Qualquer transporte sem driver/stack real deve ser `NOT_RUN` ou `BLOCKED`, nunca `PASS`.

### 5.3 STM32L4

Primeiro fixar part/board, linker/memória, clock, pinout USB FS, runner e ferramenta de flash; a `BoardDefinition` isolada não é firmware. Só depois executar:

- USB HID + CCID, descritores, CTAPHID, ATR e APDU/SELECT.
- Fonte física de user presence; se não existir, registrar explicitamente como não suportada.
- RNG/KAT, armazenamento e persistência após reboot, brownout e power-cycle.
- RDP/WRP e estado SWD/JTAG, somente com medição/evidência.

Sem caminho de firmware e runner, o gate fica `BLOQUEADO-HARDWARE`, não `PASS` por compilação de perfil.

### 5.4 UV biométrico real

O host/mock não encerra esta frente. A próxima etapa física deve:

- escolher sensor/driver e placa target;
- definir contrato de `VerificationFailed` versus indisponibilidade/I/O;
- definir uma fonte única para retries e sua persistência/anti-hammering;
- validar tempos, power-cycle, bloqueio e recuperação;
- confirmar que `GetInfo`/flags UV só anunciam capacidade quando o dispositivo real está presente e operacional.

Sem sensor conectado e validação física, UV real permanece `BLOQUEADO-HARDWARE`.

### 5.5 FIDO Conformance oficial

A suíte `tests/python/conformance`, o `SimulatorClient`, raw-CBOR e a ponte UHID podem permanecer como regressão local. Eles não conferem certificação. Para o gate oficial:

1. Obter registro/autorização e versão do FIDO Conformance Tool.
2. Fixar perfil, firmware, commit, placa e configuração.
3. Executar o conjunto autorizado, preservando logs e relatório original.
4. Registrar falhas, exceções e limitações sem convertê-las em aprovação.
5. Só preencher `CONFORMANCE OFICIAL = PASS` com relatório oficial e evidência revisável.

Estado atual: `BLOQUEADO-FIDO`; nenhuma execução oficial foi demonstrada.

---

## 6. Critérios de aceite comuns e artefatos obrigatórios

Toda tarefa que for promovida no backlog deve anexar, no PR ou em `docs/validation/`/`docs/conformance/`, conforme o caso:

- commit SHA e estado limpo/justificado do checkout;
- board/variante/MCU e revisão, quando aplicável;
- toolchain e versões de ferramentas;
- comando completo, data/hora e exit code;
- saída integral ou log apontado por caminho;
- expected/observed por item;
- firmware/ELF/UF2 e SHA-256, quando aplicável;
- manifest de VID/PID, interfaces, serial e método de flash, quando físico;
- classificação explícita `PASS`, `FAIL`, `BLOCKED` ou `NOT_RUN`;
- motivo de bloqueio e próximo desbloqueador;
- ausência de secrets, PINs, chaves privadas e dados biométricos nos logs.

Um teste unitário aprovado demonstra somente o contrato coberto pelo teste. Um build aprovado demonstra somente compilação. Uma enumeração USB aprovada não demonstra conformance CTAP. Um teste local aprovado não demonstra conformance oficial. Essas distinções devem permanecer nos documentos.

---

## 7. Próximos passos ordenados

### Próximos passos imediatos — sem hardware

1. Abrir branch/PR para P0-01 e P0-02, mantendo produção sem sensor UV e sem novo driver de rádio.
2. Implementar P0-03 e seus fixtures de veredictos/exit codes; garantir que ausência de pré-condição seja `BLOCKED`, não sucesso silencioso.
3. Corrigir P0-04, incluindo caminhos `targets/rp2350`, `--features firmware`, `--locked`, falha de artefato ausente e recipes `just` reais.
4. Atualizar P0-05 e P0-06 para que a matriz e o manifest sejam a fonte de evidência, sem preencher valores observados fictícios.
5. Fazer revisão estática dos workflows P1-01/P1-02 e do contrato NFC/BLE de P1-03; não publicar nem assinar artefatos ainda.

### Assim que um ambiente de toolchain estiver disponível

6. Registrar o ambiente e executar o gate da seção 4, começando por `fmt`, build/test workspace, clippy, Python e os focos UV/CCID.
7. Executar cross-checks e build RP2350 real somente depois de corrigir scripts e targets; guardar ELF/UF2, tamanho e hash.
8. Rodar security/fuzz/release dry-run; corrigir qualquer falha antes de considerar o CI reproduzível.
9. Atualizar o status dos itens somente com logs do commit; não converter resultados históricos em resultados atuais.

### Quando houver placa e instrumentação

10. Escolher uma variante RP2350 e completar o manifest físico; executar o gate USB HID+CCID, CTAP, persistência e user presence.
11. Fixar placa/stack/runner para nRF52840 antes de prometer NFC/BLE; validar cada transporte separadamente com leitor/central reais.
12. Fixar part/board/runner STM32L4 e implementar o caminho de firmware antes de abrir o gate físico.
13. Integrar sensor UV real somente com contrato de erros/retries aprovado e evidência de power-cycle/lockout.

### Quando houver acesso oficial FIDO

14. Preparar e executar o FIDO Conformance Tool contra o artefato e hardware identificados.
15. Publicar o relatório oficial e somente então atualizar a matriz para `CONFORMANCE OFICIAL`.
16. Separar publicação/assinatura de artefatos em ambiente protegido; nunca expor secrets a PRs ou código não confiável.

**Conclusão honesta da abertura:** há uma base relevante de código e mocks, mas o próximo ciclo ainda é de fechamento de contratos, reprodutibilidade e evidência. Hardware UV/radio, aceitação física das boards e conformance FIDO continuam bloqueados e não devem ser declarados concluídos.
