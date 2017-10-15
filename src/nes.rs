#![allow(dead_code)]

use cpu::P65;
use mem::Mem;
use ppu::PPU;

pub struct Nes {
    pub cpu: P65,
    pub mem: Mem,
    pub ppu: PPU,
}

impl Nes {
    pub fn new() -> Self {
        Nes {
            cpu: P65::new(),
            mem: Mem::new(),
            ppu: PPU::new(),
        }
    }
}

pub fn step(nes: &mut Nes) {
    nes.cpu.step(&mut nes.mem, 1);
    //nes.ppu.step(&mut nes.cart, 3);
}

