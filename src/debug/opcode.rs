#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode { Implied, Immediate, ZeroPage, Absolute, Relative }

