//! Binary-coded-decimal values for the CIA TOD registers.
//!
//! Construct and read as ordinary decimal; the BCD packing stays internal.

/// A decimal value constrained to `MIN..=MAX`, stored as its packed BCD byte.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Bcd<const MIN: u8, const MAX: u8>(u8);

impl<const MIN: u8, const MAX: u8> Bcd<MIN, MAX> {
    /// From a decimal value; `None` if outside `MIN..=MAX`. `const`, so
    /// `Seconds::new(59).unwrap()` is range-checked at compile time.
    pub const fn new(decimal: u8) -> Option<Self> {
        if decimal < MIN || decimal > MAX {
            return None;
        }
        Some(Self((decimal / 10) << 4 | (decimal % 10)))
    }

    /// The decimal value.
    pub const fn get(self) -> u8 { (self.0 >> 4) * 10 + (self.0 & 0x0f) }

    /// The raw BCD byte, to write into the register.
    pub const fn to_bcd(self) -> u8 { self.0 }

    /// From a BCD byte read from the register; `None` if a nibble is not a
    /// decimal digit or the decoded value is out of range.
    pub const fn from_bcd(byte: u8) -> Option<Self> {
        if (byte >> 4) > 9 || (byte & 0x0f) > 9 {
            return None;
        }
        Self::new((byte >> 4) * 10 + (byte & 0x0f))
    }
}

impl<const MIN: u8, const MAX: u8> core::fmt::Debug for Bcd<MIN, MAX> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result { write!(f, "{}", self.get()) }
}

/// TOD tenths of a second (0-9).
pub type Tenths = Bcd<0, 9>;
/// TOD seconds (0-59).
pub type Seconds = Bcd<0, 59>;
/// TOD minutes (0-59).
pub type Minutes = Bcd<0, 59>;
/// TOD hours, 12-hour (1-12).
pub type Hours = Bcd<1, 12>;
