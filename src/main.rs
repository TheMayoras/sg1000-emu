extern crate bus;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate z80;

use bus::{bus::*, *};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonArgs, ButtonState, Key};
use std::{cell::RefCell, rc::Rc, time::*};
use z80::cpu::*;

mod emulator;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    emulator: emulator::Emulator,
    previous_time: Instant,
    data: MutRef<Vec<u8>>,
}

impl App {
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        use graphics::*;

        self.emulator.cpu.do_operation();

        const BLACK: [f32; 4] = [0.15, 0.15, 0.15, 0.8];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const RED: [f32; 4] = [0.8, 0.25, 0.25, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 0.75, 1.0];

        let (x, y) = (
            args.window_size[0] / 2.0 - 8.0 * ("Clock:".len() as f64),
            args.window_size[1] / 2.0,
        );
        let clock = self.emulator.cpu.clock();
        let bc = self.emulator.cpu.reg_value_16(RegisterCode16::BC);
        let de = self.emulator.cpu.reg_value_16(RegisterCode16::DE);
        let hl = self.emulator.cpu.reg_value_16(RegisterCode16::HL);
        let a = self.emulator.cpu.reg_value(RegisterCode::A);
        let flags = self.emulator.cpu.reg_value(RegisterCode::Flags);

        let duration = self.previous_time.elapsed();
        self.previous_time = Instant::now();

        let data = format!("{:?}", self.data.borrow());
        let pc = self.emulator.cpu.reg_value_16(RegisterCode16::PC);
        let mut col_count = 0;
        self.data
            .borrow()
            .iter()
            .take(self.emulator.cpu.reg_value_16(RegisterCode16::PC) as usize)
            .for_each(|val| col_count += val.to_string().len() + 2);

        let col = std::iter::repeat(' ')
            .take(col_count + 1)
            .collect::<String>();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y);

            Text::new_color(WHITE, 14)
                .draw(
                    &format!("Clock: {}", clock),
                    glyphs,
                    &c.draw_state,
                    transform,
                    gl,
                )
                .unwrap();

            let transform = transform.trans(0.0, 14.0);

            Text::new_color(RED, 14)
                .draw(&format!("BC: {}", bc), glyphs, &c.draw_state, transform, gl)
                .unwrap();
            let transform = transform.trans(0.0, 14.0);

            Text::new_color(RED, 14)
                .draw(&format!("DE: {}", de), glyphs, &c.draw_state, transform, gl)
                .unwrap();
            let transform = transform.trans(0.0, 14.0);

            Text::new_color(RED, 14)
                .draw(&format!("HL: {}", hl), glyphs, &c.draw_state, transform, gl)
                .unwrap();
            let transform = transform.trans(0.0, 14.0);

            Text::new_color(RED, 14)
                .draw(&format!("A:  {}", a), glyphs, &c.draw_state, transform, gl)
                .unwrap();
            let transform = transform.trans(0.0, 14.0);

            Text::new_color(RED, 14)
                .draw(&format!("F:  0b{:08b}", flags), glyphs, &c.draw_state, transform, gl)
                .unwrap();

            let transform = transform.trans(0.0, 14.0);

            Text::new_color(BLUE, 14)
                .draw(
                    &format!("Elapsed: {} ms", duration.as_millis()),
                    glyphs,
                    &c.draw_state,
                    transform,
                    gl,
                )
                .unwrap();

            let transform = transform.trans(-600.0, 14.0);

            Text::new_color(BLUE, 14)
                .draw(&data, glyphs, &c.draw_state, transform, gl)
                .unwrap();

            let transform = transform.trans(0.0, 14.0);

            // Text::new_color(BLUE, 14)
            //     .draw(&pc.to_string(), glyphs, &c.draw_state, transform, gl)
            //     .unwrap();

            Text::new_color(BLUE, 14)
                .draw(&format!("{}^", col), glyphs, &c.draw_state, transform, gl)
                .unwrap();
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}

    fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::Space) => match args.state {
                ButtonState::Press => {
                    self.emulator.cpu.do_operation();
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() {
    let data: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(vec![
        0x1e, 0x31, 0x16, 0xF2, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19, 0x09, 0xeb,
        0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed, 0x42, 0xcb, 0x3c,
        0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30, 0xe3,
    ]));
    let connectable: Rc<RefCell<dyn BusConnectable>> =
        Rc::clone(&data) as Rc<RefCell<dyn BusConnectable>>;
    // let cpu = Cpu::new(&Rc::new(RefCell::new(
    //     Bus::builder().add_ref(&connectable).build(),
    // )));

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V2_1;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("sg-1000", [2 * 256, 2 * 192])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraMono-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        emulator: emulator::Emulator::new(),
        previous_time: Instant::now(),
        data,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, glyphs);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

        if let Some(args) = e.button_args() {
            // app.input(&args);
        }
    }
}

// extern crate image as im;
// extern crate piston_window;

// use piston_window::*;

// fn main() {
//     let opengl = OpenGL::V3_2;
//     let (width, height) = (300, 300);
//     let mut window: PistonWindow = WindowSettings::new("piston: paint", (width, height))
//         .exit_on_esc(true)
//         .graphics_api(opengl)
//         .build()
//         .unwrap();

//     let mut canvas = im::ImageBuffer::new(width, height);
//     let mut draw = false;
//     let mut texture_context = TextureContext {
//         factory: window.factory.clone(),
//         encoder: window.factory.create_command_buffer().into(),
//     };
//     let mut texture: G2dTexture =
//         Texture::from_image(&mut texture_context, &canvas, &TextureSettings::new()).unwrap();

//     let mut last_pos: Option<[f64; 2]> = None;

//     while let Some(e) = window.next() {
//         if let Some(_) = e.render_args() {
//             texture.update(&mut texture_context, &canvas).unwrap();
//             window.draw_2d(&e, |c, g, device| {
//                 // Update texture before rendering.
//                 texture_context.encoder.flush(device);

//                 clear([0.5; 4], g);
//                 image(&texture, c.transform, g);
//             });
//         }
//         if let Some(button) = e.press_args() {
//             if button == Button::Mouse(MouseButton::Left) {
//                 draw = true;
//             }
//         };
//         if let Some(button) = e.release_args() {
//             if button == Button::Mouse(MouseButton::Left) {
//                 draw = false;
//                 last_pos = None
//             }
//         };
//         if draw {
//             if let Some(pos) = e.mouse_cursor_args() {
//                 let (x, y) = (pos[0] as f32, pos[1] as f32);

//                 if let Some(p) = last_pos {
//                     let (last_x, last_y) = (p[0] as f32, p[1] as f32);
//                     let distance = vec2_len(vec2_sub(p, pos)) as u32;

//                     for i in 0..distance {
//                         let diff_x = x - last_x;
//                         let diff_y = y - last_y;
//                         let delta = i as f32 / distance as f32;
//                         let new_x = (last_x + (diff_x * delta)) as u32;
//                         let new_y = (last_y + (diff_y * delta)) as u32;
//                         if new_x < width && new_y < height {
//                             canvas.put_pixel(new_x, new_y, im::Rgba([0, 0, 0, 255]));
//                         };
//                     }
//                 };

//                 last_pos = Some(pos)
//             };
//         }
//     }
// }
