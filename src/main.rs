extern crate minifb;
extern crate rand;

use minifb::{Key, WindowOptions, Window, Scale};

const WIDTH: usize = 256;
const HEIGHT: usize = 240;

fn main() {
    // window test stuff
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let options = WindowOptions { resize: false, scale: Scale::X2, ..WindowOptions::default()};

    let mut window = Window::new("Test - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = rand::random::<u32>(); // write something more funny here!
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
