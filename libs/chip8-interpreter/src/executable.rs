use crate::{address::Address, memory::Memory};

pub trait Executable {
  fn load_into_memory(&self, memory: &mut Memory);
  fn get_entry_point(&self) -> Address;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
pub struct BaseExecutable<'a>(&'a [u8]);

impl<'a> BaseExecutable<'a> {
  pub fn new(data: &'a [u8]) -> Self {
    Self(data)
  }
}

impl<'a> Executable for BaseExecutable<'a> {
  fn load_into_memory(&self, memory: &mut Memory) {
    let mem_start = Address::new::<0x200>();
    let mem_end = mem_start + self.0.len() as i16;

    memory[mem_start..mem_end].copy_from_slice(self.0);
  }

  fn get_entry_point(&self) -> Address {
    Address::new::<0x200>()
  }
}
