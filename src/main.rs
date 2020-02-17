extern crate bus;
extern crate glutin_window;
extern crate graphics;
extern crate image as im;
extern crate opengl_graphics;
extern crate piston;
extern crate z80;

use emulator::Emulator;
use opengl_graphics::{GlyphCache, OpenGL, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::ButtonEvent;
use piston::window::WindowSettings;
use piston::EventLoop;
use piston::{ButtonArgs, RenderEvent};
use piston_window::*;
use std::path::PathBuf;

mod emulator;

pub struct App {
    pub emulator: Emulator,
}

impl App {
    fn update(&mut self) {
        self.emulator.refresh();
    }

    fn input(&mut self, args: &ButtonArgs) {
        self.emulator.input(args);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: PistonWindow = WindowSettings::new("sg-1000", [2 * 256, 2 * 192])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };

    // let ref mut glyphs = GlyphCache::new("assets/FiraMono-Regular.ttf", (), texture_settings)
    //     .expect("Could not load font");

    let path = std::env::args()
        .nth(1)
        .expect("The path of the game was not specified!");

    // Create a new game and run it.
    let mut app = App {
        emulator: emulator::Emulator::new(&PathBuf::from(path)),
    };

    let mut texture: G2dTexture = Texture::from_image(
        &mut texture_context,
        &app.emulator.ppu.borrow_mut().get_canvas().unwrap(),
        &TextureSettings::new(),
    )
    .unwrap();

    let mut events = Events::new(EventSettings::new().max_fps(60));
    while let Some(e) = events.next(&mut window) {
        if let Some(_) = e.render_args() {
            app.update();

            if let Some(canvas) = app.emulator.ppu.borrow_mut().get_canvas() {
                texture.update(&mut texture_context, &canvas).unwrap();
            }
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([0.0, 0.0, 0.0, 1.0], g);
                image(&texture, c.transform, g);
            });
        }

        if let Some(args) = e.button_args() {
            app.input(&args);
        }
    }
}
