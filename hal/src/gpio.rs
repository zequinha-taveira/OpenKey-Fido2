//! General Purpose Input/Output (GPIO) interface.

/// GPIO Pin Input trait.
pub trait InputPin {
    type Error: core::fmt::Debug;
    fn is_high(&self) -> Result<bool, Self::Error>;
    fn is_low(&self) -> Result<bool, Self::Error>;
}

/// GPIO Pin Output trait.
pub trait OutputPin {
    type Error: core::fmt::Debug;
    fn set_high(&mut self) -> Result<(), Self::Error>;
    fn set_low(&mut self) -> Result<(), Self::Error>;
}
