pub extern crate image as im;

pub mod ppu;

pub type Canvas = im::ImageBuffer<im::Rgba<u8>, Vec<u8>>;
