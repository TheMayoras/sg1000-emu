use crate::{
    ppu::{Color, ImageWriter, Ppu, Renderer, COLORS},
    Canvas,
};
use bus::BusConnectable;

const LINE_WIDTH: u16 = 242;

struct Sprite {
    x: u16,
    y: u16,
    name_entry: u16,
    early_clock: bool,
    color: Color,
}

impl Sprite {
    pub fn new(x: u8, y: u8, name_ptr: u8, clock_color: u8) -> Sprite {
        Sprite {
            x: x as u16,
            y: y as u16,
            name_entry: name_ptr as u16,
            early_clock: clock_color >= 0x80,
            color: COLORS[clock_color as usize & 0x0F],
        }
    }
}

pub struct SpriteRenderer<'a> {
    ppu: &'a mut Ppu,
    zoom: u16,
    line: u16,
}

impl<'a> SpriteRenderer<'a> {
    pub fn new(ppu: &'a mut Ppu, zoom: u16, line: u16) -> SpriteRenderer<'a> {
        SpriteRenderer { ppu, zoom, line }
    }

    /// TODO: use the bleeding in of the Early Clock Bit
    fn draw_8x8(&mut self) {
        let mut count = 0;
        let mut positions = Vec::new();

        let attr_tbl = self.ppu.sprite_attr_table();
        let gen_tbl = self.ppu.sprite_patt_gen_table();

        for spr in 0..32 {
            let attr_ptr = attr_tbl + 4 * spr;
            let sprite;
            {
                let mut ram = self.ppu.ram.borrow_mut();
                sprite = Sprite::new(
                    ram.cpu_read(attr_ptr),
                    ram.cpu_read(attr_ptr + 1),
                    ram.cpu_read(attr_ptr + 2),
                    ram.cpu_read(attr_ptr + 3),
                );
            }

            // make sure the sprite is actaully on the current line
            if sprite.y <= self.line && sprite.y + 8 > self.line {
                count += 1;
                if count >= 5 {
                    self.ppu.set_5th_sprite(spr as u8);
                    return;
                }

                let pattern_line = self.line - sprite.y;
                let pattern_ptr = gen_tbl + sprite.name_entry * 8 + pattern_line;
                let mut pattern = self.ppu.ram.borrow_mut().cpu_read(pattern_ptr);

                for i in 0..8 {
                    // check sprite coincidence
                    let coincident: bool = match positions.binary_search(&(sprite.x + i)) {
                        Ok(_) => {
                            self.ppu.set_coincidence_flag();
                            true
                        }
                        Err(pos) => {
                            positions.insert(pos, sprite.x + i);
                            false
                        }
                    };

                    // if the bit is set for this pixel and it is not already taken up
                    if pattern & 0x80 > 0 && !coincident {
                        self.color_pixel(sprite.color, sprite.x + i, self.line);
                    }

                    pattern <<= 1;
                }
            }
        }
    }

    fn draw_16x16(&mut self) {
        let mut positions = Vec::new();

        let attr_tbl = self.ppu.sprite_attr_table();
        let gen_tbl = self.ppu.sprite_patt_gen_table();

        for spr in (0..32).rev() {
            let attr_ptr = attr_tbl + 4 * spr;
            let sprite;
            {
                let mut ram = self.ppu.ram.borrow_mut();
                sprite = Sprite::new(
                    ram.cpu_read(attr_ptr),
                    ram.cpu_read(attr_ptr + 1),
                    ram.cpu_read(attr_ptr + 2),
                    ram.cpu_read(attr_ptr + 3),
                );
            }

            // check sprite coincidence
            for i in 0..8 {
                match positions.binary_search(&(sprite.x + i)) {
                    Ok(_) => self.ppu.set_coincidence_flag(),
                    Err(pos) => positions.insert(pos, sprite.x + i),
                }
            }

            // make sure the sprite is actaully on the current line
            if sprite.y <= self.line && sprite.y + 8 > self.line {
                let pattern_line = self.line - sprite.y;
                let pattern_ptr = gen_tbl + sprite.name_entry * 8 + pattern_line;
                let pattern = self.ppu.ram.borrow_mut().cpu_read(pattern_ptr);

                for i in 0..8 {
                    // if the bit is set for this pixel
                    if pattern & 0x80 > 0 {
                        self.color_pixel(sprite.color, sprite.x + i, self.line);
                    }
                }
            }
        }
    }
}

impl<'a> Renderer for SpriteRenderer<'a> {
    fn draw(&mut self) {
        let (size, zoom) = self.ppu.get_sprite_size();
        self.zoom *= zoom as u16;

        if size == 8 {
            self.draw_8x8();
        } else {
            self.draw_16x16();
        }
    }
}

impl<'a> ImageWriter for SpriteRenderer<'a> {
    fn zoom(&self) -> u16 {
        self.zoom
    }

    fn image(&mut self) -> &mut Canvas {
        &mut self.ppu.next_canvas
    }
}
