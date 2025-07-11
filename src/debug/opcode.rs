#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode { Implied, Immediate, ZeroPage, Absolute, Relative }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpcodeInfo { pub opcode: u8, pub mnemonic: &'static str, pub mode: AddressingMode, pub cycles: u8 }

