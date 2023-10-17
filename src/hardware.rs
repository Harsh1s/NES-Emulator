// Constants for stack start address and stack reset value
// The reason the NES stack ends at 253 bytes (0x01FD) rather than 256 bytes (0x01FF) is due to a hardware limitation.
// The top three addresses (0x01FD, 0x01FE, and 0x01FF) are reserved for the NES's interrupt vector table.
const STACK_START: u16 = 0x0100;
const STACK_RESET: u8 = 0xfd;

// Define the CPU struct
pub struct CPU {
    pub accumulator: u8,      // Accumulator register
    pub index_x: u8,          // X index register
    pub index_y: u8,          // Y index register
    pub status: u8,           // Status register (flags)
    pub program_counter: u16, // Program counter
    pub stack_pointer: u8,    // Stack pointer
    memory: [u8; 0xFFFF],     // Memory array to store data and instructions
}

// Enum to represent addressing modes
#[derive(Debug)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    Absolute,
    ZeroPageX,
    ZeroPageY,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    NoneAddressing,
}



impl CPU {
    // Constructor to create a new CPU instance
    pub fn new() -> Self {
        CPU {
            accumulator: 0,
            index_x: 0,
            index_y: 0,
            status: 0b00100100, // Default status flags (interrupt disabled and unused)
            program_counter: 0,
            stack_pointer: STACK_RESET, // Initial stack pointer value
            memory: [0; 0xFFFF],        // Initialize memory with all zeros
        }
    }

    // Helper function to read from memory
    fn mem_read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    // Helper function to write to memory
    fn mem_write(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    // Helper function to read a 16-bit value from memory
    fn mem_read_u16(&self, address: u16) -> u16 {
        let byte_one = self.mem_read(address) as u16;
        let byte_two = self.mem_read(address + 1) as u16;
        (byte_two as u16) << 8 | (byte_one as u16)
    }

    // Helper function to write a 16-bit value to memory
    fn mem_write_u16(&mut self, address: u16, data: u16) {
        let byte_one = (data & 0xff) as u8;
        let byte_two = (data >> 8) as u8;
        self.mem_write(address, byte_one);
        self.mem_write(address + 1, byte_two);
    }

    // Helper function to calculate the operand address based on addressing mode
    fn address_operand(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPageX => {
                let offset = self.mem_read(self.program_counter);
                let address = offset.wrapping_add(self.index_x) as u16;
                address
            }
            AddressingMode::ZeroPageY => {
                let offset = self.mem_read(self.program_counter);
                let address = offset.wrapping_add(self.index_y) as u16;
                address
            }
            AddressingMode::AbsoluteX => {
                let base = self.mem_read_u16(self.program_counter);
                let address = base.wrapping_add(self.index_x as u16);
                address
            }
            AddressingMode::AbsoluteY => {
                let base = self.mem_read_u16(self.program_counter);
                let address = base.wrapping_add(self.index_y as u16);
                address
            }
            AddressingMode::IndirectX => {
                let base = self.mem_read(self.program_counter);
                let offset: u8 = (base as u8).wrapping_add(self.index_x);
                let byte_one = self.mem_read(offset as u16);
                let byte_two = self.mem_read(offset.wrapping_add(1) as u16);
                (byte_two as u16) << 8 | (byte_one as u16)
            }
            AddressingMode::IndirectY => {
                let base = self.mem_read(self.program_counter);
                let byte_one = self.mem_read(base as u16);
                let byte_two = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (byte_two as u16) << 8 | (byte_one as u16);
                let deref = deref_base.wrapping_add(self.index_y as u16);
                deref
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }
        }
    }

    // Load instructions into memory starting at address 0x8000
    pub fn load(&mut self, instructions: Vec<u8>) {
        self.memory[0x8000..(0x8000 + instructions.len())].copy_from_slice(&instructions[..]);
        self.mem_write_u16(0xFFFC, 0x8000); // Set the reset vector
    }

    // Load instructions into memory and interpret them
    pub fn load_and_interpret(&mut self, instructions: Vec<u8>) {
        self.load(instructions);
        self.reset(); // Initialize CPU state
        self.interpret(); // Start interpretation
    }

    // Reset the CPU to its initial state
    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.index_x = 0;
        self.index_y = 0;
        self.stack_pointer = STACK_RESET;
        self.status = 0b00100100;
        self.program_counter = self.mem_read_u16(0xFFFC); // Set program counter to reset vector
    }

    // Implement the LDA instruction
    fn lda(&mut self, mode: &AddressingMode) {
        let address = self.address_operand(&mode);
        let value = self.mem_read(address);
        self.accumulator = value;
    }

    // Implement the ADC intsruction
    fn adc(&mut self, mode : &AddressingMode) {
        let address = self.address_operand(&mode);
        let value = self.mem_read(address);

        let raw_sum : i32 = (self.accumulator + value).into();

        if  raw_sum > 255 {
            self.status = self.status | 0b00000001; // set_carry_flag
        }
        else {
            self.status = self.status & 0b11111110; // unset carry flag
        }

        if self.accumulator + value > 127 {
            self.status = self.status | 0b01000000; //set overflow flag
        }
        else {
            self.status = self.status & 0b10111111; // unset overflow flag
        }
        self.accumulator += value;
        self.update_flags(self.accumulator);

    }

    // Update CPU status flags
    fn update_flags(&mut self, to_check: u8) {
        if to_check == 0 {
            self.status = self.status | 0b00000010; // Set zero flag
        } else {
            self.status = self.status & 0b11111101; // Clear zero flag
        }

        if to_check & 0b10000000 == 0b10000000 {
            self.status = self.status | 0b10000000; // Set negative flag
        } else {
            self.status = self.status & 0b01111111; // Clear negative flag
        }
    }

    // Main interpreter loop
    pub fn interpret(&mut self) {
        self.program_counter = self.mem_read_u16(0xFFFC); // Set program counter to reset vector

        loop {
            let opcode = self.memory[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                0xa9 => {
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0xa5 => {
                    self.lda(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0xb5 => {
                    self.lda(&AddressingMode::ZeroPageX);
                    self.program_counter += 1;
                }
                0xad => {
                    self.lda(&AddressingMode::Absolute);
                    self.program_counter += 2;
                }
                0xbd => {
                    self.lda(&AddressingMode::AbsoluteX);
                    self.program_counter += 2;
                }
                0xb9 => {
                    self.lda(&AddressingMode::AbsoluteY);
                    self.program_counter += 2;
                }
                0xa1 => {
                    self.lda(&AddressingMode::IndirectX);
                    self.program_counter += 1;
                }
                0xb1 => {
                    self.lda(&AddressingMode::IndirectY);
                    self.program_counter += 1;
                }

                // ADC
                0x69 => {
                    self.adc(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0x65 => {
                    self.adc(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0x75 => {
                    self.adc(&AddressingMode::ZeroPageX);
                    self.program_counter += 1;
                }
                0x6d => {
                    self.adc(&AddressingMode::Absolute);
                    self.program_counter += 2;
                }
                0x7d => {
                    self.adc(&AddressingMode::AbsoluteX);
                    self.program_counter += 2;
                }
                0x79 => {
                    self.adc(&AddressingMode::AbsoluteY);
                    self.program_counter += 2;
                }
                0x61 => {
                    self.adc(&AddressingMode::IndirectX);
                    self.program_counter += 1;
                }
                0x71 => {
                    self.adc(&AddressingMode::IndirectY);
                    self.program_counter += 1;
                }
                0x00 => return, // Exit the interpreter loop

                _ => todo!("write more functions for opcodes"),
            }
        }
    }
}

// Unit test module
#[cfg(test)]
mod test {
    use super::*;

    // Test case for the LDA (Load Accumulator) instruction with immediate addressing
    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_interpret(vec![0xa9, 0x05, 0x00]); // Load LDA instruction with value 0x05
        assert_eq!(cpu.accumulator, 5); // Check if accumulator is loaded correctly
        assert!(cpu.status & 0b0000_0010 == 0b00); // Check if zero flag is not set
        assert!(cpu.status & 0b1000_0000 == 0); // Check if negative flag is not set
    }
}
