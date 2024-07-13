use std::ops::{Index, IndexMut};

use crate::Nibble;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Key(u8);

impl Key {
  pub fn new() -> Self {
    Self(0)
  }

  pub fn is_down(&self) -> bool {
    self.0 & 0x1 >= 1
  }

  pub fn set_is_down(&mut self, is_down: bool) {
    self.0 = (self.0 & 0xFE) | is_down as u8;
  }

  pub fn is_trackable(&self) -> bool {
    self.0 & 0x2 >= 1
  }

  pub fn set_is_trackable(&mut self, is_trackable: bool) {
    self.0 = (self.0 & 0xFD) | ((is_trackable as u8) << 1);
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct Keyboard([Key; Nibble::SIZE]);

impl Keyboard {
  pub fn new() -> Self {
    Self([Key::new(); Nibble::SIZE])
  }

  pub fn iter(&self) -> impl Iterator<Item = &Key> {
    self.0.iter()
  }

  pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Key> {
    self.0.iter_mut()
  }
}

impl Index<Nibble> for Keyboard {
  type Output = Key;

  fn index(&self, index: Nibble) -> &Self::Output {
    &self.0[index.as_usize()]
  }
}

impl IndexMut<Nibble> for Keyboard {
  fn index_mut(&mut self, index: Nibble) -> &mut Self::Output {
    &mut self.0[index.as_usize()]
  }
}
