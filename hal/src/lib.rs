//! Hardware Abstraction Layer (HAL) for OpenKey FIDO2.

pub mod gpio;
pub mod led;
pub mod rng;
pub mod storage;
pub mod timer;

#[path = "secure-element.rs"]
pub mod secure_element;

pub use gpio::{InputPin, OutputPin};
pub use led::LedIndicator;
pub use rng::HardwareRng;
pub use secure_element::SecureElement;
pub use storage::FlashStorage;
pub use timer::HardwareTimer;
