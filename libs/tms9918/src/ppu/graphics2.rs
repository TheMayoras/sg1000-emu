use crate::{
    ppu::{ImageWriter, Ppu, Renderer, COLORS},
    Canvas,
};
use bus::BusConnectable;

const LINE_WIDTH: u16 = 32;

pub struct Graphics2Renderer<'a> {
    ppu: &'a mut Ppu,
    zoom: u16,
    line: u16,
}

impl<'a> Graphics2Renderer<'a> {
    pub fn new(ppu: &'a mut Ppu, zoom: u16, line: u16) -> Graphics2Renderer<'a> {
        Graphics2Renderer { ppu, zoom, line }
    }

    fn cell_row(&self) -> u16 {
        self.line / 8
    }

    fn cell_sub_row(&self) -> u16 {
        self.line % 8
    }
    fn block(&self) -> u16 {
        // there are 8 lines per cell * 8 rows of cells per block
        self.line / 64
    }
}

impl<'a> Renderer for Graphics2Renderer<'a> {
    fn draw(&mut self) {
        let cells_start = self.cell_row();
        let block = self.block();
        let patt_tbl = self.ppu.pattern_gen_table() + 2048 * block;
        let name_tbl = self.ppu.name_table();
        let color_tbl = self.ppu.color_table() + 2048 * block;

        for i in 0..LINE_WIDTH {
            let cell_ptr = name_tbl + cells_start + i;
            let name_entry = self.ppu.ram.borrow_mut().cpu_read(cell_ptr) as u16;

            let gen_ptr = patt_tbl + name_entry * 8 + self.cell_sub_row();
            let mut patt = self.ppu.ram.borrow_mut().cpu_read(gen_ptr);

            let color_ptr = color_tbl + name_entry * 8 + self.cell_sub_row();
            let color = self.ppu.ram.borrow_mut().cpu_read(color_ptr);
            let color0 = COLORS[(color & 0x0F) as usize];
            let color1 = COLORS[(color >> 4) as usize];

            for bit_num in 0..8 {
                if patt & 0b1000_0000 == 0 {
                    self.color_pixel(color0, i * 8 + bit_num, self.line);
                } else {
                    self.color_pixel(color1, i * 8 + bit_num, self.line);
                }

                patt <<= 1;
            }
        }
    }
}

impl<'a> ImageWriter for Graphics2Renderer<'a> {
    fn zoom(&self) -> u16 {
        self.zoom
    }

    fn image(&mut self) -> &mut Canvas {
        &mut self.ppu.next_canvas
    }
}
