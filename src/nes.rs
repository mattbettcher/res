#![allow(dead_code)]

use cpu::P65;
use mem::MemMap;
use mmc::mapper::Mapper;

pub struct Nes {
    pub cpu: P65,
    pub mem: MemMap,
}

impl Nes {
    pub fn new(m: Box<Mapper>) -> Self {
        Nes {
            cpu: P65::new(),
            mem: MemMap::new(m),
        }
    }
}

pub fn step(nes: &mut Nes) {
    nes.cpu.step(&mut nes.mem, 1);
    let nmi = nes.mem.ppu.step(&mut *nes.mem.mapper, 3);

    if nmi {
        nes.cpu.nmi_set();
    }
}

pub fn reset(nes: &mut Nes) {
    nes.cpu.reset(&mut nes.mem);
    // todo reset other stuff!
}

