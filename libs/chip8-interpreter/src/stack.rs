use super::{address::Address, errors::InterpreterError};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct Stack<T, const S: usize> {
  items: [T; S],
  index: usize,
  is_empty: bool,
}

impl<const S: usize> Stack<Address, S> {
  pub fn new() -> Self {
    Self {
      items: [Address::default(); S],
      index: 0,
      is_empty: true,
    }
  }

  pub fn push(&mut self, value: Address) -> Result<(), InterpreterError> {
    if self.index == S - 1 {
      return Err(InterpreterError::StackOverflow);
    }

    self.index += 1;
    self.is_empty &= false;
    self.items[self.index] = value;
    Ok(())
  }

  pub fn pop(&mut self) -> Result<Address, InterpreterError> {
    if self.is_empty {
      return Err(InterpreterError::StackUnderflow);
    }

    let value = self.items[self.index];
    self.items[self.index] = Address::default();
    if self.index != 0 {
      self.index -= 1;
    } else {
      self.is_empty = true;
    }

    Ok(value)
  }
}
