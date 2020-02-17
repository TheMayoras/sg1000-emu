use bus::{bus::*, ram::*, BusConnectable, MemoryMap, MutRef};
use piston::{Button, ButtonArgs, ButtonState, Key};
use std::fs::File;
use std::io::stdout;
use std::io::Read;
use std::path::PathBuf;
use std::{cell::RefCell, rc::Rc};
use tms9918::ppu::*;
use z80::cpu::*;

const DPAD_UP: u8 = 0;
const DPAD_DOWN: u8 = 1;
const DPAD_LEFT: u8 = 2;
const DPAD_RIGHT: u8 = 3;
const BUTTON1: u8 = 4;
const BUTTON2: u8 = 5;

#[allow(dead_code)]
pub struct Emulator {
    pub cpu: MutRef<Cpu>,
    pub ppu: MutRef<Ppu>,
    controller: MutRef<KeyboardController>,
    paused: bool,
}

impl Emulator {
    //     vec![
    //     0xFB, 0x1e, 0x31, 0x16, 0xF2, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19,
    //     0x09, 0xeb, 0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed,
    //     0x42, 0xcb, 0x3c, 0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30,
    //     0xe3, // 0xC3, 0x00, 0x00,
    // ]
    pub fn new(file: &PathBuf) -> Emulator {
        let mut zexall = File::open(file).expect("Could not find file");

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

        let controller = Rc::new(RefCell::new(KeyboardController::new()));
        let ppu = Rc::new(RefCell::new(Ppu::new()));
        let io_ports = Rc::new(RefCell::new(
            Bus::builder()
                .add_ref(&(Rc::clone(&ppu) as Rc<RefCell<dyn BusConnectable>>))
                .add_ref(&(Rc::clone(&controller) as Rc<RefCell<dyn BusConnectable>>))
                .build(),
        ));

        let cpu = Rc::new(RefCell::new(Cpu::with_pc(&data_bus, &io_ports, 0)));

        Emulator {
            cpu,
            ppu,
            controller,
            paused: false,
        }
    }

    pub fn refresh(&mut self) {
        // dont change the frame if we are paused
        if self.paused {
            return;
        }

        // loop until we hit vblank
        loop {
            let ticks = self.cpu.borrow_mut().do_operation();
            if self.ppu.borrow_mut().update(ticks) {
                self.cpu.borrow_mut().mask_interrupt = true && self.ppu.borrow().intrpt_enabled();

                // we reached the vblank, so get out of the loop
                break;
            }
        }

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

    pub fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::Space) => {
                if args.state == ButtonState::Press {
                    self.cpu.borrow_mut().nomask_interrupt = true;
                }
            }
            Button::Keyboard(Key::P) if args.state == ButtonState::Press => {
                self.paused = !self.paused
            }
            Button::Keyboard(Key::Backslash)
                if self.paused && args.state == ButtonState::Release =>
            {
                self.cpu.borrow().log(stdout()).unwrap();
                self.ppu.borrow().log(stdout()).unwrap();
                // self.ppu.log(stdout());
            }
            _ => {
                if !self.paused {
                    self.controller.borrow_mut().input(args);
                }
            }
        }
    }
}

struct KeyboardController {
    /// Note that the bit is low when the key is pressed
    joypad1: u8,
    joypad2: u8,
    dpad_up: Key,
    dpad_down: Key,
    dpad_left: Key,
    dpad_right: Key,
    button1: Key,
    button2: Key,
}

impl KeyboardController {
    pub fn new() -> KeyboardController {
        KeyboardController {
            joypad1: 0xFF,
            joypad2: 0xFF,
            dpad_up: Key::W,
            dpad_down: Key::S,
            dpad_left: Key::A,
            dpad_right: Key::D,
            button1: Key::J,
            button2: Key::K,
        }
    }

    pub fn input(&mut self, args: &ButtonArgs) {
        let is_pressed = args.state == ButtonState::Press;
        match args.button {
            Button::Keyboard(key) => self.change_button_state(key, is_pressed),
            _ => {}
        }
    }

    fn change_button_state(&mut self, key: Key, is_pressed: bool) {
        if key == self.dpad_up {
            self.set_dpad(DPAD_UP, is_pressed);
        } else if key == self.dpad_down {
            self.set_dpad(DPAD_DOWN, is_pressed);
        } else if key == self.dpad_left {
            self.set_dpad(DPAD_LEFT, is_pressed);
        } else if key == self.dpad_right {
            self.set_dpad(DPAD_RIGHT, is_pressed);
        } else if key == self.button1 {
            self.set_button(BUTTON1, is_pressed);
        } else if key == self.button2 {
            self.set_button(BUTTON2, is_pressed);
        }
    }

    fn set_dpad(&mut self, button: u8, is_pressed: bool) {
        // bit is set high if the button is not pressed
        if !is_pressed {
            // clear the other dpad buttons (only one pressed at a time)
            self.joypad1 = !(1 << button) | (self.joypad1 & 0b11_0000);
        } else {
            // clear the dbad buttons
            self.joypad1 &= 0b11_0000;
        }
    }

    fn set_button(&mut self, bit: u8, is_pressed: bool) {
        // bit is set high if the button is not pressed
        if !is_pressed {
            self.joypad1 |= 1 << bit;
        } else {
            self.joypad1 &= !(1 << bit);
        }
    }
}

impl BusConnectable for KeyboardController {
    fn accept(&self, addr: u16) -> bool {
        let val = addr & 0xFF;
        val == 0xDC || val == 0xC0 || val == 0xDD || val == 0xC1
    }

    fn cpu_read(&mut self, addr: u16) -> u8 {
        if addr & 0xFF == 0xDC || addr & 0xFF == 0xC0 {
            self.joypad1
        } else {
            self.joypad2
        }
    }

    fn cpu_write(&mut self, _addr: u16, _val: u8) -> bool {
        false
    }
}

struct MiscIo {}

impl BusConnectable for MiscIo {
    fn accept(&self, addr: u16) -> bool {
        (addr & 0xDE) == 0xDE
    }

    fn cpu_read(&mut self, addr: u16) -> u8 {
        0
    }

    fn cpu_write(&mut self, addr: u16, val: u8) -> bool {
        true
    }
}
