use std::ops::{Index, IndexMut};

use super::nibble::Nibble;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Registers([u8; Nibble::SIZE]);

impl Registers {
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

  fn index(&self, index: Nibble) -> &Self::Output {
    &self.0[index.as_usize()]
  }
}

impl IndexMut<Nibble> for Registers {
  fn index_mut(&mut self, index: Nibble) -> &mut Self::Output {
    &mut self.0[index.as_usize()]
  }
}
