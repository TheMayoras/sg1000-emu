use crate::Canvas;
use bus::{bus::*, ram::Ram, BusConnectable, MutRef};
use graphics1::Graphics1Renderer;
use graphics2::Graphics2Renderer;
use im::*;
use std::mem;
use std::{cell::RefCell, rc::Rc};
use textmode::TextModeRenderer;

mod graphics1;
mod graphics2;
mod sprites;
mod textmode;

type Color = Rgba<u8>;

pub const WIDTH: u32 = 256;
pub const HEIGHT: u32 = 192;
pub const VRAM_SIZE: usize = 0x4001; // 16 kbytes

const CYCLES_PER_LINE: u64 = 228;

const MAX_LINES: u16 = 262;

pub static COLORS: [Rgba<u8>; 16] = [
    Rgba([0, 0, 0, 0]),             // transparent
    Rgba([0, 0, 0, 0xFF]),          // black
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
#[inline]
fn is_bit_set(val: u32, bit: u8) -> bool {
    (val >> bit) & 1 > 0
}

enum RWState {
    None,
    First(u8),
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum GrahpicsMode {
    Graphics1,
    Graphics2,
    Multicolor,
    Text,
}

#[allow(dead_code)]
#[rustfmt::skip]
pub struct Ppu {
    canvas:   Option<Canvas>,
    next_canvas:  Canvas,
    ram:          MutRef<Ram>,
    status_reg:   u8,
    registers:    [u8; 8],
    line:         u16,
    max_lines:    u16,
    clock_cycles: u64,
    rw_state:     RWState,
    cpu_addr:     u16,
    image_zoom:   u8
}

impl Ppu {
    #[rustfmt::skip]
    pub fn new() -> Ppu {
        let mut ppu = Ppu {
            canvas:       Some(ImageBuffer::new(4*WIDTH, 4*HEIGHT)),
            next_canvas:  ImageBuffer::new(4*WIDTH, 4*HEIGHT),
            ram:          Rc::new(RefCell::new(Ram::builder().size(VRAM_SIZE).build())),
            status_reg:   0,
            registers:    [0; 8],
            line:         0,
            max_lines:    MAX_LINES,
            clock_cycles: 0,
            rw_state:     RWState::None,
            cpu_addr:     0,
            image_zoom:   2,
        };
        ppu.registers[1] = 0b1_0000;
        ppu.registers[7] = 0xE1;

        ppu
    }

    /// Gets the current canvas.
    /// # Returns
    /// None if there was no change in the canvas
    /// Some with the new canvas
    ///     Note that calling this will consume the canvas
    pub fn get_canvas(&mut self) -> Option<Canvas> {
        self.canvas.take()
    }

    fn ram_read(&mut self) -> u8 {
        let val = self.ram.borrow_mut().cpu_read(self.cpu_addr);
        self.cpu_addr += 1;
        val
    }

    // get the status register
    // this resets the top two bits of the register and resets the read_write state
    fn get_status_reg(&mut self) -> u8 {
        let output = self.status_reg;
        self.status_reg &= !(0b11 << 7);
        self.rw_state = RWState::None;

        output
    }

    fn text_color(&self) -> Color {
        COLORS[(self.registers[7] >> 4) as usize]
    }

    fn text_back_color(&self) -> Color {
        COLORS[(self.registers[7] & 0x0F) as usize]
    }

    /// Returns the sprite size in the form
    /// (size, zoom)
    fn get_sprite_size(&self) -> (u8, u8) {
        (
            if is_bit_set(self.registers[1].into(), 1) {
                16
            } else {
                8
            },
            if is_bit_set(self.registers[1].into(), 0) {
                2
            } else {
                1
            },
        )
    }

    fn is_blank_screen(&self) -> bool {
        !is_bit_set(self.registers[1].into(), 6)
    }

    pub fn intrpt_enabled(&self) -> bool {
        is_bit_set(self.registers[1].into(), 5)
    }

    fn graphics_mode(&self) -> GrahpicsMode {
        use GrahpicsMode::*;
        let m3 = is_bit_set(self.registers[0].into(), 1);
        let m2 = is_bit_set(self.registers[1].into(), 3);
        let m1 = is_bit_set(self.registers[1].into(), 4);

        if !m1 && !m2 && !m3 {
            Graphics1
        } else if !m1 && !m2 && m3 {
            Graphics2
        } else if !m1 && m2 && !m3 {
            Multicolor
        } else if m1 && !m2 && !m3 {
            Text
        } else {
            //eprintln!("Illegal Graphics mode! M1={}, M2={}, M3={}", m1, m2, m3);
            Text
        }
    }

    fn name_table(&self) -> u16 {
        self.registers[2] as u16 * 0x400
    }

    fn color_table(&self) -> u16 {
        self.registers[3] as u16 * 0x40
    }

    fn pattern_gen_table(&self) -> u16 {
        self.registers[4] as u16 * 0x800
    }

    fn sprite_attr_table(&self) -> u16 {
        self.registers[5] as u16 * 0x80
    }

    fn sprite_patt_gen_table(&self) -> u16 {
        self.registers[6] as u16 * 0x800
    }
}

// Graphics Modes
impl Ppu {
    pub fn update(&mut self, cycles: u64) -> bool {
        self.clock_cycles += cycles;
        let mut vblank = false;

        if self.clock_cycles >= CYCLES_PER_LINE {
            if self.line < HEIGHT as u16 && !self.is_blank_screen() {
                self.scan_line();
            }
            self.clock_cycles -= CYCLES_PER_LINE;
            self.line += 1;

            vblank = self.line as u32 == HEIGHT;

            self.line %= self.max_lines;
        }

        if vblank {
            let mut swapped_canvas = ImageBuffer::new(
                WIDTH * self.image_zoom as u32,
                HEIGHT * self.image_zoom as u32,
            );
            mem::swap(&mut swapped_canvas, &mut self.next_canvas);
            self.canvas = Some(swapped_canvas);

            // set the interrupt flag
            self.status_reg |= 1 << 7;
        }

        return vblank;
    }

    fn scan_line(&mut self) {
        use GrahpicsMode::*;

        if self.line as u32 >= HEIGHT {
            return;
        }

        match self.graphics_mode() {
            Text => TextModeRenderer::new(self, self.image_zoom.into(), self.line).draw(),
            Graphics1 => Graphics1Renderer::new(self, self.image_zoom.into(), self.line).draw(),
            Graphics2 => Graphics2Renderer::new(self, self.image_zoom.into(), self.line).draw(),
            _ => unimplemented!("Graphics mode: {:?}", self.graphics_mode()),
        };
    }

    fn set_coincidence_flag(&mut self) {
        self.status_reg |= 1 << 6;
    }

    fn set_5th_sprite(&mut self, num: u8) {
        self.status_reg |= num & 0b1_1111;
    }
}

impl BusConnectable for Ppu {
    fn accept(&self, addr: u16) -> bool {
        (addr & 0xBE) == 0xBE || (addr & 0xBF) == 0xBF
    }

    fn cpu_read(&mut self, addr: u16) -> u8 {
        //println!("Reading address {:x} from PPU", addr);
        match addr & 0xFF {
            0xBF => self.get_status_reg(),
            0xBE => self.ram_read(),
            _ => panic!("Attempting to read data from PPU at IO address {}", addr),
        }
    }

    fn cpu_write(&mut self, addr: u16, val: u8) -> bool {
        // println!(
        //     "Writing val 0x{:x} (0b{:04b} {:04b}) to address 0x{:x} in PPU",
        //     val,
        //     (val >> 4) & 0b1111,
        //     val & 0b1111,
        //     addr & 0xFF,
        // );
        match addr & 0xFF {
            0xBE => {
                self.ram.borrow_mut().cpu_write(self.cpu_addr, val);
                // println!(
                //     "
                // ===== WRITING 0x{:02x} TO 0x{:04x} IN PPU ====
                // ",
                //     val, self.cpu_addr
                // );
                self.cpu_addr += 1;
            }
            0xBF => match self.rw_state {
                RWState::None => {
                    self.rw_state = RWState::First(val);
                }
                RWState::First(fst) => {
                    if val >> 6 == 0b01 {
                        // set cpu address
                        let high = val as u16 & 0b0011_1111;
                        self.cpu_addr = (high << 8) | fst as u16;
                    // println!("Setting PPU Addr to 0x{:04x}", self.cpu_addr);
                    } else if val >> 4 == 0b1000 {
                        // write to register
                        let reg = val & 0b1111;
                        if reg <= 0b111 {
                            self.registers[reg as usize] = fst;
                        }
                        // println!("Writing 0x{:02x} to register {} in PPU", fst, reg);
                    }

                    self.rw_state = RWState::None;
                }
            },
            _ => panic!("Attempting to write to PPU at IO address {}", addr),
        };

        false
    }
}

trait Renderer {
    fn draw(&mut self);
}

trait ImageWriter {
    fn image(&mut self) -> &mut Canvas;
    fn zoom(&self) -> u16;

    fn color_pixel(&mut self, color: Color, x: u16, y: u16) {
        let x_begin = x * self.zoom();
        let y_begin = y * self.zoom();
        let x_end = (x + 1) * self.zoom();
        let y_end = (y + 1) * self.zoom();

        for point_x in x_begin..x_end {
            for point_y in y_begin..y_end {
                self.image()
                    .put_pixel(point_x.into(), point_y.into(), color);
            }
        }
    }
}
