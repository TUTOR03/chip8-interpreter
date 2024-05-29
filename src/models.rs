use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Nibble(u8);

impl Nibble {
  pub const WIDTH: usize = 4;
  pub const SIZE: usize = 2usize.pow(Self::WIDTH as u32);
  pub const MAX: Self = Self((Self::SIZE - 1) as u8);

  pub fn new<const VALUE: u8>() -> Self {
    debug_assert!(VALUE as usize <= Self::SIZE, "Nibble overflow");
    Self(VALUE % Self::SIZE as u8)
  }

  pub fn as_u8(self) -> u8 {
    self.0
  }

  pub fn as_usize(self) -> usize {
    self.0 as usize
  }

  pub fn as_i16(self) -> i16 {
    self.0 as i16
  }
}

impl TryFrom<u8> for Nibble {
  type Error = ();

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    if value > Self::MAX.0 {
      return Err(());
    }

    Ok(Self(value))
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Address(u16);

impl Address {
  pub const WIDTH: usize = 12;
  pub const SIZE: usize = 2usize.pow(Self::WIDTH as u32);
  pub const MAX: Self = Self((Self::SIZE - 1) as u16);

  pub const fn new<const VALUE: u16>() -> Self {
    debug_assert!(VALUE as usize <= Self::SIZE, "Address overflow");
    Self(VALUE % Self::SIZE as u16)
  }

  pub fn as_u16(self) -> u16 {
    self.0
  }

  pub fn as_usize(self) -> usize {
    self.0 as usize
  }
}

impl TryFrom<u16> for Address {
  type Error = ();

  fn try_from(value: u16) -> Result<Self, Self::Error> {
    if value > Self::MAX.0 {
      return Err(());
    }

    Ok(Self(value))
  }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<i16> for Address {
  type Output = Address;

  fn add(self, rhs: i16) -> Self::Output {
    Address(self.0.wrapping_add_signed(rhs) % Self::SIZE as u16)
  }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<i16> for Address {
  fn add_assign(&mut self, rhs: i16) {
    self.0 = self.0.wrapping_add_signed(rhs) % Self::SIZE as u16
  }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpCode(u16);

impl OpCode {
  pub fn from_bytes(first: u8, second: u8) -> Self {
    Self(((first as u16) << 8) + second as u16)
  }

  pub fn get_nibble(self, index: u32) -> Nibble {
    Nibble::try_from(self.get_part(index, 4) as u8).unwrap()
  }

  pub fn get_word(self, index: u32) -> u8 {
    self.get_part(index, 8) as u8
  }

  pub fn get_address(self) -> Address {
    Address::try_from(self.get_part(0, 12)).unwrap()
  }

  fn get_part(self, index: u32, width: u32) -> u16 {
    let power = (index + 1) * width;
    let mask = (2usize.pow(power) - 1) as u16;
    (self.0 & mask).checked_shr(index * width).unwrap_or(0)
  }
}

impl From<OpCode> for u16 {
  fn from(value: OpCode) -> Self {
    value.0
  }
}

pub struct Stack<const S: usize> {
  items: [Address; S],
  index: usize,
  is_empty: bool,
}

#[allow(clippy::new_without_default)]
impl<const S: usize> Stack<S> {
  pub fn new() -> Self {
    Self {
      items: [Address::new::<0>(); S],
      index: 0,
      is_empty: true,
    }
  }

  pub fn push(&mut self, value: Address) -> Option<()> {
    if self.index == S - 1 {
      return Some(());
    }

    self.index += 1;
    self.is_empty &= false;
    self.items[self.index] = value;
    None
  }

  pub fn pop(&mut self) -> Option<Address> {
    if self.is_empty {
      return None;
    }

    let value = self.items[self.index];
    self.items[self.index] = Address::new::<0>();
    if self.index != 0 {
      self.index -= 1;
    } else {
      self.is_empty = true;
    }

    Some(value)
  }
}
