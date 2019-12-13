#[macro_use]
#[deny(missing_docs)]
extern crate num_derive;

mod bus;
mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::new(Vec::new());
    let i: u8 = 0xFA; // -128
    let j: u8 = 0x80; // 100

    println!("{} - {} = {}", j as i8, i as i8, sub(j, i) as i8);
    println!("{} - {} = {}", i as i8, j as i8, sub(i, j) as i8);
}

fn sub(l: u8, r: u8) -> u8 {
    let l = l as u16;
    let r = r as u16;

    let result = if l >= r {
        l - r
    } else {
        let remainder = r - l;
        0xFF - remainder + 1
    };

    result as u8
}
