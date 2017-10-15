#![allow(dead_code)]

//use bitfield::{Bit, BitRange};


// nt = name table
// bg = background
// pt = pattern table
// spr = sprite

// breaking down sprite attributes using bitfield
bitfield! {
    #[derive(Copy, Clone)]
    pub struct SpriteAttributes(u8);
    impl Debug;
    #[inline]
    pub palette, set_palette: 1, 0;
    #[inline]
    pub bg_priority, set_bg_priority: 5;
    #[inline]
    pub hori_flip, set_hori_flip: 6;
    #[inline]
    pub vert_flip, set_vert_flip: 7;
    
    #[inline]
    pub get, set: 7, 0;
    unused, _ : 4, 2;    
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct CtrlReg1(u8);
    impl Debug;
    #[inline]
    pub nt_scroll_addr, set_nt_scroll_addr: 1, 0;
    #[inline]
    pub vram_addr_inc, set_vram_addr_inc: 2;
    #[inline]
    pub pt_addr_8x8, set_pt_addr_8x8: 3;
    #[inline]
    pub pt_addr_bg, set_pt_addr_bg: 4;
    #[inline]
    pub spr_size, set_spr_size: 5;
    #[inline]
    pub nmi_on_vblank, set_nmi_on_vblank: 7;
    #[inline]

    #[inline]
    pub get, set: 7, 0;
    unused, _: 6;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct CtrlReg2(u8);
    impl Debug;
    #[inline]
    pub mono_mode, set_mono_mode: 0;
    #[inline]
    pub bg_clip, set_bg_clip: 1;
    #[inline]
    pub spr_clip, set_spr_clip: 2;
    #[inline]
    pub bg_vis, set_bg_vis: 3;
    #[inline]
    pub spr_vis, set_spr_vis: 4;
    #[inline]
    pub color_emphasis, set_color_emphasis: 7, 5;

    #[inline]
    pub get, set: 7, 0;
}

bitfield! {
    #[derive(Copy, Clone)]
    pub struct StatusReg(u8);
    impl Debug;
    #[inline]
    pub lost_spr, set_lost_spr: 5;
    #[inline]
    pub spr_0_hit, set_spr_0_hit: 6;
    #[inline]
    pub vblank_flag, set_vblank_flag: 7;

    #[inline]
    pub get, set: 7, 0;
    unused, _: 4, 0;
}

// broke down the sprites for easier access
#[derive(Copy, Clone)]
pub struct Spr {
    // vertical position - 1
    pub y: u8,
    // 8x8 mode: bit 7-0 8 bit tile number   8x16 mode: bit 0 = pat table, bit 7-1 tile number
    pub tile: u8,
    // todo - maybe break this down with a c style enum?
    pub attrib: SpriteAttributes,     
    // horizontal position
    pub x: u8
}

pub struct PPU {
    pub vram: [u8; 0x1000],     // 4k of vram, nes only has 2k actual, other 2k is on some carts
    pub oam: [Spr; 64],         // 64 sprites - this is high level representation - need to ensure each byte is in the correct place
    pub palette: [u8; 0x20],    // 25 palette entries todo- maybe break this down????

    oam_sort_index: [usize; 8], // 8 sprites for the current scanline (index into oam)

    ctrl_reg_1: CtrlReg1,
    ctrl_reg_2: CtrlReg2,
    status: StatusReg,
    spr_addr: u8,

    // port 2005
    hori_scroll_origin: u8,
    vert_scroll_origin: u8,
    // port 2006
    vram_addr: u16,           // this is actually only 14 bits

    io_flipflop: bool,        // write flipflop for ports 2005 & 2006 - 
}

impl PPU {
    pub fn new() -> Self {
        PPU {
            vram: [0; 0x1000],      // vram should probably be cleared to random values, not 0
            oam: [Spr{x:0, y:0, tile: 0, attrib: SpriteAttributes(0)}; 64],
            palette: [0; 0x20],
            oam_sort_index: [0; 8],
            ctrl_reg_1: CtrlReg1(0),    // need to set the starting bits!
            ctrl_reg_2: CtrlReg2(0),
            status: StatusReg(0),
            spr_addr: 0,
            hori_scroll_origin: 0,
            vert_scroll_origin: 0,
            vram_addr: 0,
            io_flipflop: false
        }
    }

    pub fn step() {

    }

    // i/o ports for ppu (mapped to CPU memory 0x2000-0x2007 and mirrored through 0x3fff)
    // mirroring is handled in the cpu decoder in nes.rs
    pub fn read_reg(&mut self, address: usize) -> u8 {

        // todo - this is not done. many of these registers return different under different conditions, need to emulate it here!
        match address {
            0x2000 => self.ctrl_reg_1.get(),
            0x2001 => self.ctrl_reg_2.get(),
            // need to reset 1st/2nd-write flip-flop (used by port 2005 & 2006)
            // reading also resets vblank_flag
            0x2002 => self.status.get(),    
            0x2003 => self.spr_addr,
            0x2004 => { 
                // to maintain highlevel sprite design, this is way over complicated
                let temp_spr = &self.oam[(self.spr_addr / 4) as usize]; // get the sprite we are writing to
                match self.spr_addr % 4 {
                    0 => temp_spr.y,
                    1 => temp_spr.tile,
                    2 => temp_spr.attrib.get(),
                    3 => temp_spr.x,
                    _ => panic!("This should be impossible!")
                }},
            //0x2005 => ,   // can't read 2005????
            //0x2006 => ,   // can't read 2006????
            0x2007 => {
                let addr = self.vram[self.vram_addr as usize];
                if self.ctrl_reg_1.vram_addr_inc() {
                    self.vram_addr += 32;
                } else {
                    self.vram_addr += 1;                    
                }
                addr
            }
            _ => panic!()
        }
    }

    pub fn write_reg(&mut self, address: usize, data: u8) {
         // todo - this is not done. many of these registers can be written to multiple times, need to emulate it here!
        match address {
            0x2000 => self.ctrl_reg_1.set(data),
            0x2001 => self.ctrl_reg_2.set(data),
            0x2002 => self.status.set(data),
            0x2003 => self.spr_addr = data,
            0x2004 => { 
                // to maintain highlevel sprite design, this is way over complicated
                let addr = (self.spr_addr / 4) as usize;
                match self.spr_addr % 4 {
                    0 => self.oam[addr].y = data,
                    1 => self.oam[addr].tile = data,
                    2 => self.oam[addr].attrib.set(data),
                    3 => self.oam[addr].x = data,
                    _ => panic!("This should be impossible!")
                }
                self.spr_addr += 1;     // when we write to oam we must inc the address after
                },
            0x2005 => {
                if !self.io_flipflop {
                    // first write
                    self.hori_scroll_origin = data;
                    self.io_flipflop = true;
                } else {
                    // second write
                    self.vert_scroll_origin = data;
                    self.io_flipflop = false;
                }
            },
            0x2006 => {
                if !self.io_flipflop {
                    // first write
                    self.vram_addr &= 0xff;     // clear top 8 bits
                    self.vram_addr |= ((data & 0x3f) as u16) << 8;  // write top 6 bits
                    self.io_flipflop = true;
                } else {
                    // second write
                    self.vram_addr &= 0xff00;     // clear bottom 8 bits
                    self.vram_addr |= data as u16;
                    self.io_flipflop = false;
                }
            },
            0x2007 => {
                self.vram[self.vram_addr as usize] = data;
                if self.ctrl_reg_1.vram_addr_inc() {
                    self.vram_addr += 32;
                } else {
                    self.vram_addr += 1;                    
                }
            }
            _ => panic!()
        }
    }
}

