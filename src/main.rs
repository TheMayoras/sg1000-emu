#[macro_use]
#[deny(missing_docs)]
extern crate num_derive;

mod bus;
mod cpu;

use cpu::*;

fn main() {
    let buf = get_buf();
    let mut cpu = Cpu::new(buf);

    cpu.do_operation();
    cpu.do_operation();
    cpu.do_operation();
    cpu.do_operation();
    cpu.do_operation();
    cpu.do_operation();

    assert_eq!(0xFF, cpu.reg_value(RegisterCode::A));
    assert_eq!(0x81, cpu.reg_value(RegisterCode::C));
    assert_eq!(0x01, cpu.reg_value(RegisterCode::B));
    assert!(cpu.flag(Flags::Carry));
}

#[rustfmt::skip]
fn get_buf() -> Vec<u8> {
    vec![
        0x3E, 0x80, // LD A, *
        0x06, 0x01, // LD B, *
        0x80,       // Add A, B
        0x4F,       // LD C, A
        0x3E, 0x00, // Ld A, *
        0x90,       // Sub A, B
    ]
}
