#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirroring { Horizontal, Vertical, FourScreen }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderSummary { pub prg_rom_banks: u8, pub chr_rom_banks: u8, pub mapper: u8, pub mirroring: Mirroring }

