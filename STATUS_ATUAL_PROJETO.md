# Status Atual do Projeto — OpenKey-Fido2

**Data da análise:** 9 de outubro de 2026
**Repositório:** [zequinha-taveira/OpenKey-Fido2](https://github.com/zequinha-taveira/OpenKey-Fido2)
**Branch analisado:** `main`
**Commit analisado:** `ccac49b` — merge de correções de build do workspace
**Versão declarada:** `0.1.1`
**Licença:** MIT OR Apache-2.0

## 1. Resumo executivo

O OpenKey-Fido2 está em um estágio de **protótipo avançado / pré-validação de hardware**, com um núcleo funcional e bastante abrangente de FIDO2/WebAuthn em Rust, simulador host, persistência cifrada, transportes modulares e suíte de testes automatizados.

A maior parte do trabalho de protocolo e infraestrutura está marcada como concluída no `TODO.md`. O projeto já cobre, no host/simulador, os principais fluxos de CTAP2.1, incluindo PIN/UV, Credential Management, Large Blobs, HMAC-secret, attestation, configuração do autenticador, persistência e transporte CTAPHID.

O projeto **ainda não deve ser considerado pronto para produção ou conformidade oficial**, principalmente porque permanecem pendentes:

- validação física em placas reais;
- sensor/driver de User Verification (UV) real;
- integração completa de NFC e BLE com stacks de hardware;
- execução do FIDO Conformance Tool oficial;
- assinatura real dos artefatos de release;
- confirmação de compatibilidade completa com WebAuthn, CTAP2.0 legado e U2F/CTAP1.

## 2. Escopo e arquitetura implementados

- Firmware FIDO2/WebAuthn em Rust, com núcleo preparado para `no_std`.
- Workspace Cargo com **24 membros declarados** entre crates de núcleo, targets, exemplos, testes e ferramentas.
- Camadas principais:
  - `openkey-core`: domínio MCU-independent;
  - `openkey-crypto`: primitivas criptográficas;
  - `openkey-storage`: armazenamento cifrado e backends;
  - `openkey-transport`: CTAPHID, USB-HID, CCID e adaptadores;
  - `openkey-device`: composição do autenticador;
  - `targets/simulator`: simulador sem necessidade de hardware;
  - `targets/rp2350`, `targets/rp2040`, `targets/stm32` e `targets/generic`.
- Arquitetura documentada em `docs/architecture.md`, com dependências direcionais e contratos entre módulos.
- Estratégia de persistência com `StorageBackend` injetável, encryption at rest via ChaCha20-Poly1305 e recuperação simulada contra perda de energia.

## 3. Funcionalidades principais concluídas

### CTAP2.1 e WebAuthn

- `MakeCredential`, `GetAssertion`, `GetInfo`, `GetVersion`.
- `GetNextAssertion`, `Selection` e `Reset`.
- `ClientPIN` com protocolos 1 e 2, troca ECDH, tokens e permissões.
- Credential Management: enumeração de RPs/credenciais, atualização e exclusão.
- Large Blobs e `largeBlobKey`.
- `authenticatorConfig`, incluindo `alwaysUv`, `minPINLength`, Enterprise Attestation e controles de configuração.
- Attestation `none`, `self` e `packed`.
- Extensões `credProtect`, `credBlob`, `minPinLength` e `hmac-secret`.
- Validações de allow list/exclude list, RP ID, algoritmos e payload CBOR residual.

### Criptografia e proteção de segredos

- Ed25519, ES256, ES384, PS256 e RS256.
- HMAC-SHA256 e SHA-256.
- ChaCha20-Poly1305 para encryption at rest.
- ECIES com X25519, HKDF-SHA256 e ChaCha20-Poly1305.
- Comparações constant-time e zeroização de material sensível.
- Contador monotônico de assinaturas e proteção de PIN com rate limiting.

### Storage e persistência

- Credenciais cifradas, lookup por ID/RP ID e armazenamento de PIN.
- Persistência de `signCount`, Reset e Large Blobs entre reinícios.
- Backend de arquivo para desenvolvimento e abstração para flash.
- Wear leveling atualmente informativo/simulado, sem rotação física real.

### Transportes, boards e firmware

- Framing CTAPHID, fragmentação, remontagem, timeout, cancelamento e gerenciamento de canais.
- USB-HID concreto via `usb-device` para o target RP2350.
- CCID com roteamento ISO 7816 e applets Yubico/OATH/Management no firmware RP2350.
- Perfis para NRF52840, STM32L4, ESP32C3, RP2350, RP2350-Zero e GENERIC.
- User presence via BOOTSEL no RP2350/RP2350-Zero.
- Builds bare-metal preparados para RP2350 e nRF52840.
- Identidade USB padrão `1209:0001`; identidade YubiKey `1050:0407` disponível somente para teste privado.
- NFC e BLE possuem interfaces, framing e mocks host-verificáveis, mas ainda não têm stack de hardware completa validada.

## 4. Testes e automação

O repositório contém, por inspeção estática:

- aproximadamente **702 funções/atributos de teste Rust**;
- **28 arquivos** de teste Python;
- aproximadamente **281 funções de teste Python**;
- **31 ADRs** em `docs/adr`.

A documentação registra uma suíte CTAP2.1 de conformidade em modo raw-CBOR, com 53 testes mencionados como aprovados em execução anterior, além de testes E2E de persistência, ClientPIN, extensões, attestation, comandos CTAP2, segurança e bridge CTAPHID.

O CI configurado em `.github/workflows/ci.yml` cobre:

1. `cargo fmt --check`;
2. `cargo build --workspace`;
3. `cargo test --workspace`;
4. `cargo clippy --workspace --all-targets -- -D warnings`;
5. build da extensão Python;
6. execução de `pytest tests/python`.

### Resultado da validação nesta análise

A validação local **não pôde executar build/testes Rust**, pois o ambiente Sandbox não possui `cargo`, `rustc` nem `rustup` instalados. Portanto, não há um novo resultado local de aprovação ou reprovação do código; o bloqueio é exclusivamente de ferramenta do ambiente:

```text
bash: cargo: command not found
```

O checkout está limpo (`git status` sem alterações locais).

## 5. Situação atual no GitHub

- Repositório público, sem release publicada.
- Issues abertas: nenhuma encontrada.
- Pull requests listadas: 10 no histórico, incluindo PRs recentes de dependências e CI.
- No commit analisado, os workflows `Security` e `Dependabot Updates` concluíram com sucesso.
- Os workflows `CI` e `CodeQL Security` do commit `ccac49b` estavam em andamento no momento da análise.
- Houve execuções anteriores com falha em Dependabot/Security; recomenda-se acompanhar os resultados finais dos workflows atuais antes de marcar a revisão como totalmente verde.

## 6. Gaps e riscos remanescentes

### Alta prioridade — validação externa

1. **Validação física do firmware** em RP2350/RP2350-Zero, nRF52840 e STM32L4: USB, HID, CCID, clocks, flash, reset e persistência.
2. **Execução do FIDO Conformance Tool oficial**, que requer acesso/registro de participante.
3. **Teste real com browser e hosts FIDO2**, além do simulador e bridges locais.
4. **Assinatura real de artefatos de release**: o workflow está preparado, mas depende do segredo protegido `PRIVATE_KEY_B64`.

### Alta/média prioridade — hardware e transportes

1. Implementar/validar sensor ou dispositivo real de UV; atualmente a abstração e mocks existem, mas não há biometria real integrada.
2. Ligar o adaptador CCID a um driver/perfil de board específico e validar em placa.
3. Implementar stack NFC ISO 14443/NFCT real e validar com leitor.
4. Implementar servidor BLE GATT/SoftDevice/controlador e validar com central BLE.
5. Implementar o driver do LED WS2812B do RP2350-Zero, atualmente com o pino registrado, mas driver ainda pendente.

### Compatibilidade e documentação

- `docs/fido2.md` ainda descreve CTAP2.1, WebAuthn 2/3, CTAP2.0 legado e U2F como “planned / pending verification”; esse documento deve ser atualizado para refletir a implementação host-verificável e separar claramente o que foi validado do que ainda depende de hardware/conformidade.
- O README já registra corretamente gaps importantes: ausência de sensor biométrico real, ausência de secure element auditado, assinatura de artefatos pendente e FIDO Conformance Tool pendente.
- O desgaste de flash permanece como contador informativo; não há rotação física de wear leveling implementada.

## 7. Classificação de maturidade

| Área | Situação | Avaliação |
|---|---|---|
| Núcleo CTAP2.1 | Implementado e coberto por testes documentados | Forte no host/simulador |
| Criptografia | Ampla implementação com mitigação de segredos | Requer revisão/auditoria independente para produção |
| Persistência | Implementada e testada no simulador | Falta confirmar flash real e perda de energia |
| USB-HID/CTAPHID | Implementação concreta e mocks | Falta validação física completa |
| CCID/OATH/Management | Host-verificável e integrado ao firmware RP2350 | Falta validação física |
| NFC | Framing/adaptadores presentes | Stack e hardware pendentes |
| BLE | Framing/adaptadores presentes | Stack e hardware pendentes |
| User Presence | BOOTSEL no RP2350 | Funcionalidade depende da placa real |
| User Verification | Abstração, mocks e gates presentes | Sensor real pendente |
| Conformidade FIDO oficial | Não executada | Bloqueada por ferramenta/acesso externo |
| Release | Sem release publicada | Assinatura e validação final pendentes |

## 8. Próximos passos recomendados

1. Instalar Rust 1.85+ e executar exatamente os gates do README e do CI.
2. Aguardar e registrar o resultado final de CI/CodeQL no commit `ccac49b`.
3. Atualizar `docs/fido2.md` com uma matriz “implementado / host-validado / hardware-validado / conformidade oficial”.
4. Executar o runbook de validação física do RP2350-Zero e registrar logs, enumeração USB e persistência após power cycle.
5. Repetir a validação em nRF52840 e STM32L4.
6. Configurar assinatura protegida dos artefatos e publicar uma primeira release somente após os testes físicos.
7. Planejar integração de UV, NFC e BLE como incrementos separados, cada um com testes de hardware reproduzíveis.

## Conclusão

O OpenKey-Fido2 apresenta uma **base técnica substancial e modular**, com o núcleo de protocolo, criptografia, storage e simulador em estágio avançado. O principal risco atual não é ausência de funcionalidades básicas, mas sim a distância entre a implementação host-verificável e a **validação em hardware real, auditoria de segurança, interoperabilidade de campo e conformidade oficial FIDO**.

A recomendação é tratar o projeto como **release candidate técnico para validação**, não como produto final de produção, até fechar os gaps de hardware, conformance e assinatura de artefatos.
