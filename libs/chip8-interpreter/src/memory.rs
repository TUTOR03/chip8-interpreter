use std::ops::{Index, IndexMut, Range};

use super::address::Address;

pub struct Memory([u8; Address::SIZE]);

impl Memory {
  #[inline]
  pub fn new() -> Self {
    Self([0; Address::SIZE])
  }
}

impl Default for Memory {
  fn default() -> Self {
    Self::new()
  }
}

impl Index<Address> for Memory {
  type Output = u8;

  #[inline]
  fn index(&self, index: Address) -> &Self::Output {
    &self.0[index.as_usize()]
  }
}

impl IndexMut<Address> for Memory {
  #[inline]
  fn index_mut(&mut self, index: Address) -> &mut Self::Output {
    &mut self.0[index.as_usize()]
  }
}

impl Index<Range<Address>> for Memory {
  type Output = [u8];

  #[inline]
  fn index(&self, range: Range<Address>) -> &Self::Output {
    &self.0[range.start.as_usize()..range.end.as_usize()]
  }
}

impl IndexMut<Range<Address>> for Memory {
  #[inline]
  fn index_mut(&mut self, range: Range<Address>) -> &mut Self::Output {
    &mut self.0[range.start.as_usize()..range.end.as_usize()]
  }
}
