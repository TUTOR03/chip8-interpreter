use std::fmt;

use super::errors::InterpreterError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Nibble(u8);

impl Nibble {
  pub const WIDTH: usize = 4;
  pub const SIZE: usize = 1 << Self::WIDTH;
  pub const MAX: Self = Self((Self::SIZE - 1) as u8);

  pub fn new<const VALUE: u8>() -> Self {
    Self(VALUE % Self::SIZE as u8)
  }

  pub fn as_u8(self) -> u8 {
    self.0
  }

  pub fn as_usize(self) -> usize {
    self.0 as usize
  }
}

impl fmt::Display for Nibble {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TryFrom<u8> for Nibble {
  type Error = InterpreterError;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    if value > Self::MAX.0 {
      return Err(InterpreterError::NibbleOverflow);
    }

    Ok(Self(value))
  }
}
