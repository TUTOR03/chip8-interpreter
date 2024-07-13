use crate::{address::Address, memory::Memory};

pub trait Executable {
  fn load_into_memory(&self, memory: &mut Memory);
  fn get_entry_point(&self) -> Address;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct BaseExecutable<'a> {
  image: &'a [u8],
  load_point: Address,
}

impl<'a> BaseExecutable<'a> {
  pub fn new(image: &'a [u8], load_point: Address) -> Self {
    Self { image, load_point }
  }
}

impl<'a> Executable for BaseExecutable<'a> {
  fn load_into_memory(&self, memory: &mut Memory) {
    let mem_end = self.load_point + (self.image.len() - 1) as i16;
    memory[self.load_point..=mem_end].copy_from_slice(self.image);
  }

  fn get_entry_point(&self) -> Address {
    Address::new::<0x200>()
  }
}
