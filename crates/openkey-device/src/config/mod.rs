//! Configuração de produto do dispositivo (`openkey-device-config`).
//!
//! Tipos puros (`core` + `alloc` apenas, sem dependências internas) que
//! descrevem as decisões comerciais de um produto: transportes, protocolos,
//! attestation, PIN, extensões e identidade USB. Consumidos por
//! `openkey-device-profile` (`DeviceProfile` + builder) e pelo firmware.

extern crate alloc;

/// Transporte ativo instanciado pelo `EmbeddedAuthenticator`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransportType {
    /// USB-HID (CTAPHID).
    UsbHid,
    /// USB-CCID (smartcard).
    UsbCcid,
    /// NFC ISO/IEC 14443.
    Nfc,
    /// BLE GATT (FIDO Bluetooth Service).
    BleGatt,
}

/// Configuração do transporte ativo de um produto.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransportConfig {
    /// Transporte selecionado para o produto.
    pub transport_type: TransportType,
}

impl TransportConfig {
    /// Cria a configuração para o transporte informado.
    pub fn new(transport_type: TransportType) -> Self {
        Self { transport_type }
    }

    /// Atalho para USB-HID.
    pub fn usb_hid() -> Self {
        Self::new(TransportType::UsbHid)
    }

    /// Atalho para USB-CCID.
    pub fn usb_ccid() -> Self {
        Self::new(TransportType::UsbCcid)
    }

    /// Atalho para NFC.
    pub fn nfc() -> Self {
        Self::new(TransportType::Nfc)
    }

    /// Atalho para BLE GATT.
    pub fn ble_gatt() -> Self {
        Self::new(TransportType::BleGatt)
    }
}

/// Physical transports a product can expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Transport {
    /// USB-CCID (smartcard).
    UsbCcid,
    /// USB-HID (CTAPHID).
    UsbHid,
    /// NFC ISO/IEC 14443.
    Nfc,
    /// Bluetooth Low Energy.
    Ble,
}

/// Supported protocol versions, mapped to CTAP2/WebAuthn wire versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocol {
    /// CTAP 2.0.
    Ctap2,
    /// CTAP 2.1.
    Ctap21,
    /// U2F / CTAP1 (legado).
    U2f,
    /// WebAuthn Level 2.
    WebAuthn,
}

/// Attestation format advertised by the authenticator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttestationType {
    /// Sem attestation (`fmt: "none"`).
    None,
    /// Formato `packed` com certificado do lote.
    Packed,
    /// Self-attestation com a própria chave da credencial.
    SelfAttested,
}

/// PIN policy for user verification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PinPolicy {
    /// ClientPIN não é anunciado nem aceito.
    Disabled,
    /// PIN pode ser configurado pelo usuário.
    Optional,
    /// PIN obrigatório para operações sensíveis.
    Required,
}

/// WebAuthn extensions supported by the authenticator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Extension {
    /// `credProtect` — política de proteção da credencial.
    CredProtect,
    /// `credBlob` — blob opaco associado à credencial.
    CredBlob,
    /// `minPinLength` — expõe o tamanho mínimo de PIN ao relying party.
    MinPinLength,
    /// `hmac-secret` — derivação de segredo simétrico por credencial.
    HmacSecret,
}

/// Preset de fornecedor para a identidade USB (VID/PID + strings USB).
///
/// Fonte única de verdade da identidade exposta no barramento; o firmware
/// (`targets/rp2350/src/composite.rs`) deriva a sua identidade destes presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsbVendorPreset {
    /// pid.codes openkey-fido2 padrão (`1209:0001`) — builds distribuídos.
    OpenKey,
    /// Yubico YubiKey 5 (`1050:0407`) — **NÃO PARA DISTRIBUIÇÃO** (VID/PID
    /// de terceiro, uso privado apenas; ADR-0025). Opt-in via
    /// `--features yubikey5-identity` (`yubikey4-identity` é alias).
    YubiKey5,
    /// Yubico com VID:PID `1050:0407` e Product Name `"Yubico Yubikey"`.
    /// The default USB identity pid.codes is 0x1209:0x0001; the YubiKey USB identity
    /// that ykman / Yubico Authenticator auto-recognize is the opt-in VID:PID=Yubikey5 build,
    /// not for distribution.
    YubicoYubikey,
}

/// Identidade USB do produto (VID/PID + strings de identificação USB).
///
/// The default USB identity pid.codes is `0x1209:0x0001`; the YubiKey USB identity
/// that ykman / Yubico Authenticator auto-recognize is the opt-in VID:PID=Yubikey5 build,
/// not for distribution.
/// `None` = identidade padrão pid.codes `1209:0001`. `Some(1050:0407)` =
/// modo compatível YubiKey 5 para ykman/Yubico Authenticator (ADR-0025).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsbIdentity {
    /// Vendor ID (USB-IF).
    pub vid: u16,
    /// Product ID.
    pub pid: u16,
    /// Nome do fabricante (string USB `iManufacturer`).
    pub manufacturer: &'static str,
    /// Nome do produto (string USB `iProduct`).
    pub product: &'static str,
}

impl UsbIdentity {
    /// Cria a identidade VID:PID com strings vazias (ver `preset()` para
    /// identidades completas com preset de fornecedor).
    pub const fn new(vid: u16, pid: u16) -> Self {
        Self {
            vid,
            pid,
            manufacturer: "",
            product: "",
        }
    }

    /// Identidade completa a partir do preset de fornecedor.
    pub const fn preset(preset: UsbVendorPreset) -> Self {
        match preset {
            UsbVendorPreset::OpenKey => Self::openkey(),
            UsbVendorPreset::YubiKey5 => Self::yubikey(),
            UsbVendorPreset::YubicoYubikey => Self::yubico_yubikey(),
        }
    }

    /// pid.codes openkey-fido2 padrão.
    pub const fn openkey() -> Self {
        Self {
            vid: 0x1209,
            pid: 0x0001,
            manufacturer: "openkey-fido2",
            product: "FIDO2 Authenticator",
        }
    }

    /// Yubico YubiKey 5 (OTP+FIDO+CCID, `0x0407`).
    ///
    /// **NÃO PARA DISTRIBUIÇÃO** — VID `0x1050` pertence à Yubico (USB-IF).
    ///
    /// O nome do reader PCSC (`manufacturer + product`) PRECISA conter os
    /// tokens de interface em maiúsculas (`OTP`/`FIDO`/`CCID`): o
    /// `ykman.pcsc._pid_from_name` deriva `PID.of(YK4, interfaces)` por
    /// substring e `PID["YK4_"]` (nome sem tokens) levanta `KeyError`,
    /// derrubando o helper do Yubico Authenticator. Com estes tokens o nome
    /// vira `Yubico YubiKey OTP+FIDO+CCID 0` → `YK4_OTP_FIDO_CCID` (`0x0407`).
    pub const fn yubikey() -> Self {
        Self {
            vid: 0x1050,
            pid: 0x0407,
            manufacturer: "Yubico",
            product: "YubiKey OTP+FIDO+CCID",
        }
    }

    /// Yubico com VID:PID `1050:0407` e Product Name `"Yubico Yubikey"`.
    ///
    /// The default USB identity pid.codes is 0x1209:0x0001; the YubiKey USB identity
    /// that ykman / Yubico Authenticator auto-recognize is the opt-in VID:PID=Yubikey5 build,
    /// not for distribution.
    pub const fn yubico_yubikey() -> Self {
        Self {
            vid: 0x1050,
            pid: 0x0407,
            manufacturer: "Yubico",
            product: "Yubico Yubikey",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openkey_preset_is_default_identity() {
        let id = UsbIdentity::preset(UsbVendorPreset::OpenKey);
        assert_eq!((id.vid, id.pid), (0x1209, 0x0001));
        assert_eq!(id, UsbIdentity::openkey());
    }

    #[test]
    fn test_yubikey_presets_share_vid_pid() {
        assert_eq!(UsbIdentity::yubikey().vid, 0x1050);
        assert_eq!(UsbIdentity::yubikey().pid, 0x0407);
        assert_eq!(UsbIdentity::yubico_yubikey().pid, 0x0407);
        assert_eq!(
            UsbIdentity::preset(UsbVendorPreset::YubiKey5),
            UsbIdentity::yubikey()
        );
    }

    #[test]
    fn test_transport_config_shortcuts() {
        assert_eq!(
            TransportConfig::usb_hid().transport_type,
            TransportType::UsbHid
        );
        assert_eq!(
            TransportConfig::ble_gatt().transport_type,
            TransportType::BleGatt
        );
    }
}
