extern crate bus;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate z80_cpu_emu;

use bus::bus::Bus;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{Filter, GlGraphics, GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{Button, ButtonArgs, ButtonState, Key};
use z80_cpu_emu::cpu::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    cpu: Cpu,
}

impl App {
    fn render(&mut self, args: &RenderArgs, glyphs: &mut GlyphCache) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);
        let clock = self.cpu.clock();

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let transform = c.transform.trans(x, y);

            let transform = c.transform.trans(x, y);

            Text::new_color(WHITE, 14)
                .draw("Clock:", glyphs, &c.draw_state, transform, gl)
                .unwrap();

            Text::new_color(WHITE, 14)
                .draw(
                    &format!("{}", clock),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(x, y + 14.0),
                    gl,
                )
                .unwrap();
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {}

    fn input(&mut self, args: &ButtonArgs) {
        match args.button {
            Button::Keyboard(Key::Space) => match args.state {
                ButtonState::Press => {
                    self.cpu.do_operation();
                }
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() {
    let cpu = Cpu::new(
        Bus::builder()
            .add(vec![
                0x1e, 0x24, 0x16, 0x00, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19, 0x09,
                0xeb, 0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed, 0x42,
                0xcb, 0x3c, 0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30, 0xe3,
            ])
            .build(),
    );

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let ref mut glyphs = GlyphCache::new("assets/FiraSans-Regular.ttf", (), texture_settings)
        .expect("Could not load font");

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        cpu,
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
            app.input(&args);
        }
    }
}
