// A very simple Mapper with no esoteric features or bank switching.
// Reference capabilities: https://wiki.nesdev.com/w/index.php/NROM

use cart::NesHeader;
use mmc::mapper::*;

pub struct Nrom {
    pub prg_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mirroring: Mirroring,
}

impl Nrom {
    pub fn new(header: NesHeader, chr: &[u8], prg: &[u8]) -> Nrom {
        return Nrom {
            prg_rom: prg.to_vec(),
            prg_ram: Vec::new(),
            chr_rom: chr.to_vec(),
            mirroring: header.mirroring,
        }
    }
}

impl Mapper for Nrom {
    fn mirroring(&self) -> Mirroring {
        return self.mirroring;
    }
    
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000 ... 0x1FFF => return self.chr_rom[address as usize],
            0x6000 ... 0x7FFF => {
                let prg_ram_len = self.prg_ram.len();
                if prg_ram_len > 0 {
                    return self.prg_ram[((address - 0x6000) % (prg_ram_len as u16)) as usize];
                } else {
                    return 0;
                }
            },
            0x8000 ... 0xFFFF => {
                let prg_rom_len = self.prg_rom.len();
                return self.prg_rom[(address % (prg_rom_len as u16)) as usize];
            },
            _ => return 0
        }
    }

    fn write_byte(&mut self, address: u16, data: u8) {
        match address {
            0x6000 ... 0x7FFF => {
                let prg_ram_len = self.prg_ram.len();
                if prg_ram_len > 0 {
                    self.prg_ram[((address - 0x6000) % (prg_ram_len as u16)) as usize] = data;
                }
            },
            _ => {}
        }
    }
}
