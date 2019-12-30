#[macro_use]
#[deny(missing_docs)]
extern crate num_derive;

mod bus;
mod cpu;

use cpu::*;

fn main() {
    let buf = vec![
        0x1e, 0x69, 0x16, 0xDB, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19, 0x09, 0xeb,
        0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed, 0x42, 0xcb, 0x3c,
        0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30, 0xe3,
    ];

    let mut cpu = Cpu::new(buf);

    while cpu.next_byte_no_inc() != 0 {
        cpu.do_operation();
    }
    let print_first = move || {
        println!("1). Reg DE: {}", cpu.reg_value_16(RegisterCode16::DE));
        println!("1). Reg HL: {}", cpu.reg_value_16(RegisterCode16::HL));
        println!("1). Clock:  {}\n", cpu.clock());
    };

    let buf = vec![
        0x1e, 0x24, 0x16, 0x00, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19, 0x09, 0xeb,
        0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed, 0x42, 0xcb, 0x3c,
        0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30, 0xe3,
    ];

    let mut cpu = Cpu::new(buf);

    while cpu.next_byte_no_inc() != 0 {
        cpu.do_operation();
    }

    print_first();

    println!("2). Reg DE: {}", cpu.reg_value_16(RegisterCode16::DE));
    println!("2). Reg HL: {}", cpu.reg_value_16(RegisterCode16::HL));
    println!("2). Clock:  {}", cpu.clock());
}

#[cfg(test)]
mod tests {
    use super::cpu::*;

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

    #[rustfmt::skip]
    fn get_buf2() -> Vec<u8> {
        vec![
            0x06, 0x0A,           // LD B, 10
            0x3C,                 // INC A
            0x10, -3 as i8 as u8, // Move back to the INC A
        ]
    }

    #[test]
    fn test_cpu1() {
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

    #[test]
    fn test_cpu2() {
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

        let mut cpu = Cpu::new(get_buf2());
        while cpu.reg_value(RegisterCode::A) < 10 {
            cpu.do_operation();
        }

        assert_eq!(10, cpu.reg_value(RegisterCode::A));
    }

    #[test]
    fn test_cpu_sqrt() {
        let buf = vec![
            0x1e, 0x69, 0x16, 0xDB, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19, 0x09,
            0xeb, 0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed, 0x42,
            0xcb, 0x3c, 0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30, 0xe3,
        ];

        let mut cpu = Cpu::new(buf);

        while cpu.next_byte_no_inc() != 0 {
            cpu.do_operation();
        }

        assert_eq!(237, cpu.reg_value_16(RegisterCode16::HL));
    }

    #[test]
    fn test_cpu_sqrt2() {
        let buf = vec![
            0x1e, 0x24, 0x16, 0x00, 0x01, 0x00, 0x80, 0x61, 0x69, 0xcb, 0x38, 0xcb, 0x19, 0x09,
            0xeb, 0xed, 0x52, 0x38, 0x04, 0xeb, 0x09, 0x18, 0x05, 0x19, 0xeb, 0xb7, 0xed, 0x42,
            0xcb, 0x3c, 0xcb, 0x1d, 0xcb, 0x38, 0xcb, 0x19, 0x30, 0xe3,
        ];

        let mut cpu = Cpu::new(buf);

        while cpu.next_byte_no_inc() != 0 {
            cpu.do_operation();
        }

        assert_eq!(6, cpu.reg_value_16(RegisterCode16::HL));
    }
}
