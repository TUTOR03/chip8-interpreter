use crate::{interpreter::Memory, models::Address};

pub trait Executable {
  fn load_into_memory(&self, memory: &mut Memory);
  fn get_entry_point(&self) -> Address;
}

pub struct Chip8BaseExecutable<'a>(&'a [u8]);

impl<'a> Chip8BaseExecutable<'a> {
  const ENTRY_ADDRESS: Address = Address::new::<0x200>();

  pub fn new(data: &'a [u8]) -> Self {
    Self(data)
  }
}

impl<'a> Executable for Chip8BaseExecutable<'a> {
  fn load_into_memory(&self, memory: &mut Memory) {
    let mem_start = Self::ENTRY_ADDRESS.as_usize();
    let mem_end = mem_start + self.0.len();

    memory[mem_start..mem_end].copy_from_slice(self.0);
  }

  fn get_entry_point(&self) -> Address {
    Self::ENTRY_ADDRESS
  }
}
