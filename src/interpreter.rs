use crate::{
  executable::Executable,
  models::{Address, Nibble, OpCode, Stack},
  platform::{Platform, Point, Sprite},
};

pub type Memory = [u8; Address::SIZE];

pub struct Interpreter<P: Platform> {
  platform: P,
  registres: [u8; 16],
  index_register: Address,
  memory: Memory,
  instruction_address: Address,
  stack: Stack<16>,
}

const ADDRESS_BYTE_STEP: i16 = 2;

impl<P: Platform> Interpreter<P> {
  pub fn new<E: Executable>(platform: P, executable: E) -> Self {
    let mut res = Self {
      platform,
      registres: [0; 16],
      index_register: Address::new::<0>(),
      memory: [0; Address::SIZE],
      stack: Stack::new(),
      instruction_address: executable.get_entry_point(),
    };

    executable.load_into_memory(&mut res.memory);
    res
  }

  pub fn run_one_instruction(&mut self) -> Result<(), ()> {
    let next_op_code = OpCode::from_bytes(
      self.memory[self.instruction_address.as_u16() as usize],
      self.memory[self.instruction_address.as_u16() as usize],
    );

    let instruction = match Instruction::try_from(next_op_code) {
      Ok(instruction) => instruction,
      Err(err) => return Err(err),
    };

    let mut instruction_step: i16 = 1;
    match instruction {
      Instruction::SkipIfEqual(reg_index, value) => {
        if self.registres[reg_index.as_usize()] == value {
          instruction_step = 2;
        }
      }

      Instruction::SkipIfNotEqual(reg_index, value) => {
        if self.registres[reg_index.as_usize()] != value {
          instruction_step = 2;
        }
      }

      Instruction::SkipIfRegistersEqual(x_reg_index, y_reg_index) => {
        let reg_x = self.registres[x_reg_index.as_usize()];
        let reg_y = self.registres[y_reg_index.as_usize()];
        if reg_x == reg_y {
          instruction_step = 2;
        }
      }

      Instruction::SkipIfRegistersNotEqual(x_reg_index, y_reg_index) => {
        let reg_x = self.registres[x_reg_index.as_usize()];
        let reg_y = self.registres[y_reg_index.as_usize()];
        if reg_x != reg_y {
          instruction_step = 2;
        }
      }

      Instruction::SetRegister(reg_index, value) => {
        self.registres[reg_index.as_usize()] = value;
      }

      Instruction::CopyRegister(x_reg_index, y_reg_index) => {
        self.registres[x_reg_index.as_usize()] = self.registres[y_reg_index.as_usize()];
      }

      Instruction::SetAddressRegister(address) => {
        self.index_register = address;
      }

      Instruction::SetRegisterRandom(reg_index, mask) => {
        self.registres[reg_index.as_usize()] = self.platform.get_random_byte() & mask;
      }

      Instruction::WriteRegistersToMem(end_index) => {
        (0..=end_index.as_usize()).for_each(|reg_index| {
          let address = self.index_register + reg_index as i16;
          self.memory[address.as_usize()] = self.registres[reg_index];
        });
      }

      Instruction::LoadRegistersFromMem(end_index) => {
        (0..=end_index.as_usize()).for_each(|reg_index| {
          let address = self.index_register + reg_index as i16;
          self.registres[reg_index] = self.memory[address.as_usize()]
        });
      }

      Instruction::RegisterToBCD(reg_index) => {
        let reg = self.registres[reg_index.as_usize()];

        (0..=2).for_each(|offset| {
          self.memory[self.index_register.as_usize() + offset] =
            (reg / (10 * (offset + 1)) as u8) % 10;
        })
      }

      Instruction::AddRegister(reg_index, value) => {
        self.registres[reg_index.as_usize()] += value;
      }

      Instruction::OrRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registres[x_reg_index.as_usize()];
        let reg_y = self.registres[y_reg_index.as_usize()];
        self.registres[x_reg_index.as_usize()] = reg_x | reg_y;
      }

      Instruction::AndRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registres[x_reg_index.as_usize()];
        let reg_y = self.registres[y_reg_index.as_usize()];
        self.registres[x_reg_index.as_usize()] = reg_x & reg_y;
      }

      Instruction::XorRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registres[x_reg_index.as_usize()];
        let reg_y = self.registres[y_reg_index.as_usize()];
        self.registres[x_reg_index.as_usize()] = reg_x ^ reg_y;
      }

      Instruction::AddAddressRegister(reg_index) => {
        self.index_register += self.registres[reg_index.as_usize()] as i16;
      }

      Instruction::AddRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registres[x_reg_index.as_usize()];
        let reg_y = self.registres[y_reg_index.as_usize()];
        match reg_x.checked_add(reg_y) {
          Some(value) => {
            self.registres[x_reg_index.as_usize()] = value;
            self.registres[15] = 0;
          }
          None => {
            self.registres[15] = 1;
          }
        }
      }

      Instruction::SubRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registres[x_reg_index.as_usize()];
        let reg_y = self.registres[y_reg_index.as_usize()];
        match reg_x.checked_sub(reg_y) {
          Some(value) => {
            self.registres[x_reg_index.as_usize()] = value;
            self.registres[15] = 1;
          }
          None => {
            self.registres[15] = 0;
          }
        }
      }

      Instruction::RShiftRegisters(x_reg_index, y_reg_index) => {
        let reg_y = self.registres[y_reg_index.as_usize()];
        self.registres[x_reg_index.as_usize()] = reg_y >> 1;
        self.registres[15] = reg_y & 0x1;
      }

      Instruction::SubRegisterReversed(x_reg_index, y_reg_index) => {
        let reg_x = self.registres[x_reg_index.as_usize()];
        let reg_y = self.registres[y_reg_index.as_usize()];
        match reg_y.checked_sub(reg_x) {
          Some(value) => {
            self.registres[x_reg_index.as_usize()] = value;
            self.registres[15] = 1;
          }
          None => {
            self.registres[15] = 0;
          }
        }
      }

      Instruction::LShiftRegisters(x_reg_index, y_reg_index) => {
        let reg_y = self.registres[y_reg_index.as_usize()];
        self.registres[x_reg_index.as_usize()] = reg_y << 1;
        self.registres[15] = reg_y & 0x80;
      }

      Instruction::ClearScreen => self.platform.clear_screen(),

      Instruction::DrawSprite(x_reg, y_reg, rows_count) => {
        let mem_start = self.index_register.as_usize();
        let mem_end = (self.index_register + rows_count.as_i16()).as_usize();
        let sprite = Sprite::new(&self.memory[mem_start..mem_end]);

        let pos = Point {
          x: self.registres[x_reg.as_usize()],
          y: self.registres[y_reg.as_usize()],
        };
        self.registres[15] = self.platform.draw_sprite(pos, sprite) as u8;
      }

      Instruction::Jump(address) => {
        self.instruction_address = address;
        return Ok(());
      }

      Instruction::Call(address) => {
        self.stack.push(address);
        self.instruction_address = address + instruction_step * ADDRESS_BYTE_STEP;
        return Ok(());
      }

      Instruction::Return => {
        if let Some(address) = self.stack.pop() {
          self.instruction_address = address;
        }

        return Ok(());
      }

      Instruction::JumpV0(address) => {
        self.instruction_address = address + self.registres[0] as i16;
        return Ok(());
      }
    }

    self.instruction_address += instruction_step * ADDRESS_BYTE_STEP;
    Ok(())
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
  /* Условия */
  SkipIfEqual(Nibble, u8),
  SkipIfNotEqual(Nibble, u8),
  SkipIfRegistersEqual(Nibble, Nibble),
  SkipIfRegistersNotEqual(Nibble, Nibble),
  /* Запись регистров */
  SetRegister(Nibble, u8),
  CopyRegister(Nibble, Nibble),
  SetAddressRegister(Address),
  SetRegisterRandom(Nibble, u8),
  WriteRegistersToMem(Nibble),
  LoadRegistersFromMem(Nibble),
  RegisterToBCD(Nibble),
  /* Математические операции */
  AddRegister(Nibble, u8),
  OrRegisters(Nibble, Nibble),
  AndRegisters(Nibble, Nibble),
  XorRegisters(Nibble, Nibble),
  AddAddressRegister(Nibble),
  AddRegisters(Nibble, Nibble),
  SubRegisters(Nibble, Nibble),
  RShiftRegisters(Nibble, Nibble),
  SubRegisterReversed(Nibble, Nibble),
  LShiftRegisters(Nibble, Nibble),
  /* Экран */
  ClearScreen,
  DrawSprite(Nibble, Nibble, Nibble),
  /* Работа с адресами */
  Jump(Address),
  JumpV0(Address),
  Call(Address),
  Return,
}

impl TryFrom<OpCode> for Instruction {
  type Error = ();

  fn try_from(code: OpCode) -> Result<Self, Self::Error> {
    let code_nibbles = (
      code.get_nibble(3).as_u8(),
      code.get_nibble(2).as_u8(),
      code.get_nibble(1).as_u8(),
      code.get_nibble(0).as_u8(),
    );

    let instruction = match code_nibbles {
      /* Условия */
      (0x3, _, _, _) => Instruction::SkipIfEqual(code.get_nibble(2), code.get_word(0)),
      (0x4, _, _, _) => Instruction::SkipIfNotEqual(code.get_nibble(2), code.get_word(0)),
      (0x5, _, _, 0x0) => Instruction::SkipIfRegistersEqual(code.get_nibble(2), code.get_nibble(1)),
      (0x9, _, _, 0x0) => {
        Instruction::SkipIfRegistersNotEqual(code.get_nibble(2), code.get_nibble(1))
      }
      /* Запись регистров */
      (0x6, _, _, _) => Instruction::SetRegister(code.get_nibble(2), code.get_word(0)),
      (0x8, _, _, 0x0) => Instruction::CopyRegister(code.get_nibble(2), code.get_nibble(1)),
      (0xA, _, _, _) => Instruction::SetAddressRegister(code.get_address()),
      (0xC, _, _, _) => Instruction::SetRegisterRandom(code.get_nibble(2), code.get_word(0)),
      (0xF, _, 0x5, 0x5) => Instruction::WriteRegistersToMem(code.get_nibble(2)),
      (0xF, _, 0x6, 0x5) => Instruction::LoadRegistersFromMem(code.get_nibble(2)),
      (0xF, _, 0x3, 0x3) => Instruction::RegisterToBCD(code.get_nibble(2)),
      /* Математические операции */
      (0x7, _, _, _) => Instruction::AddRegister(code.get_nibble(2), code.get_word(0)),
      (0x8, _, _, 0x1) => Instruction::OrRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, _, _, 0x2) => Instruction::AndRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, _, _, 0x3) => Instruction::XorRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0xF, _, 0x1, 0xE) => Instruction::AddAddressRegister(code.get_nibble(2)),
      (0x8, _, _, 0x4) => Instruction::AddRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, _, _, 0x5) => Instruction::SubRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, _, _, 0x6) => Instruction::RShiftRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, _, _, 0x7) => Instruction::SubRegisterReversed(code.get_nibble(2), code.get_nibble(1)),
      (0x8, _, _, 0xE) => Instruction::LShiftRegisters(code.get_nibble(2), code.get_nibble(1)),
      /* Экран */
      (0x0, 0x0, 0xE, 0x0) => Instruction::ClearScreen,
      (0xD, _, _, _) => {
        Instruction::DrawSprite(code.get_nibble(2), code.get_nibble(1), code.get_nibble(0))
      }
      /* Работа с адресами */
      (0x1, _, _, _) => Instruction::Jump(code.get_address()),
      (0xB, _, _, _) => Instruction::JumpV0(code.get_address()),
      (0x2, _, _, _) => Instruction::Call(code.get_address()),
      (0x0, 0x0, 0xE, 0xE) => Instruction::Return,
      _ => return Err(()),
    };
    Ok(instruction)
  }
}
