#![allow(dead_code)]

use ppu::PPU;
use mmc::mapper::*;
use cpu::Bus;

pub struct MemMap {
    pub ram: [u8; 0x0800],
    pub mapper: Box<Mapper>,
    pub ppu: PPU,
}

impl MemMap {
    pub fn new(m: Box<Mapper>) -> Self {
        MemMap {
            ram: [0; 0x0800],
            ppu: PPU::new(),
            mapper: m,
        }
    }
}

// memory bus for cpu -> ram & cart & i/o ports
impl <'a> Bus for MemMap {
    fn read(&mut self, a: usize) -> u8 {
        match a {
            0x0000 ... 0x1fff => self.ram[a % 0x0800],   // work ram mirrored
            0x4018 ... 0xffff => self.mapper.read_byte(a as u16),     // prg rom
            // ports
            0x2000 ... 0x3fff => self.ppu.read_reg(a % 8),   // ppu registers
            //0x4000 ... 0x4013 => apu.read_reg(a),       // apu registers
            0x4016 ... 0x4017 => 0,                         // joypad i/o NOTE: not implented yet!!!!

            0x4014 => unimplemented!(),                 // spr-ram dma register
            _ => 0      // not implemented reeturn 0
        }
    }

    fn write(&mut self, a: usize, v: u8) { 
        match a {
            0x0000 ... 0x1fff => self.ram[a % 0x0800] = v,   // work ram mirrored
            //0x4018 ... 0xffff => debug!("ROM Write @ 0x{:x} of 0x{:x}", a, v),
            // ports
            0x2000 ... 0x3fff => self.ppu.write_reg(a % 8, v),
            _ => println!("Invalid Write")
        }
    }
}
/*
// memory bus for PPU -> cart
impl Bus for PPU {
    fn read(&mut self, a: usize) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, a: usize, v: u8) {
        unimplemented!()
    }
}


// memory bus to the cartridge
impl Bus for Cart {
    fn read(&mut self, a: usize) -> u8 {
        unimplemented!()
    }

    fn write(&mut self, a: usize, v: u8) {
        unimplemented!()
    }
}*/