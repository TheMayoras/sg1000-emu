use bus::{bus::*, MutRef};
use std::{cell::RefCell, rc::Rc};
use tms9918::ppu::*;
use z80::cpu::*;

#[allow(dead_code)]
pub struct Emulator {
    pub cpu: MutRef<Cpu>,
    pub ppu: MutRef<Ppu>,
}

impl Emulator {
    pub fn new() -> Emulator {
        let data_bus: MutRef<Bus> = Rc::new(RefCell::new(
            vec![
                0xFB, 0x1e, 0x31, 0x16, 0xF2, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19,
                0x09, 0xeb, 0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed,
                0x42, 0xcb, 0x3c, 0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30,
                0xe3, // 0xC3, 0x00, 0x00,
            ]
            .into(),
        ));

        let io_ports = Rc::new(RefCell::new(Bus::builder().build()));

        let cpu = Rc::new(RefCell::new(Cpu::new(&data_bus, &io_ports)));
        let ppu = Rc::new(RefCell::new(Ppu::new(&io_ports, &cpu)));

        Emulator { cpu, ppu }
    }

    pub fn refresh(&mut self) {
        (0..10).for_each(|_| {
            self.cpu.borrow_mut().do_operation();
        });
    }

    pub fn halt_cpu(&mut self) {
        self.cpu.borrow_mut().halt();
    }

    pub fn reset_halt_cpu(&mut self) {
        self.cpu.borrow_mut().reset_halt();
    }

    pub fn flip_halt_cpu(&mut self) {
        if self.cpu.borrow().is_halted() {
            self.reset_halt_cpu();
        } else {
            self.halt_cpu();
        }
    }

    pub fn cpu(&self) -> &RefCell<Cpu> {
        self.cpu.as_ref()
    }
}
