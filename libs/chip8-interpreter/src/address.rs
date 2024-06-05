use std::fmt;
use std::ops::{Add, AddAssign};

use crate::errors::InterpreterError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Address(u16);

impl Address {
  pub const WIDTH: usize = 12;
  pub const SIZE: usize = 1 << Self::WIDTH;
  pub const MAX: Self = Self((Self::SIZE - 1) as u16);

  #[inline]
  pub fn new<const VALUE: u16>() -> Self {
    Self(VALUE % Self::SIZE as u16)
  }

  #[inline]
  pub fn as_u16(self) -> u16 {
    self.0
  }

  #[inline]
  pub fn as_usize(self) -> usize {
    self.0 as usize
  }
}

impl fmt::Display for Address {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl TryFrom<u16> for Address {
  type Error = InterpreterError;

  #[inline]
  fn try_from(value: u16) -> Result<Self, Self::Error> {
    if value > Self::MAX.0 {
      return Err(InterpreterError::AddressOverflow);
    }

    Ok(Self(value))
  }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<i16> for Address {
  type Output = Address;

  #[inline]
  fn add(self, rhs: i16) -> Self::Output {
    Address(self.0.wrapping_add_signed(rhs) % Self::SIZE as u16)
  }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<i16> for Address {
  #[inline]
  fn add_assign(&mut self, rhs: i16) {
    self.0 = self.0.wrapping_add_signed(rhs) % Self::SIZE as u16
  }
}
