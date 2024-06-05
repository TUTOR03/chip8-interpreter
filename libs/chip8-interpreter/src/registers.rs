use std::ops::{Index, IndexMut};

use super::nibble::Nibble;

pub struct Registers([u8; Nibble::SIZE]);

impl Registers {
  #[inline]
  pub fn new() -> Self {
    Self([0; Nibble::SIZE])
  }
}

impl Default for Registers {
  fn default() -> Self {
    Self::new()
  }
}

impl Index<Nibble> for Registers {
  type Output = u8;

  #[inline]
  fn index(&self, index: Nibble) -> &Self::Output {
    &self.0[index.as_usize()]
  }
}

impl IndexMut<Nibble> for Registers {
  #[inline]
  fn index_mut(&mut self, index: Nibble) -> &mut Self::Output {
    &mut self.0[index.as_usize()]
  }
}
