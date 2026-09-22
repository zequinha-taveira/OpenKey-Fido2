//! Device discovery across USB HID and serial ports.

#[derive(Debug, Default)]
pub struct DeviceDiscovery;

impl DeviceDiscovery {
    pub fn scan() -> Vec<String> {
        Vec::new()
    }
}
