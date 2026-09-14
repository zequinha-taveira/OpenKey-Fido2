//! STM32L4 board profile (ST, Cortex-M4F).
//!
//! Autenticador com USB-HID e USB-CCID. Passe [`STM32L4`] para
//! `DeviceProfileBuilder::from_board` ou `EmbeddedAuthenticator::new_with_board`.

#![no_std]

use board_generic::BoardDefinition;

/// STM32L4-based authenticator with USB-HID and USB-CCID transports.
pub const STM32L4: BoardDefinition = BoardDefinition::new(
    "stm32l4-fido",
    [
        0x53, 0x54, 0x4d, 0x33, 0x32, 0x4c, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x02,
    ],
)
.usb_hid()
.usb_ccid()
.secure_storage(true)
.crypto_accelerator(true)
.led(0)
.button(1)
.i2c_sda(8)
.i2c_scl(9)
.spi_mosi(10)
.spi_miso(11)
.spi_clk(12)
.cs(13)
.reset(14)
.irq(15);

#[cfg(test)]
mod tests {
    use super::*;
    use board_generic::UserPresenceSource;

    #[test]
    fn test_stm32l4_identity() {
        assert_eq!(STM32L4.name, "stm32l4-fido");
        assert_eq!(STM32L4.aaguid[15], 0x02);
    }

    #[test]
    fn test_stm32l4_has_no_auto_presence() {
        assert_eq!(STM32L4.presence_source, UserPresenceSource::None);
    }
}
