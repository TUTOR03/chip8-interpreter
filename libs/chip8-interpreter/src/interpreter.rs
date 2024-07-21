use std::fmt;

use crate::{
  address::Address,
  errors::InterpreterError,
  executable::Executable,
  memory::Memory,
  nibble::Nibble,
  platform::Platform,
  registers::Registers,
  sprite::{Point, Sprite},
  stack::Stack,
  SCREEN_HEIGHT, SCREEN_WIDTH,
};

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Interpreter<P: Platform> {
  platform: P,
  registers: Registers,
  index_register: Address,
  memory: Memory,
  instruction_address: Address,
  stack: Stack<Address, 16>,
  expecting_key: Option<Nibble>,
  is_crashed: bool,
}

const ADDRESS_BYTE_STEP: i16 = 2;

impl<P: Platform> Interpreter<P> {
  pub fn new<E: Executable>(platform: P, executable: E) -> Self {
    let mut res = Self {
      platform,
      registers: Registers::new(),
      index_register: Address::default(),
      memory: Memory::new(),
      stack: Stack::new(),
      instruction_address: executable.get_entry_point(),
      expecting_key: None,
      is_crashed: false,
    };

    executable.load_into_memory(&mut res.memory);
    res
  }

  pub fn get_platform_mut(&mut self) -> &mut P {
    &mut self.platform
  }

  pub fn run_next(&mut self) -> Result<(), InterpreterError> {
    if self.is_crashed {
      return Err(InterpreterError::Crashed);
    }

    if let Some(expecting_key_reg_index) = self.expecting_key {
      let Some(pressed_key) = self.platform.get_last_pressed_key() else {
        return Ok(());
      };

      self.registers[expecting_key_reg_index] = pressed_key.as_u8();
      self.expecting_key = None;
    }

    let instruction_res = self.process_next_instruction();
    if instruction_res.is_err() {
      self.is_crashed = true;
      return instruction_res;
    }

    Ok(())
  }

  fn process_next_instruction(&mut self) -> Result<(), InterpreterError> {
    let next_op_code = OpCode::from_bytes(
      self.memory[self.instruction_address],
      self.memory[self.instruction_address + 1],
    );
    let instruction = Instruction::try_from(next_op_code)?;

    let mut instruction_step: i16 = 1;
    match instruction {
      Instruction::SkipIfEqual(reg_index, value) => {
        if self.registers[reg_index] == value {
          instruction_step = 2;
        }
      }

      Instruction::SkipIfNotEqual(reg_index, value) => {
        if self.registers[reg_index] != value {
          instruction_step = 2;
        }
      }

      Instruction::SkipIfRegistersEqual(x_reg_index, y_reg_index) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        if reg_x == reg_y {
          instruction_step = 2;
        }
      }

      Instruction::SkipIfRegistersNotEqual(x_reg_index, y_reg_index) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        if reg_x != reg_y {
          instruction_step = 2;
        }
      }

      Instruction::SkipIfKeyDown(reg_index) => {
        let key = Nibble::try_from(self.registers[reg_index])?;
        if self.platform.is_key_down(key) {
          instruction_step = 2;
        }
      }

      Instruction::SkipIfKeyUp(reg_index) => {
        let key = Nibble::try_from(self.registers[reg_index])?;
        if !self.platform.is_key_down(key) {
          instruction_step = 2;
        }
      }

      Instruction::SetRegister(reg_index, value) => {
        self.registers[reg_index] = value;
      }

      Instruction::CopyRegister(x_reg_index, y_reg_index) => {
        self.registers[x_reg_index] = self.registers[y_reg_index];
      }

      Instruction::SetAddressRegister(address) => {
        self.index_register = address;
      }

      Instruction::SetRegisterRandom(reg_index, mask) => {
        self.registers[reg_index] = self.platform.get_random_byte() & mask;
      }

      Instruction::GetDelayTimer(reg_index) => {
        self.registers[reg_index] = self.platform.get_delay_timer();
      }

      Instruction::WriteRegistersToMem(end_index) => {
        (0..=end_index.as_usize()).for_each(|reg_index| {
          let address = self.index_register + reg_index as i16;
          self.memory[address] = self.registers[Nibble::try_from(reg_index as u8).unwrap()];
        });
        self.index_register += (end_index.as_usize() + 1) as i16;
      }

      Instruction::LoadRegistersFromMem(end_index) => {
        (0..=end_index.as_usize()).for_each(|reg_index| {
          let address = self.index_register + reg_index as i16;
          self.registers[Nibble::try_from(reg_index as u8).unwrap()] = self.memory[address]
        });
        self.index_register += (end_index.as_usize() + 1) as i16;
      }

      Instruction::RegisterToBCD(reg_index) => {
        let reg = self.registers[reg_index];

        (0..=2).for_each(|offset| {
          let dev = 10_u8.pow(2 - offset);
          self.memory[self.index_register + offset as i16] = reg / dev % 10;
        });
      }

      Instruction::AddRegister(reg_index, value) => {
        let reg = self.registers[reg_index];
        self.registers[reg_index] = reg.wrapping_add(value);
      }

      Instruction::OrRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        self.registers[x_reg_index] = reg_x | reg_y;
        self.registers[Nibble::new::<15>()] = 0;
      }

      Instruction::AndRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        self.registers[x_reg_index] = reg_x & reg_y;
        self.registers[Nibble::new::<15>()] = 0;
      }

      Instruction::XorRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        self.registers[x_reg_index] = reg_x ^ reg_y;
        self.registers[Nibble::new::<15>()] = 0;
      }

      Instruction::AddAddressRegister(reg_index) => {
        self.index_register += self.registers[reg_index] as i16;
      }

      Instruction::AddRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        let (value, is_overflowed) = reg_x.overflowing_add(reg_y);
        self.registers[x_reg_index] = value;
        self.registers[Nibble::new::<15>()] = is_overflowed as u8;
      }

      Instruction::SubRegisters(x_reg_index, y_reg_index) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        let (value, is_overflowed) = reg_x.overflowing_sub(reg_y);
        self.registers[x_reg_index] = value;
        self.registers[Nibble::new::<15>()] = !is_overflowed as u8;
      }

      Instruction::RShiftRegisters(x_reg_index, y_reg_index) => {
        let reg_y = self.registers[y_reg_index];
        self.registers[x_reg_index] = reg_y >> 1;
        self.registers[Nibble::new::<15>()] = reg_y & 0x1;
      }

      Instruction::SubRegisterReversed(x_reg_index, y_reg_index) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        let (value, is_overflowed) = reg_y.overflowing_sub(reg_x);
        self.registers[x_reg_index] = value;
        self.registers[Nibble::new::<15>()] = !is_overflowed as u8;
      }

      Instruction::LShiftRegisters(x_reg_index, y_reg_index) => {
        let reg_y = self.registers[y_reg_index];
        self.registers[x_reg_index] = reg_y << 1;
        self.registers[Nibble::new::<15>()] = (reg_y & 0x80) >> 7;
      }

      Instruction::ClearScreen => self.platform.clear_screen(),

      Instruction::DrawSprite(x_reg_index, y_reg_index, rows_count) => {
        let reg_x = self.registers[x_reg_index];
        let reg_y = self.registers[y_reg_index];
        let mem_start = self.index_register;
        let mem_end = self.index_register + rows_count.as_u8() as i16;
        let sprite = Sprite::new(&self.memory[mem_start..mem_end]);

        let pos = Point {
          x: reg_x % SCREEN_WIDTH as u8,
          y: reg_y % SCREEN_HEIGHT as u8,
        };
        self.registers[Nibble::new::<15>()] = self.platform.draw_sprite(pos, sprite) as u8;
      }

      Instruction::SetDelayTimer(reg_index) => {
        self.platform.set_delay_timer(self.registers[reg_index]);
      }

      Instruction::SetSoundTimer(reg_index) => {
        self.platform.set_sound_timer(self.registers[reg_index]);
      }

      Instruction::WaitForKeyDown(reg_index) => {
        self.expecting_key = Some(reg_index);
        self.platform.get_last_pressed_key();
      }

      Instruction::Jump(address) => {
        self.instruction_address = address;
        return Ok(());
      }

      Instruction::Call(address) => {
        self
          .stack
          .push(self.instruction_address + ADDRESS_BYTE_STEP)?;
        self.instruction_address = address;
        return Ok(());
      }

      Instruction::Return => {
        self.instruction_address = self.stack.pop()?;
        return Ok(());
      }

      Instruction::JumpV0(address) => {
        self.instruction_address = address + self.registers[Nibble::new::<0>()] as i16;
        return Ok(());
      }
    }

    self.instruction_address += instruction_step * ADDRESS_BYTE_STEP;
    Ok(())
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Default)]
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

impl fmt::Display for OpCode {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl From<OpCode> for u16 {
  fn from(value: OpCode) -> Self {
    value.0
  }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Instruction {
  /* Условия */
  SkipIfEqual(Nibble, u8),
  SkipIfNotEqual(Nibble, u8),
  SkipIfRegistersEqual(Nibble, Nibble),
  SkipIfRegistersNotEqual(Nibble, Nibble),
  SkipIfKeyDown(Nibble),
  SkipIfKeyUp(Nibble),
  /* Запись регистров */
  SetRegister(Nibble, u8),
  CopyRegister(Nibble, Nibble),
  SetAddressRegister(Address),
  SetRegisterRandom(Nibble, u8),
  GetDelayTimer(Nibble),
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
  /* Таймеры */
  SetDelayTimer(Nibble),
  SetSoundTimer(Nibble),
  /* Клавиатура */
  WaitForKeyDown(Nibble),
  /* Работа с адресами */
  Jump(Address),
  JumpV0(Address),
  Call(Address),
  Return,
}

impl TryFrom<OpCode> for Instruction {
  type Error = InterpreterError;

  fn try_from(code: OpCode) -> Result<Self, Self::Error> {
    let code_nibbles = (
      code.get_nibble(3).as_u8(),
      code.get_nibble(2).as_u8(),
      code.get_nibble(1).as_u8(),
      code.get_nibble(0).as_u8(),
    );

    let instruction = match code_nibbles {
      /* Условия */
      (0x3, ..) => Instruction::SkipIfEqual(code.get_nibble(2), code.get_word(0)),
      (0x4, ..) => Instruction::SkipIfNotEqual(code.get_nibble(2), code.get_word(0)),
      (0x5, .., 0x0) => Instruction::SkipIfRegistersEqual(code.get_nibble(2), code.get_nibble(1)),
      (0x9, .., 0x0) => {
        Instruction::SkipIfRegistersNotEqual(code.get_nibble(2), code.get_nibble(1))
      }
      (0xE, _, 0x9, 0xE) => Instruction::SkipIfKeyDown(code.get_nibble(2)),
      (0xE, _, 0xA, 0x1) => Instruction::SkipIfKeyUp(code.get_nibble(2)),
      /* Запись регистров */
      (0x6, ..) => Instruction::SetRegister(code.get_nibble(2), code.get_word(0)),
      (0x8, .., 0x0) => Instruction::CopyRegister(code.get_nibble(2), code.get_nibble(1)),
      (0xA, ..) => Instruction::SetAddressRegister(code.get_address()),
      (0xC, ..) => Instruction::SetRegisterRandom(code.get_nibble(2), code.get_word(0)),
      (0xF, _, 0x0, 0x7) => Instruction::GetDelayTimer(code.get_nibble(2)),
      (0xF, _, 0x5, 0x5) => Instruction::WriteRegistersToMem(code.get_nibble(2)),
      (0xF, _, 0x6, 0x5) => Instruction::LoadRegistersFromMem(code.get_nibble(2)),
      (0xF, _, 0x3, 0x3) => Instruction::RegisterToBCD(code.get_nibble(2)),
      /* Математические операции */
      (0x7, ..) => Instruction::AddRegister(code.get_nibble(2), code.get_word(0)),
      (0x8, .., 0x1) => Instruction::OrRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, .., 0x2) => Instruction::AndRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, .., 0x3) => Instruction::XorRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0xF, _, 0x1, 0xE) => Instruction::AddAddressRegister(code.get_nibble(2)),
      (0x8, .., 0x4) => Instruction::AddRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, .., 0x5) => Instruction::SubRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, .., 0x6) => Instruction::RShiftRegisters(code.get_nibble(2), code.get_nibble(1)),
      (0x8, .., 0x7) => Instruction::SubRegisterReversed(code.get_nibble(2), code.get_nibble(1)),
      (0x8, .., 0xE) => Instruction::LShiftRegisters(code.get_nibble(2), code.get_nibble(1)),
      /* Экран */
      (0x0, 0x0, 0xE, 0x0) => Instruction::ClearScreen,
      (0xD, ..) => {
        Instruction::DrawSprite(code.get_nibble(2), code.get_nibble(1), code.get_nibble(0))
      }
      /* Таймеры */
      (0xF, _, 0x1, 0x5) => Instruction::SetDelayTimer(code.get_nibble(2)),
      (0xF, _, 0x1, 0x8) => Instruction::SetSoundTimer(code.get_nibble(2)),
      /* Клавиатура */
      (0xF, _, 0x0, 0xA) => Instruction::WaitForKeyDown(code.get_nibble(2)),
      /* Работа с адресами */
      (0x1, ..) => Instruction::Jump(code.get_address()),
      (0xB, ..) => Instruction::JumpV0(code.get_address()),
      (0x2, ..) => Instruction::Call(code.get_address()),
      (0x0, 0x0, 0xE, 0xE) => Instruction::Return,
      _ => return Err(InterpreterError::UnknownOpCode(code)),
    };
    Ok(instruction)
  }
}
