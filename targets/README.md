# targets/ — Implementações específicas de hardware

Única camada que pode depender de PAC/HAL de vendor, `cortex-m-rt`,
`panic-halt`, `embedded-alloc` e linker scripts. Perfis de board
(`BoardDefinition` consts) vivem nas crates `profile-*`; firmware
executável nas pastas do chip.

| Pasta | Crate | Conteúdo |
|---|---|---|
| `targets/profile-generic/` | `openkey-profile-generic` | `GENERIC` (host/dev, USB-CCID) |
| `targets/profile-nrf52840/` | `openkey-profile-nrf52840` | `NRF52840` (USB-HID, NFC, BLE) |
| `targets/profile-stm32/` | `openkey-profile-stm32` | `STM32L4` (USB-HID, USB-CCID) |
| `targets/profile-esp32/` | `openkey-profile-esp32` | `ESP32C3` (USB-HID, BLE) |
| `targets/profile-rp2350/` | `openkey-profile-rp2350` | `RP2350`, `RP2350_ZERO`, `YUBIKEY_4_5`, `rp2350_with_pins` |
| `targets/rp2350/` | `rp2350-firmware` (standalone) | binário bare-metal + `vendor/ring` + `pico2.yaml`/`tiny2350.yaml` |
| `targets/nrf52840/` | `nrf52840-firmware` (standalone) | binário bare-metal |
| `targets/rp2040/` | reservado | sem perfil (seguir padrão `rp2350`) |

Registro de `getrandom(custom)` e heap `embedded-alloc` vivem no `main.rs` do target.
