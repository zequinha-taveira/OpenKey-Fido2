#[cfg(feature = "embedded")]
use alloc::string::ToString;
use alloc::vec::Vec;
#[cfg(feature = "embedded")]
use transport_core::embedded::{ApduResponse, UsbCcidDevice};
#[cfg(feature = "embedded")]
use transport_core::iso7816::Apdu;
use transport_core::{Transport, TransportError};

/// Adaptador que encapsula um [`UsbCcidDevice`] e gerencia APDU frames.
#[cfg(feature = "embedded")]
pub struct FramedCcidTransport<D: UsbCcidDevice> {
    device: D,
    initialized: bool,
}

#[cfg(feature = "embedded")]
impl<D: UsbCcidDevice> FramedCcidTransport<D> {
    /// Cria uma nova instância a partir de um dispositivo CCID concreto.
    pub fn new(device: D) -> Self {
        Self {
            device,
            initialized: false,
        }
    }

    /// Retorna uma referência ao dispositivo CCID subjacente.
    pub fn device(&self) -> &D {
        &self.device
    }

    /// Retorna uma referência mutável ao dispositivo CCID subjacente.
    pub fn device_mut(&mut self) -> &mut D {
        &mut self.device
    }
}

#[cfg(feature = "embedded")]
impl<D: UsbCcidDevice + Send + Sync> Transport for FramedCcidTransport<D> {
    fn init(&mut self) -> Result<(), TransportError> {
        self.device.init().map_err(TransportError::from)?;
        self.initialized = true;
        Ok(())
    }

    fn send(&mut self, data: &[u8]) -> Result<(), TransportError> {
        if !self.initialized {
            return Err(TransportError::NotInitialized);
        }

        let resp = ApduResponse::success(data.to_vec());
        let apdu_bytes = resp.to_bytes();

        self.device
            .send_ccid_block(&apdu_bytes)
            .map_err(TransportError::from)?;

        Ok(())
    }

    fn recv(&mut self) -> Result<Vec<u8>, TransportError> {
        if !self.initialized {
            return Err(TransportError::NotInitialized);
        }

        let mut buf = vec![0u8; self.device.max_transfer_size()];
        let len = self
            .device
            .recv_ccid_block(&mut buf)
            .map_err(TransportError::from)?;

        // O parser ISO canônico cobre os casos curtos e estendidos e rejeita
        // truncamento/trailing bytes antes de entregar somente o payload.
        let apdu = Apdu::parse(&buf[..len])
            .map_err(|_| TransportError::RecvError("framing".to_string()))?;

        Ok(apdu.data.to_vec())
    }

    fn close(&mut self) -> Result<(), TransportError> {
        self.initialized = false;
        Ok(())
    }
}

#[cfg(all(test, feature = "embedded"))]
mod tests {
    use super::*;
    use transport_core::embedded::EmbeddedTransportError;

    struct MockCcid {
        sent_blocks: Vec<Vec<u8>>,
        recv_data: Vec<u8>,
        initialized: bool,
        fail_init: bool,
    }

    impl MockCcid {
        fn new(recv_data: Vec<u8>) -> Self {
            Self {
                sent_blocks: Vec::new(),
                recv_data,
                initialized: false,
                fail_init: false,
            }
        }

        fn failing_init() -> Self {
            Self {
                sent_blocks: Vec::new(),
                recv_data: Vec::new(),
                initialized: false,
                fail_init: true,
            }
        }
    }

    impl UsbCcidDevice for MockCcid {
        fn init(&mut self) -> Result<(), EmbeddedTransportError> {
            if self.fail_init {
                return Err(EmbeddedTransportError::SendFailed);
            }
            self.initialized = true;
            Ok(())
        }

        fn send_ccid_block(&mut self, buf: &[u8]) -> Result<(), EmbeddedTransportError> {
            if !self.initialized {
                return Err(EmbeddedTransportError::NotInitialized);
            }
            self.sent_blocks.push(buf.to_vec());
            Ok(())
        }

        fn recv_ccid_block(&mut self, buf: &mut [u8]) -> Result<usize, EmbeddedTransportError> {
            if !self.initialized {
                return Err(EmbeddedTransportError::NotInitialized);
            }
            if self.recv_data.is_empty() {
                return Err(EmbeddedTransportError::Timeout);
            }
            let len = self.recv_data.len();
            buf[..len].copy_from_slice(&self.recv_data);
            Ok(len)
        }
    }

    /// Caso 3E/4E ISO 7816-4: `[header][00][LcHi LcLo][data][LeHi LeLo]`.
    fn extended_apdu(data: &[u8], le: Option<u16>) -> Vec<u8> {
        let mut raw = vec![0x80, 0x10, 0x00, 0x00, 0x00];
        raw.extend_from_slice(&(data.len() as u16).to_be_bytes());
        raw.extend_from_slice(data);
        if let Some(le) = le {
            raw.extend_from_slice(&le.to_be_bytes());
        }
        raw
    }

    /// Caso 2E puro: `[header][00][LeHi LeLo]`.
    fn extended_le_only(le: u16) -> Vec<u8> {
        let mut raw = vec![0x80, 0xC0, 0x00, 0x00, 0x00];
        raw.extend_from_slice(&le.to_be_bytes());
        raw
    }

    #[test]
    fn test_framed_ccid_send_recv() {
        // Raw APDU: CLA=0x00 INS=0x10 P1=0x00 P2=0x00 Lc=0x03 Data=[1,2,3]
        let raw_apdu = vec![0x00, 0x10, 0x00, 0x00, 0x03, 1, 2, 3];
        let mock = MockCcid::new(raw_apdu);
        let mut transport = FramedCcidTransport::new(mock);
        transport.init().unwrap();

        let received = transport.recv().unwrap();
        assert_eq!(received, vec![1, 2, 3]);

        transport.send(&[4, 5, 6]).unwrap();
        let sent = &transport.device().sent_blocks[0];
        // Expect data + SW1(0x90) + SW2(0x00)
        assert_eq!(sent, &vec![4, 5, 6, 0x90, 0x00]);
    }

    #[test]
    fn test_framed_ccid_extended_apdu_matches_canonical_parser() {
        let data: Vec<u8> = (0..300).map(|i| i as u8).collect();
        let raw_apdu = extended_apdu(&data, Some(0x0100));
        let canonical = Apdu::parse(&raw_apdu).unwrap();
        assert_eq!(canonical.data, data);
        assert_eq!(canonical.le, Some(256));

        let mut transport = FramedCcidTransport::new(MockCcid::new(raw_apdu.clone()));
        transport.init().unwrap();
        assert_eq!(transport.recv().unwrap(), canonical.data);
    }

    #[test]
    fn test_framed_ccid_short_and_extended_le_are_consumed_without_data() {
        let cases = [
            (vec![0x00, 0xC0, 0x00, 0x00, 0x00], Some(256)),
            (extended_le_only(0x0100), Some(256)),
            (extended_le_only(0x0000), Some(65_536)),
        ];

        for (raw_apdu, expected_le) in cases {
            let canonical = Apdu::parse(&raw_apdu).unwrap();
            assert!(canonical.data.is_empty());
            assert_eq!(canonical.le, expected_le);

            let mut transport = FramedCcidTransport::new(MockCcid::new(raw_apdu));
            transport.init().unwrap();
            assert!(transport.recv().unwrap().is_empty());
        }
    }

    #[test]
    fn test_framed_ccid_truncated_apdu_maps_to_framing_receive_error() {
        let cases = [
            // Lc curto declara três bytes, mas só um chegou.
            vec![0x00, 0x10, 0x00, 0x00, 0x03, 0xAA],
            // Lc estendido declara 0x012C bytes, mas só um chegou.
            vec![0x80, 0x10, 0x00, 0x00, 0x00, 0x01, 0x2C, 0xAA],
            // O caso 4E tem apenas um byte sobrando depois dos dados, em vez
            // dos dois bytes de Le.
            vec![0x80, 0x10, 0x00, 0x00, 0x00, 0x00, 0x01, 0xAA, 0xFF],
        ];

        for raw_apdu in cases {
            assert!(Apdu::parse(&raw_apdu).is_err());
            let mut transport = FramedCcidTransport::new(MockCcid::new(raw_apdu));
            transport.init().unwrap();
            assert!(matches!(
                transport.recv(),
                Err(TransportError::RecvError(msg)) if msg == "framing"
            ));
        }
    }

    #[test]
    fn test_framed_ccid_send_appends_success_status_words() {
        let mut transport = FramedCcidTransport::new(MockCcid::new(Vec::new()));
        transport.init().unwrap();
        transport.send(&[0xDE, 0xAD]).unwrap();

        assert_eq!(
            transport.device().sent_blocks,
            vec![ApduResponse::success(vec![0xDE, 0xAD]).to_bytes()]
        );
    }

    #[test]
    fn test_framed_ccid_io_before_init_returns_not_initialized() {
        let raw_apdu = vec![0x00, 0x10, 0x00, 0x00, 0x03, 1, 2, 3];
        let mut transport = FramedCcidTransport::new(MockCcid::new(raw_apdu));
        assert!(matches!(
            transport.send(b"x"),
            Err(TransportError::NotInitialized)
        ));
        assert!(matches!(
            transport.recv(),
            Err(TransportError::NotInitialized)
        ));
    }

    #[test]
    fn test_framed_ccid_init_failure_propagates() {
        let mut transport = FramedCcidTransport::new(MockCcid::failing_init());
        assert!(matches!(
            transport.init(),
            Err(TransportError::SendError(_))
        ));
        // Falha no init não pode marcar o transporte como inicializado.
        assert!(matches!(
            transport.send(b"x"),
            Err(TransportError::NotInitialized)
        ));
        assert!(matches!(
            transport.recv(),
            Err(TransportError::NotInitialized)
        ));
    }

    #[test]
    fn test_framed_ccid_empty_recv_returns_timeout() {
        let mut transport = FramedCcidTransport::new(MockCcid::new(Vec::new()));
        transport.init().unwrap();
        match transport.recv() {
            Err(TransportError::RecvError(msg)) => assert!(msg.contains("timeout")),
            other => panic!("expected timeout RecvError, got {:?}", other),
        }
    }

    #[test]
    fn test_framed_ccid_close_resets_and_reinit_roundtrips() {
        let raw_apdu = vec![0x00, 0x10, 0x00, 0x00, 0x03, 1, 2, 3];
        let mut transport = FramedCcidTransport::new(MockCcid::new(raw_apdu));
        transport.init().unwrap();
        assert_eq!(transport.recv().unwrap(), vec![1, 2, 3]);

        assert!(transport.close().is_ok());
        assert!(matches!(
            transport.send(b"x"),
            Err(TransportError::NotInitialized)
        ));
        assert!(matches!(
            transport.recv(),
            Err(TransportError::NotInitialized)
        ));

        transport.init().unwrap();
        assert_eq!(transport.recv().unwrap(), vec![1, 2, 3]);
        transport.send(&[7, 8]).unwrap();
        let sent = transport.device().sent_blocks.last().unwrap();
        assert_eq!(sent, &vec![7, 8, 0x90, 0x00]);
    }
}
