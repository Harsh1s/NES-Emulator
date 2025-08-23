#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirroring { Horizontal, Vertical, FourScreen }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeaderSummary { pub prg_rom_banks: u8, pub chr_rom_banks: u8, pub mapper: u8, pub mirroring: Mirroring }

pub fn parse_ines_header(header: &[u8; 16]) -> Option<HeaderSummary> {
    if &header[0..4] != b"NES\x1A" { return None; }
    let flags6 = header[6];
    let flags7 = header[7];
    let mirroring = if flags6 & 0x08 != 0 { Mirroring::FourScreen } else if flags6 & 0x01 != 0 { Mirroring::Vertical } else { Mirroring::Horizontal };
    Some(HeaderSummary { prg_rom_banks: header[4], chr_rom_banks: header[5], mapper: (flags6 >> 4) | (flags7 & 0xF0), mirroring })
}
