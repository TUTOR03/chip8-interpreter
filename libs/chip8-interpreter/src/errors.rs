use super::{address::Address, nibble::Nibble};
use crate::interpreter::OpCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
  #[error("Unknown opcode: {0}")]
  UnknownOpCode(OpCode),
  #[error("Max nibble size exceeded. Should be less or equal {}", Nibble::MAX)]
  NibbleOverflow,
  #[error("Max address size exceeded. Should be less or equal {}", Address::MAX)]
  AddressOverflow,
  #[error("Stack overflow")]
  StackOverflow,
  #[error("Stack underflow")]
  StackUnderflow,
}
