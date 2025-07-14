#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode { Implied, Immediate, ZeroPage, Absolute, Relative }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpcodeInfo { pub opcode: u8, pub mnemonic: &'static str, pub mode: AddressingMode, pub cycles: u8 }

pub fn decode_opcode(opcode: u8) -> OpcodeInfo {
    match opcode {
        0x00 => OpcodeInfo { opcode, mnemonic: "BRK", mode: AddressingMode::Implied, cycles: 7 },
        0xEA => OpcodeInfo { opcode, mnemonic: "NOP", mode: AddressingMode::Implied, cycles: 2 },
        0xA9 => OpcodeInfo { opcode, mnemonic: "LDA", mode: AddressingMode::Immediate, cycles: 2 },
        0xA5 => OpcodeInfo { opcode, mnemonic: "LDA", mode: AddressingMode::ZeroPage, cycles: 3 },
        0xAD => OpcodeInfo { opcode, mnemonic: "LDA", mode: AddressingMode::Absolute, cycles: 4 },
        0x4C => OpcodeInfo { opcode, mnemonic: "JMP", mode: AddressingMode::Absolute, cycles: 3 },
        0xD0 => OpcodeInfo { opcode, mnemonic: "BNE", mode: AddressingMode::Relative, cycles: 2 },
        _ => OpcodeInfo { opcode, mnemonic: "???", mode: AddressingMode::Implied, cycles: 0 },
    }
}

