use crate::{
    ppu::{sprites::SpriteRenderer, Color, ImageWriter, Ppu, Renderer, COLORS},
    Canvas,
};
use bus::BusConnectable;

const LINE_WIDTH: u16 = 32;

pub struct Graphics1Renderer<'a> {
    ppu: &'a mut Ppu,
    zoom: u16,
    line: u16,
}

impl<'a> Graphics1Renderer<'a> {
    pub fn new(ppu: &'a mut Ppu, zoom: u16, line: u16) -> Graphics1Renderer<'a> {
        Graphics1Renderer { ppu, zoom, line }
    }

    fn extract_color(color_entry: u8) -> (Color, Color) {
        (
            COLORS[(color_entry as usize >> 4) & 0x0F],
            COLORS[color_entry as usize & 0x0F],
        )
    }

    fn cell_row(&self) -> u16 {
        self.line / 8
    }

    fn cell_sub_row(&self) -> u16 {
        self.line % 8
    }
}

impl<'a> Renderer for Graphics1Renderer<'a> {
    fn draw(&mut self) {
        let zoom = self.zoom();
        let line = self.line;

        let name_tbl = self.ppu.name_table();
        let patt_tbl = self.ppu.pattern_gen_table();
        let colr_tbl = self.ppu.color_table();

        for cell in 0..LINE_WIDTH {
            let name_entry_ptr = name_tbl + self.cell_row() * LINE_WIDTH + cell;
            let name_entry = self.ppu.ram.borrow_mut().cpu_read(name_entry_ptr) as u16;

            let colr_entry_ptr = colr_tbl + name_entry / 8;
            let colr_entry = self.ppu.ram.borrow_mut().cpu_read(colr_entry_ptr);

            let (color1, color0) = Graphics1Renderer::extract_color(colr_entry);

            let patt_entry_ptr = patt_tbl + name_entry * 8 + self.cell_sub_row();
            let mut patt_entry = self.ppu.ram.borrow_mut().cpu_read(patt_entry_ptr);

            for bit_num in 0..8 {
                let color = if patt_entry & 0b1000_0000 > 0 {
                    color1
                } else {
                    color0
                };

                self.color_pixel(color, cell * 8 + bit_num, line);
                patt_entry <<= 1;
            }
        }

        SpriteRenderer::new(&mut self.ppu, zoom, line).draw();
    }
}

impl<'a> ImageWriter for Graphics1Renderer<'a> {
    fn zoom(&self) -> u16 {
        self.zoom
    }

    fn image(&mut self) -> &mut Canvas {
        &mut self.ppu.next_canvas
    }
}
