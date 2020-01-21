use crate::Canvas;
use im::*;
use std::{cell::RefCell, rc::Rc};
use bus::{MutRef, ram::Ram, bus::*};
use z80::cpu::Cpu;

pub const WIDTH: u32 = 256;
pub const HEIGHT: u32 = 192;
pub const VRAM_SIZE: usize = 16*1024; // 16 kbytes

pub static COLORS: [Rgba<u8>; 16] = [
    Rgba([0,0,0,0]), // transparent
    Rgba([0,0,0,0xFF]), // black
    Rgba([0x20, 0xC0, 0x20, 0xFF]), // green
    Rgba([0x60, 0xE0, 0x60, 0xFF]), // bright green   
    Rgba([0x20, 0x20, 0xE0, 0xFF]), // blue           
    Rgba([0x40, 0x60, 0xE0, 0xFF]), // bright blue    
    Rgba([0xA0, 0x20, 0x20, 0xFF]), // dark red       
    Rgba([0x40, 0xC0, 0xE0, 0xFF]), // cyan (?)       
    Rgba([0xE0, 0x20, 0x20, 0xFF]), // red            
    Rgba([0xE0, 0x60, 0x60, 0xFF]), // bright red     
    Rgba([0xC0, 0xC0, 0x20, 0xFF]), // yellow         
    Rgba([0xC0, 0xC0, 0x80, 0xFF]), // bright yellow  
    Rgba([0x20, 0x80, 0x20, 0xFF]), // dark green     
    Rgba([0xC0, 0x40, 0xA0, 0xFF]), // pink           
    Rgba([0xA0, 0xA0, 0xA0, 0xFF]), // gray           
    Rgba([0xE0, 0xE0, 0xE0, 0xFF]), // white          
];

#[allow(dead_code)]
fn is_bit_set(val: u32, bit: u8) -> bool {
    (val >> bit) & 1 > 0
}

#[allow(dead_code)]
#[rustfmt::skip]
pub struct Ppu {
    pub canvas:   Canvas,
    next_canvas:  Canvas,
    ram:          MutRef<Ram>,
    io_bus:       MutRef<Bus>,
    cpu:          MutRef<Cpu>,
    status_reg:   u8,
    registers:    [u8; 8],
    line:         u16,
    max_lines:    u16,
    clock_cycles: u64,
}

impl Ppu {
    #[rustfmt::skip]
    pub fn new(io_bus: &MutRef<Bus>, cpu: &MutRef<Cpu>) -> Ppu {    
        Ppu {
            canvas:       ImageBuffer::new(WIDTH, HEIGHT),
            next_canvas:  ImageBuffer::new(WIDTH, HEIGHT),
            ram:          Rc::new(RefCell::new(Ram::builder().size(VRAM_SIZE).build())),
            io_bus:       Rc::clone(io_bus),
            cpu:          Rc::clone(cpu),
            status_reg:   0,
            registers:    [0; 8],
            line:         0,
            max_lines:    254,
            clock_cycles: 0,
        }
    }
}
