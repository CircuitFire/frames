use crate::prelude::*;

pub fn print_pixel(pix: Pixel) {
    match pix {
        Pixel::Opaque(x) => {
            print!("[{}]", x.character)
        }
        Pixel::Clear => {
            print!("[ ]")
        }
    }
}

pub fn print_buffer(buf: &ScreenBuf) {
    let size = buf.size();
    for y in 0..size.y {
        for x in 0..size.x {
            print_pixel(buf.buffer.get(Coord{x: x, y: y}))
        }
        println!("")
    }
}