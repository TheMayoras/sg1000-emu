use bus::{bus::*, ram::*, BusConnectable, MemoryMap, MutRef};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::{cell::RefCell, rc::Rc};
use tms9918::ppu::*;
use z80::cpu::*;

#[allow(dead_code)]
pub struct Emulator {
    pub cpu: MutRef<Cpu>,
    pub ppu: MutRef<Ppu>,
}

impl Emulator {
    //     vec![
    //     0xFB, 0x1e, 0x31, 0x16, 0xF2, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19,
    //     0x09, 0xeb, 0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed,
    //     0x42, 0xcb, 0x3c, 0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30,
    //     0xe3, // 0xC3, 0x00, 0x00,
    // ]
    pub fn new() -> Emulator {
        let mut zexall = File::open(
            "D:\\Software\\SoftwareLanguages\\Rust\\sg-1000-emu\\resources\\hello_world2.sg",
        )
        .unwrap();
        let mut data = Vec::with_capacity(0xFFFF + 1);
        // zexall.seek(SeekFrom::Start(0x69)).unwrap();
        zexall.read_to_end(&mut data).unwrap();
        let data_bus: MutRef<Bus> = Rc::new(RefCell::new(
            Bus::builder()
                .add(
                    Ram::builder()
                        .data(data)
                        .map(MemoryMap::from(0..0x8000))
                        .build(),
                )
                .add(
                    Ram::builder()
                        .map(MemoryMap::from(0xA000..0xC000))
                        .mirror(MemoryMap::from(0xC000..=0xFFFF))
                        .build(),
                )
                .build(),
        ));

        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let io_ports = Rc::new(RefCell::new(
            Bus::builder()
                .add_ref(&(Rc::clone(&ppu) as Rc<RefCell<dyn BusConnectable>>))
                .build(),
        ));

        let cpu = Rc::new(RefCell::new(Cpu::with_pc(&data_bus, &io_ports, 0)));

        Emulator { cpu, ppu }
    }

    pub fn refresh(&mut self) {
        (0..500).for_each(|_| {
            let ticks = self.cpu.borrow_mut().do_operation();
            if self.ppu.borrow_mut().update(ticks) {
                self.cpu.borrow_mut().mask_interrupt = true;
                println!(
                    "
                =====================SETTING CPU INTERRUPT=====================
                "
                )
            }
        });
        println!(
            "
            
            ================                  ================
            ================ REFRESH COMPLETE ================
            ================                  ================
            
            "
        );
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
