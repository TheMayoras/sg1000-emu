use crate::Canvas;
use im::*;
use std::{cell::RefCell, rc::Rc};
use bus::{MutRef, ram::Ram, bus::*};

pub const WIDTH: u32 = 256;
pub const HEIGHT: u32 = 192;
pub const VRAM_SIZE: usize = 16*1024; // 16 kbytes

#[allow(dead_code)]
#[rustfmt::skip]
pub struct Ppu {
    pub canvas:  Canvas,
    next_canvas: Canvas,
    ram:         MutRef<Ram>,
    io_bus:      MutRef<Bus>,
}

impl Ppu {
    #[rustfmt::skip]
    pub fn new(io_bus: &MutRef<Bus>) -> Ppu {    
        Ppu {
            canvas:      ImageBuffer::new(WIDTH, HEIGHT),
            next_canvas: ImageBuffer::new(WIDTH, HEIGHT),
            ram:         Rc::new(RefCell::new(Ram::builder().size(VRAM_SIZE).build())),
            io_bus:      Rc::clone(io_bus),
        }
    }
}
