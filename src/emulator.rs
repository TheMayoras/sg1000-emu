use bus::{MutRef, bus::*};
use std::{boxed::Box, cell::RefCell, rc::Rc};
use tms9918::ppu::*;
use z80::cpu::*;

#[allow(dead_code)]
pub struct Emulator {
    pub cpu: Cpu,
    pub ppu: Ppu,
}

impl Emulator {
    pub fn new() -> Emulator {
        let data_bus: MutRef<Bus> = Rc::new(RefCell::new(vec![
            0x1e, 0x31, 0x16, 0xF2, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19, 0x09,
            0xeb, 0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed, 0x42,
            0xcb, 0x3c, 0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30, 0xe3,
        ]
        .into()));

        let io_ports = Rc::new(RefCell::new(Bus::builder().build()));

        let cpu = Cpu::new(&data_bus, &io_ports);
        let ppu = Ppu::new(&io_ports);

        Emulator {
            cpu,
            ppu
        }
    }

    pub fn refresh(&mut self) {
        
    }
}
