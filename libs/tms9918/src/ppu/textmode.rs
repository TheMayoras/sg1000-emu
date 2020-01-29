use crate::{
    ppu::{ImageWriter, Ppu, Renderer},
    Canvas,
};
use bus::BusConnectable;

const LINE_WIDTH: u16 = 40;

pub struct TextModeRenderer<'a> {
    ppu: &'a mut Ppu,
    zoom: u16,
    line: u16,
}

impl<'a> TextModeRenderer<'a> {
    pub fn new(ppu: &'a mut Ppu, zoom: u16, line: u16) -> TextModeRenderer<'a> {
        TextModeRenderer { ppu, zoom, line }
    }

    #[inline]
    fn cell_row(&self) -> u16 {
        self.line / 8
    }

    #[inline]
    fn inner_cell_row(&self) -> u16 {
        self.line % 8
    }
}

impl<'a> Renderer for TextModeRenderer<'a> {
    fn draw(&mut self) {
        let back_color = self.ppu.text_back_color();
        let text_color = self.ppu.text_color();

        let name_tbl = self.ppu.name_table();
        let patt_tbl = self.ppu.pattern_gen_table();
        let cell_row = self.cell_row();
        for i in 0..LINE_WIDTH {
            let name_tbl_ptr = name_tbl + cell_row * LINE_WIDTH + i; // get the name table entry
            let mut patt_tbl_ptr = self.ppu.ram.borrow_mut().cpu_read(name_tbl_ptr) as u16;
            patt_tbl_ptr = patt_tbl + 8 * patt_tbl_ptr; // get the pointer to the pattern generator entry
            patt_tbl_ptr += self.inner_cell_row() as u16; // get the row in the pattern table

            let mut pattern = self.ppu.ram.borrow_mut().cpu_read(patt_tbl_ptr); // the actual row of the pattern cell we care about
            for bit_num in 0..7 {
                if (pattern & 0b1000_0000) == 0 {
                    self.color_pixel(back_color, i * 6 + bit_num, self.line);
                } else {
                    self.color_pixel(text_color, i * 6 + bit_num, self.line);
                }
                pattern <<= 1;
            }
        }
    }
}

impl<'a> ImageWriter for TextModeRenderer<'a> {
    fn zoom(&self) -> u16 {
        self.zoom
    }

    fn image(&mut self) -> &mut Canvas {
        &mut self.ppu.next_canvas
    }
}
