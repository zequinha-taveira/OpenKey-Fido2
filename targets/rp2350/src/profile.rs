//! RP2350 board profiles (Raspberry Pi, Cortex-M33).
//!
//! [`RP2350`] (dev board), [`RP2350_ZERO`] (Waveshare RP2350-Zero) e
//! [`YUBIKEY_4_5`] (variante YubiKey 4/5). Use [`rp2350_with_pins`] para
//! pinagem customizada. Passe o perfil para
//! `DeviceProfileBuilder::from_board` ou `EmbeddedAuthenticator::new_with_board`.

use board_generic::{BoardDefinition, SecurityFeatures, UserPresenceSource};

/// Raspberry Pi RP2350-based authenticator with USB-HID and USB-CCID transports.
///
/// The RP2350 provides hardware security features including ARM TrustZone
/// (Cortex-M33), secure boot with RSA signature verification, hardware TRNG,
/// SHA-256 accelerator, OTP memory for key storage, unique chip ID, and
/// debug disable via OTP.
///
/// Pin assignments reflect a typical dev board. Use [`rp2350_with_pins`] to
/// override for boards with different wiring.
pub const RP2350: BoardDefinition = rp2350_with_pins(Rp2350Pins {
    i2c_sda: 4,
    i2c_scl: 5,
    spi_mosi: 6,
    spi_miso: 7,
    spi_clk: 8,
    cs: 9,
    reset: 10,
    irq: 11,
    led: 25,
    button: 13,
});

/// RP2350 board with custom pin assignments.
///
/// Use this when the target board uses different GPIOs than the defaults.
///
/// ```
/// use profile_rp2350::{rp2350_with_pins, Rp2350Pins};
///
/// let board = rp2350_with_pins(Rp2350Pins {
///     i2c_sda: 2,
///     i2c_scl: 3,
///     spi_mosi: 16,
///     spi_miso: 19,
///     spi_clk: 18,
///     cs: 17,
///     reset: 20,
///     irq: 21,
///     led: 10,
///     button: 15,
/// });
/// ```
#[inline]
pub const fn rp2350_with_pins(pins: Rp2350Pins) -> BoardDefinition {
    BoardDefinition::new(
        "rp2350-fido",
        [
            0x52, 0x50, 0x32, 0x33, 0x35, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x05,
        ],
    )
    .usb_hid()
    .usb_ccid()
    .secure_storage(true)
    .crypto_accelerator(true)
    .security_features(SecurityFeatures::rp2350())
    // User presence reaproveita o BOOTSEL (linha CS da flash QSPI), não um GPIO.
    .presence_source(UserPresenceSource::Bootsel)
    .i2c_sda(pins.i2c_sda)
    .i2c_scl(pins.i2c_scl)
    .spi_mosi(pins.spi_mosi)
    .spi_miso(pins.spi_miso)
    .spi_clk(pins.spi_clk)
    .cs(pins.cs)
    .reset(pins.reset)
    .irq(pins.irq)
    .led(pins.led)
    .button(pins.button)
}

/// Pin assignments for [`rp2350_with_pins`].
pub struct Rp2350Pins {
    /// GPIO do sinal I2C SDA.
    pub i2c_sda: u8,
    /// GPIO do sinal I2C SCL.
    pub i2c_scl: u8,
    /// GPIO do sinal SPI MOSI.
    pub spi_mosi: u8,
    /// GPIO do sinal SPI MISO.
    pub spi_miso: u8,
    /// GPIO do clock SPI.
    pub spi_clk: u8,
    /// GPIO do chip select.
    pub cs: u8,
    /// GPIO de reset do periférico.
    pub reset: u8,
    /// GPIO de interrupção.
    pub irq: u8,
    /// GPIO do LED de status (user presence).
    pub led: u8,
    /// GPIO do botão de user presence.
    pub button: u8,
}

/// Waveshare RP2350-Zero — perfil do board comercial.
///
/// Diferenças em relação ao [`RP2350`] genérico (dev board):
/// - WS2812B no GPIO16 (via PIO, não GPIO comum);
/// - sem botão de usuário GPIO (BOOT entra em modo download; RUN só reseta);
/// - cristal de 12 MHz igual ao Pico 2;
/// - USB Type-C, porta única USB 1.1 device;
/// - sem sensor biométrico — UV permanece via PIN/pinUvAuthToken
///   (`GetInfo` continua omitindo a opção `uv`).
pub const RP2350_ZERO: BoardDefinition = BoardDefinition::new(
    "rp2350-zero",
    [
        0x52, 0x50, 0x32, 0x33, 0x35, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x06,
    ],
)
.usb_hid()
// CCID mantido por paridade com o perfil RP2350 (mesma porta única USB,
// interface dual consistente com a descoberta de capacidades do RP2350).
.usb_ccid()
.secure_storage(true)
.crypto_accelerator(true)
.security_features(SecurityFeatures::rp2350())
// User presence reaproveita o BOOTSEL (linha CS da flash QSPI), como no RP2350.
.presence_source(UserPresenceSource::Bootsel)
// Pinos espelhados nos defaults do RP2350 (GP4/GP5 I2C0, GP6-GP9 SPI) — todos
// livres no header de borda do RP2350-Zero (GP0-GP22, GP26-GP29) e sem colisão
// com o GPIO16 do LED.
.i2c_sda(4)
.i2c_scl(5)
.spi_mosi(6)
.spi_miso(7)
.spi_clk(8)
.cs(9)
.reset(10)
.irq(11)
// WS2812B em GPIO16 via PIO (driver PIO pendente — pino registrado para referência).
.led(16)
// Sem botão GPIO dedicado: BOOT é a linha CS da flash QSPI (modo download) e
// RUN só reseta. Sentinela `u8::MAX` marca "não conectado" (fora da faixa de
// GPIOs do bank0); a presença vem de `UserPresenceSource::Bootsel`.
.button(u8::MAX);

/// YubiKey 4/5 — perfil de produto compatível com o ecossistema YubiKey.
///
/// Mesma pinagem e transportes do RP2350-Zero (HID+CCID, BOOTSEL para
/// user presence, WS2812B em GPIO16), mas com `SecurityFeatures::yubico()`
/// (secure boot ✅ + secure lock ✅ — `tamper_detection=true`, ADR-0025)
/// e AAGUID dedicado. VID:PID `1050:0407` é do produto (`DeviceProfile.usb`).
pub const YUBIKEY_4_5: BoardDefinition = BoardDefinition::new(
    "yubikey-4-5",
    [
        0x59, 0x55, 0x42, 0x49, 0x4b, 0x45, 0x59, 0x34, 0x2d, 0x35, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x07,
    ],
)
.usb_hid()
.usb_ccid()
.secure_storage(true)
.crypto_accelerator(true)
.security_features(SecurityFeatures::yubico())
.presence_source(UserPresenceSource::Bootsel)
.i2c_sda(4)
.i2c_scl(5)
.spi_mosi(6)
.spi_miso(7)
.spi_clk(8)
.cs(9)
.reset(10)
.irq(11)
.led(16)
.button(u8::MAX);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rp2350_uses_bootsel_for_presence() {
        assert_eq!(RP2350.presence_source, UserPresenceSource::Bootsel);
    }

    #[test]
    fn test_rp2350_zero_uses_bootsel_for_presence() {
        assert_eq!(RP2350_ZERO.presence_source, UserPresenceSource::Bootsel);
    }

    #[test]
    fn test_rp2350_zero_identity() {
        assert_eq!(RP2350_ZERO.name, "rp2350-zero");
        // Mesmo prefixo ASCII "RP2350" da família, sufixo sequencial próprio.
        assert_eq!(RP2350_ZERO.aaguid[..6], RP2350.aaguid[..6]);
        assert_eq!(RP2350_ZERO.aaguid[15], 0x06);
        assert_ne!(RP2350_ZERO.name, RP2350.name);
    }

    #[test]
    fn test_rp2350_zero_led_is_ws2812_gpio16() {
        assert_eq!(RP2350_ZERO.led_pin, 16);
    }

    #[test]
    fn test_yubikey_4_5_secure_boot_and_lock() {
        assert_eq!(YUBIKEY_4_5.presence_source, UserPresenceSource::Bootsel);
        assert_eq!(YUBIKEY_4_5.led_pin, 16);
        assert_eq!(YUBIKEY_4_5.button_pin, u8::MAX);
        assert_eq!(YUBIKEY_4_5.security, SecurityFeatures::yubico());
    }

    #[test]
    fn test_yubikey_identity() {
        assert_eq!(YUBIKEY_4_5.name, "yubikey-4-5");
        assert_eq!(YUBIKEY_4_5.aaguid[15], 0x07);
    }
}
