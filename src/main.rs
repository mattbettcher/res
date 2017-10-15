#[macro_use]
extern crate bitfield;
extern crate minifb;
extern crate rand;

mod cpu;
mod disasm;
mod nes;
mod mem;
mod ppu;
mod mmc;
mod cart;

use minifb::{Key, WindowOptions, Window, Scale};
use nes::*;
use std::error::Error;
use std::fs::File;
use std::io::Read;

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

fn load(nes: &mut Nes, file_path: &str) {
    let mut file = match File::open(file_path) {
      Err(why) => panic!("Couldn't open {}: {}", file_path, why.description()),
      Ok(file) => file,
    };
    // Read the whole thing
    let mut cartridge = Vec::new();
    match file.read_to_end(&mut cartridge) {
      Err(why) => panic!("Couldn't read data: {}", why.description()),
      Ok(bytes_read) => {
        println!("Data read successfully: {}", bytes_read);

        let nes_header = cart::extract_header(&cartridge);
        cart::print_header_info(nes_header);
        let mapper = cart::load_from_cartridge(nes_header, &cartridge);
        *nes = Nes::new(mapper);
      }
    }
}

fn main() {
    let file_path = "Donkey Kong (GC).nes";
    let mut nes = Nes::new(Box::new(mmc::none::NoneMapper::new()));
    
    load(&mut nes, file_path);
    reset(&mut nes);

    //stdout().flush();

    // window test stuff
    //let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let options = WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()};

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for _i in 0..100 {
            step(&mut nes);
            println!("{}", cpu::status_string(&nes.cpu, &mut nes.mem));
        }
        
        //for i in buffer.iter_mut() {
        //    *i = rand::random::<u32>(); // write something more funny here!
       // }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        //window.update_with_buffer(&buffer).unwrap();
        window.update();
    }
}
