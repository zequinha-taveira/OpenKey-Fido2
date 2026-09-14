//! Generic board profile for host/dev fallback.
//!
//! [`GENERIC`] é o perfil padrão de desenvolvimento (USB-CCID apenas, sem
//! presença automática). Passe para `DeviceProfileBuilder::from_board` ou
//! `EmbeddedAuthenticator::new_with_board`.

#![no_std]

use board_generic::BoardDefinition;

/// Generic board profile with USB-CCID transport only.
pub const GENERIC: BoardDefinition = BoardDefinition::new(
    "generic-fido",
    [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xff,
    ],
)
.usb_ccid()
.i2c_sda(0)
.i2c_scl(1)
.spi_mosi(2)
.spi_miso(3)
.spi_clk(4)
.cs(5)
.reset(6)
.irq(7)
.led(8)
.button(9);

#[cfg(test)]
mod tests {
    use super::*;
    use board_generic::UserPresenceSource;

    #[test]
    fn test_generic_identity() {
        assert_eq!(GENERIC.name, "generic-fido");
        assert_eq!(GENERIC.aaguid[15], 0xff);
    }

    #[test]
    fn test_generic_has_no_auto_presence() {
        assert_eq!(GENERIC.presence_source, UserPresenceSource::None);
    }
}
