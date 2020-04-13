use super::*;
use crate::flag;
use bus::ram::Ram;
use bus::BusConnectable;
use std::{cell::RefCell, rc::Rc};

impl Into<Cpu> for Vec<u8> {
    fn into(self) -> Cpu {
        let ram = Ram::builder().data(self).build();
        let bus = Bus::new(vec![
            Rc::new(RefCell::new(ram)) as MutRef<dyn BusConnectable>
        ]);

        Cpu::new(
            &Rc::new(RefCell::new(bus)),
            &Rc::new(RefCell::new(Bus::default())),
        )
    }
}

#[inline]
fn get_cpu() -> Cpu {
    Cpu::new(
        &Rc::new(RefCell::new(Bus::new(vec![Rc::new(RefCell::new(vec![
            0xab, 0xcd, 0xef,
        ]))]))),
        &Rc::new(RefCell::new(Bus::default())),
    )
}

#[test]
fn test_set_flag_macro() {
    use super::Flags::*;

    let mut cpu: Cpu = Vec::new().into();
    cpu.set_flag(Carry, true);
    flag!(cpu; set Flags::Carry);
    assert!(cpu.flag(Carry));

    flag!(cpu; unset Carry);
    assert!(!cpu.flag(Carry));
}

#[test]
fn test_inc_clock() {
    let mut cpu = Cpu::new(
        &Rc::new(RefCell::new(Bus::default())),
        &Rc::new(RefCell::new(Bus::default())),
    );
    assert_eq!(0, cpu.clock());

    cpu.tick_clock(1);
    assert_eq!(1, cpu.clock());

    cpu.tick_clock(5);
    assert_eq!(6, cpu.clock());
}

#[test]
fn test_set_reg_a() {
    let mut cpu = Cpu::new(
        &Rc::new(RefCell::new(Bus::default())),
        &Rc::new(RefCell::new(Bus::default())),
    );
    cpu.set_reg_value(RegisterCode::A, 10);
    assert_eq!(10, cpu.reg_value(RegisterCode::A));
}

#[test]
fn test_register_16() {
    let mut cpu = Cpu::new(
        &Rc::new(RefCell::new(Bus::default())),
        &Rc::new(RefCell::new(Bus::default())),
    );
    cpu.set_reg_value(RegisterCode::B, 0xBB);
    cpu.set_reg_value(RegisterCode::C, 0xCC);
    assert_eq!(0xBBCC, cpu.reg_value_16(RegisterCode16::BC));
}

#[test]
fn test_immediate_addressing() {
    let mut cpu = Cpu::new(
        &Rc::new(RefCell::new(Bus::new(vec![Rc::new(RefCell::new(vec![
            0xab, 0xbc, 0xde,
        ]))]))),
        &Rc::new(RefCell::new(Bus::default())),
    );

    assert_eq!(0xab, cpu.imm_addr());
}

#[test]
// note that this uses two bytes and we are in little endian order
fn test_immediate_extended_addressing() {
    let mut cpu = Cpu::new(
        &Rc::new(RefCell::new(Bus::new(vec![Rc::new(RefCell::new(vec![
            0xab, 0xcd, 0xef,
        ]))]))),
        &Rc::new(RefCell::new(Bus::default())),
    );

    assert_eq!(0xcdab, cpu.imm_addr_ex());
}

#[test]
fn test_relative_addressing() {
    let mut cpu = Cpu::new(
        &Rc::new(RefCell::new(Bus::new(vec![Rc::new(RefCell::new(vec![
            0xff, 0xff, 0,
        ]))]))),
        &Rc::new(RefCell::new(Bus::default())),
    );
    assert_eq!(0, cpu.rel_addr());

    let pc = cpu.get_pc() as i16;
    let pc = (pc + 1 - 1) as u16;
    assert_eq!(pc, cpu.rel_addr());
}

#[test]
fn test_relative_addressing_pc_cast_is_neg() {
    let mut vec = Vec::with_capacity(0xff + 5);
    vec.resize(0xff + 5, 0);
    for i in 0..0xff + 5 {
        vec[i] = (i % 0xff) as u8;
    }

    let mut cpu = Cpu::new(
        &Rc::new(RefCell::new(Bus::new(vec![Rc::new(RefCell::new(vec))]))),
        &Rc::new(RefCell::new(Bus::default())),
    );
    // we have vec[0, 1, 2, 3, 4, ..., 0xff, 0, 1, 2, 3, 4]
    cpu.set_pc(0xf0); // 0xf0 = 240 or 0xf0 = -16
    assert_eq!(0xf1 - 16, cpu.rel_addr());
}

#[test]
fn test_extended_addressing() {
    let mut cpu = get_cpu();

    assert_eq!(0xcdab, cpu.ext_addr());
}

#[test]
fn test_register_indexed_addressing() {
    let mut cpu = get_cpu();
    cpu.set_reg_value_16(RegisterCode16::IY, 0xa015);
    // 0xAB = -85
    assert_eq!(0xa015 - 85, cpu.index_addr(RegisterCode16::IY));

    let mut cpu = get_cpu();
    cpu.set_reg_value_16(RegisterCode16::IY, 0x0000);
    // 0xAB = -85
    assert_eq!(0xFFFF - 84, cpu.index_addr(RegisterCode16::IY));

    let mut cpu: Cpu = vec![0xFF].into();
    cpu.set_reg_value_16(RegisterCode16::IY, 0x0000);
    assert_eq!(0xFFFF, cpu.index_addr(RegisterCode16::IY));

    let mut cpu: Cpu = vec![0x01].into();
    cpu.set_reg_value_16(RegisterCode16::IY, 0xFFFF);
    assert_eq!(0x0000, cpu.index_addr(RegisterCode16::IY));
}

#[test]
fn test_indirect_reg_addr() {
    let mut cpu = get_cpu();
    cpu.set_reg_value(RegisterCode::H, 0xab);
    cpu.set_reg_value(RegisterCode::L, 0xcd);

    assert_eq!(0xabcd, cpu.indirect_reg_addr(RegisterCode16::HL));
}

#[test]
fn test_set_flags() {
    let mut cpu = get_cpu();

    cpu.set_flag(Flags::Carry, false);
    assert_eq!(0, (cpu.reg[RegisterCode::Flags as usize]) & 1);
    cpu.set_flag(Flags::Carry, true);
    assert_eq!(1, (cpu.reg[RegisterCode::Flags as usize]) & 1);

    cpu.set_flag(Flags::Sign, false);
    assert_eq!(0, (cpu.reg[RegisterCode::Flags as usize] >> 7) & 1);

    cpu.set_flag(Flags::Sign, true);
    assert_eq!(1, (cpu.reg[RegisterCode::Flags as usize] >> 7) & 1);
}

#[test]
fn test_get_flags() {
    let mut cpu = get_cpu();

    cpu.reg[RegisterCode::Flags as usize] = 0b10; //< Subtract is now set
    assert_eq!(true, cpu.flag(Flags::Subtract));

    cpu.reg[RegisterCode::Flags as usize] = 0b11110;
    assert_eq!(false, cpu.flag(Flags::Carry));
}

/* ----------------     test inc     ------------------- */
#[test]
fn test_inc_reg() {
    let mut cpu = get_cpu();

    // test normal
    cpu.set_reg_value(RegisterCode::A, 0x0);
    cpu.inc_reg(RegisterCode::A);
    assert_eq!(1, cpu.reg[RegisterCode::A as usize]);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    // test half carry flag
    cpu.set_reg_value(RegisterCode::A, 0b1101_1111);
    cpu.inc_reg(RegisterCode::A);
    assert_eq!(0b1110_0000, cpu.reg[RegisterCode::A as usize]);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));

    // test overflow
    cpu.set_reg_value(RegisterCode::A, 0xFF);
    cpu.inc_reg(RegisterCode::A);
    assert_eq!(0, cpu.reg[RegisterCode::A as usize]);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));

    // test wrap around to negative
    cpu.set_reg_value(RegisterCode::A, 0x7F);
    cpu.inc_reg(RegisterCode::A);
    assert_eq!(-128, cpu.reg[RegisterCode::A as usize] as i8);
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
}

#[test]
fn test_inc_addr() {
    let mut cpu: Cpu = vec![0x00, 0x80, 0xFF, 0x7F].into();

    // 0x00
    cpu.inc_addr(0x00);
    assert_eq!(0x01, cpu.fetch(0x00));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));

    // 0x80
    cpu.inc_addr(0x01);
    assert_eq!(-127 as i8 as u8, cpu.fetch(0x01));
    assert_eq!(0x81, cpu.fetch(0x01));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));

    // 0xFF
    cpu.inc_addr(0x02);
    assert_eq!(0, cpu.fetch(0x02));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));

    // 0x7F
    cpu.inc_addr(0x03);
    assert_eq!(-128 as i8 as u8, cpu.fetch(0x03));
    assert_eq!(0x80, cpu.fetch(0x03));
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
}

#[test]
fn test_inc_reg16() {
    use super::Flags::*;
    use super::RegisterCode16::*;

    let mut cpu: Cpu = Vec::new().into();
    let overflow = cpu.flag(OverflowParity);
    let sign = cpu.flag(Sign);
    let zero = cpu.flag(Zero);
    let carry = cpu.flag(Carry);
    let subtract = cpu.flag(Subtract);
    let halfcarry = cpu.flag(HalfCarry);

    cpu.set_reg_value_16(RegisterCode16::HL, 0);
    cpu.inc_reg16(HL);
    assert_eq!(1, cpu.reg_value_16(HL));
    assert_eq!(overflow, cpu.flag(OverflowParity));
    assert_eq!(sign, cpu.flag(Sign));
    assert_eq!(zero, cpu.flag(Zero));
    assert_eq!(carry, cpu.flag(Carry));
    assert_eq!(subtract, cpu.flag(Subtract));
    assert_eq!(halfcarry, cpu.flag(HalfCarry));

    cpu.set_reg_value_16(RegisterCode16::HL, 0xFFFF);
    cpu.inc_reg16(HL);
    assert_eq!(0, cpu.reg_value_16(HL));
    assert_eq!(overflow, cpu.flag(OverflowParity));
    assert_eq!(sign, cpu.flag(Sign));
    assert_eq!(zero, cpu.flag(Zero));
    assert_eq!(carry, cpu.flag(Carry));
    assert_eq!(subtract, cpu.flag(Subtract));
    assert_eq!(halfcarry, cpu.flag(HalfCarry));

    cpu.set_reg_value_16(RegisterCode16::HL, 0x7FFF);
    cpu.inc_reg16(HL);
    assert_eq!(0x8000, cpu.reg_value_16(HL));
    assert_eq!(overflow, cpu.flag(OverflowParity));
    assert_eq!(sign, cpu.flag(Sign));
    assert_eq!(zero, cpu.flag(Zero));
    assert_eq!(carry, cpu.flag(Carry));
    assert_eq!(subtract, cpu.flag(Subtract));
    assert_eq!(halfcarry, cpu.flag(HalfCarry));

    cpu.set_reg_value_16(RegisterCode16::HL, 0x8000);
    cpu.inc_reg16(HL);
    assert_eq!(-32767 as i16 as u16, cpu.reg_value_16(HL));
    assert_eq!(0x8001, cpu.reg_value_16(HL));
    assert_eq!(overflow, cpu.flag(OverflowParity));
    assert_eq!(sign, cpu.flag(Sign));
    assert_eq!(zero, cpu.flag(Zero));
    assert_eq!(carry, cpu.flag(Carry));
    assert_eq!(subtract, cpu.flag(Subtract));
    assert_eq!(halfcarry, cpu.flag(HalfCarry));
}

/* ----------------     test dec     ------------------- */
#[test]
fn test_dec_reg() {
    let mut cpu = get_cpu();

    // test normal dec
    cpu.set_reg_value(RegisterCode::A, 1);
    cpu.dec_reg(RegisterCode::A);
    assert_eq!(0, cpu.reg_value(RegisterCode::A));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test wrap around
    cpu.set_reg_value(RegisterCode::A, 0);
    cpu.dec_reg(RegisterCode::A);
    assert_eq!(0xFF, cpu.reg_value(RegisterCode::A));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test wrap around
    cpu.set_reg_value(RegisterCode::A, 0x80);
    cpu.dec_reg(RegisterCode::A);
    assert_eq!(0x7F, cpu.reg_value(RegisterCode::A));
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test half adder
    cpu.set_reg_value(RegisterCode::A, 0b1011_0000);
    cpu.dec_reg(RegisterCode::A);
    assert_eq!(0b1010_1111, cpu.reg_value(RegisterCode::A));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Subtract));
}

#[test]
fn test_dec_addr() {
    let mut cpu: Cpu = vec![0x00, 0x80, 0xFF, 0x7F].into();

    // 0x00
    cpu.dec_addr(0x00);
    assert_eq!(0xFF, cpu.fetch(0x00));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // 0x80
    cpu.dec_addr(0x01);
    assert_eq!(0x7F, cpu.fetch(0x01));
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // 0xFF
    cpu.dec_addr(0x02);
    assert_eq!(0xFE, cpu.fetch(0x02));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // 0x7F
    cpu.dec_addr(0x03);
    assert_eq!(0x7E, cpu.fetch(0x03));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Subtract));
}

#[test]
fn test_dec_reg16() {
    use super::Flags::*;
    use super::RegisterCode16::*;

    let mut cpu: Cpu = Vec::new().into();
    let overflow = cpu.flag(OverflowParity);
    let sign = cpu.flag(Sign);
    let zero = cpu.flag(Zero);
    let carry = cpu.flag(Carry);
    let subtract = cpu.flag(Subtract);
    let halfcarry = cpu.flag(HalfCarry);

    cpu.set_reg_value_16(HL, 0x0001);
    cpu.dec_reg16(HL);
    assert_eq!(0x0000, cpu.reg_value_16(HL));
    assert_eq!(overflow, cpu.flag(OverflowParity));
    assert_eq!(sign, cpu.flag(Sign));
    assert_eq!(zero, cpu.flag(Zero));
    assert_eq!(carry, cpu.flag(Carry));
    assert_eq!(subtract, cpu.flag(Subtract));
    assert_eq!(halfcarry, cpu.flag(HalfCarry));

    cpu.set_reg_value_16(HL, 0x0000);
    cpu.dec_reg16(HL);
    assert_eq!(0xFFFF, cpu.reg_value_16(HL));
    assert_eq!(overflow, cpu.flag(OverflowParity));
    assert_eq!(sign, cpu.flag(Sign));
    assert_eq!(zero, cpu.flag(Zero));
    assert_eq!(carry, cpu.flag(Carry));
    assert_eq!(subtract, cpu.flag(Subtract));
    assert_eq!(halfcarry, cpu.flag(HalfCarry));

    cpu.set_reg_value_16(HL, 0xFFFF);
    cpu.dec_reg16(HL);
    assert_eq!(0xFFFE, cpu.reg_value_16(HL));
    assert_eq!(overflow, cpu.flag(OverflowParity));
    assert_eq!(sign, cpu.flag(Sign));
    assert_eq!(zero, cpu.flag(Zero));
    assert_eq!(carry, cpu.flag(Carry));
    assert_eq!(subtract, cpu.flag(Subtract));
    assert_eq!(halfcarry, cpu.flag(HalfCarry));
}

/* ----------------     test add     ------------------- */
#[test]
fn test_add_val_val() {
    let mut cpu = get_cpu();

    let result = cpu.add_val_val(0x00, 0x00, false);
    assert_eq!(0, result);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    // test overflow to 0x80
    let result = cpu.add_val_val(0x70, 0x10, false);
    assert_eq!(0x80, result);
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    // test half carry
    let result = cpu.add_val_val(0b0000_1010, 0b0000_1110, false);
    assert_eq!(0b0001_1000, result);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    // test wrap around
    let result = cpu.add_val_val(0xF1, 0x0F, false);
    assert_eq!(0, result);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    // test val + 0
    let result = cpu.add_val_val(0x80, 0, false);
    assert_eq!(0x80, result);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    let result = cpu.add_val_val(0x70, 0, false);
    assert_eq!(0x70, result);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));
}

#[test]
fn test_add_a_reg() {
    let mut cpu = get_cpu();

    cpu.set_reg_value(RegisterCode::A, 0xFF);
    cpu.set_reg_value(RegisterCode::B, 1);
    cpu.add_a_reg(RegisterCode::B);
    assert_eq!(0, cpu.reg_value(RegisterCode::A));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    cpu.set_reg_value(RegisterCode::A, 0b11110110); // -10
    cpu.set_reg_value(RegisterCode::B, 15);
    cpu.add_a_reg(RegisterCode::B);
    assert_eq!(5, cpu.reg_value(RegisterCode::A));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    cpu.set_reg_value(RegisterCode::A, 0b10011100); // -100
    cpu.set_reg_value(RegisterCode::B, 15);
    cpu.add_a_reg(RegisterCode::B);
    assert_eq!(-85, cpu.reg_value(RegisterCode::A) as i8);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    cpu.set_reg_value(RegisterCode::A, 0x7F); // 127
    cpu.set_reg_value(RegisterCode::B, 1); // -> should wrap around and overflow
    cpu.add_a_reg(RegisterCode::B);
    assert_eq!(-128, cpu.reg_value(RegisterCode::A) as i8);
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Subtract));
}

#[test]
fn test_add_a_reg_carry() {
    use super::Flags::*;

    let mut cpu = get_cpu();

    cpu.set_reg_value(RegisterCode::A, 0xFE);
    cpu.set_reg_value(RegisterCode::B, 1);
    flag!(cpu; set Carry);
    cpu.add_a_reg_carry(RegisterCode::B);
    assert_eq!(0, cpu.reg_value(RegisterCode::A));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    cpu.set_reg_value(RegisterCode::A, 0b1111_0110); // -10
    cpu.set_reg_value(RegisterCode::B, 0b0000_1111);
    flag!(cpu; set Carry);
    cpu.add_a_reg_carry(RegisterCode::B);
    assert_eq!(6, cpu.reg_value(RegisterCode::A));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    cpu.set_reg_value(RegisterCode::A, 0b10011100); // -100
    cpu.set_reg_value(RegisterCode::B, 0b1111);
    flag!(cpu; set Carry);
    cpu.add_a_reg_carry(RegisterCode::B);
    assert_eq!(-84, cpu.reg_value(RegisterCode::A) as i8);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));

    // test wrap around
    cpu.set_reg_value(RegisterCode::A, 0x7F); // 127
    cpu.set_reg_value(RegisterCode::B, 0);
    flag!(cpu; set Carry);
    cpu.add_a_reg_carry(RegisterCode::B);
    assert_eq!(-128, cpu.reg_value(RegisterCode::A) as i8);
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));
}

#[test]
fn test_add_a_addr() {
    let mut cpu: Cpu = vec![0x01].into();

    cpu.set_reg_value(RegisterCode::A, 0xFF);
    cpu.add_a_addr(0);
    assert_eq!(0, cpu.reg_value(RegisterCode::A));
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Carry));
    assert_eq!(false, cpu.flag(Flags::Subtract));
}

/* ----------------     test sub     ------------------- */
#[test]
fn test_sub_val_val() {
    let mut cpu: Cpu = vec![].into();

    // test value - value
    let result = cpu.sub_val_val(0x80, 0x80, false);
    assert_eq!(result, 0x00);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(true, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test overflow
    let result = cpu.sub_val_val(0x80, 0x01, false);
    assert_eq!(result, 0x7F);
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test 0 - val
    let result = cpu.sub_val_val(0x00, 0x01, false);
    assert_eq!(result, 0xFF);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test val - 0
    let result = cpu.sub_val_val(0x80, 0, false);
    assert_eq!(result, 0x80);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    let result = cpu.sub_val_val(0x70, 0, false);
    assert_eq!(result, 0x70);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));
}

#[test]
fn test_sub_val_val_carry() {
    use super::Flags::*;

    let mut cpu: Cpu = vec![].into();

    // test value - value
    flag!(cpu; set Carry);
    let result = cpu.sub_val_val(0x80, 0x80, true);
    assert_eq!(result, 0xFF);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test overflow
    flag!(cpu; set Carry);
    let result = cpu.sub_val_val(0x80, 0x00, true);
    assert_eq!(result, 0x7F);
    assert_eq!(true, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test 0 - val
    flag!(cpu; set Carry);
    let result = cpu.sub_val_val(0x00, 0x00, true);
    assert_eq!(result, 0xFF);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    // test val - 0
    flag!(cpu; set Carry);
    let result = cpu.sub_val_val(0x80, 0xFF, true);
    assert_eq!(result, 0x80);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(true, cpu.flag(Flags::Sign));
    assert_eq!(false, cpu.flag(Flags::HalfCarry));
    assert_eq!(false, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));

    flag!(cpu; set Carry);
    let result = cpu.sub_val_val(0x00, 0x80, true);
    assert_eq!(result, 0x7F);
    assert_eq!(false, cpu.flag(Flags::OverflowParity));
    assert_eq!(false, cpu.flag(Flags::Zero));
    assert_eq!(false, cpu.flag(Flags::Sign));
    assert_eq!(true, cpu.flag(Flags::HalfCarry));
    assert_eq!(true, cpu.flag(Flags::Carry));
    assert_eq!(true, cpu.flag(Flags::Subtract));
}

#[test]
fn test_sbc_a_reg() {
    use super::Flags::*;
    use super::RegisterCode::*;

    let mut cpu: Cpu = vec![].into();

    // test wrap around from carry
    cpu.set_reg_value(A, 0x00);
    cpu.set_reg_value(B, 0x00);
    flag!(cpu; set Carry);
    cpu.sub_a_reg_carry(B);
    assert_eq!(0xFF, cpu.reg_value(A));
    assert_eq!(false, cpu.flag(OverflowParity));
    assert_eq!(false, cpu.flag(Zero));
    assert_eq!(true, cpu.flag(Sign));
    assert_eq!(true, cpu.flag(HalfCarry));
    assert_eq!(true, cpu.flag(Carry));
    assert_eq!(true, cpu.flag(Subtract));

    // test a - b - carry == 0
    cpu.set_reg_value(A, 0xFF);
    cpu.set_reg_value(B, 0xFE);
    flag!(cpu; set Carry);
    cpu.sub_a_reg_carry(B);
    assert_eq!(0x00, cpu.reg_value(A));
    assert_eq!(false, cpu.flag(OverflowParity));
    assert_eq!(true, cpu.flag(Zero));
    assert_eq!(false, cpu.flag(Sign));
    assert_eq!(false, cpu.flag(HalfCarry));
    assert_eq!(false, cpu.flag(Carry));
    assert_eq!(true, cpu.flag(Subtract));

    // test a - b == 0
    cpu.set_reg_value(A, 0x50);
    cpu.set_reg_value(B, 0x50);
    flag!(cpu; set Carry);
    cpu.sub_a_reg_carry(B);
    assert_eq!(0xFF, cpu.reg_value(A));
    assert_eq!(false, cpu.flag(OverflowParity));
    assert_eq!(false, cpu.flag(Zero));
    assert_eq!(true, cpu.flag(Sign));
    assert_eq!(true, cpu.flag(HalfCarry));
    assert_eq!(true, cpu.flag(Carry));
    assert_eq!(true, cpu.flag(Subtract));

    // test 0 - (-1) - 1 == 0
    cpu.set_reg_value(A, 0x00);
    cpu.set_reg_value(B, 0xFF);
    flag!(cpu; set Carry);
    cpu.sub_a_reg_carry(B);
    assert_eq!(0x00, cpu.reg_value(A));
    assert_eq!(false, cpu.flag(OverflowParity));
    assert_eq!(true, cpu.flag(Zero));
    assert_eq!(false, cpu.flag(Sign));
    assert_eq!(false, cpu.flag(HalfCarry));
    assert_eq!(false, cpu.flag(Carry));
    assert_eq!(true, cpu.flag(Subtract));

    // test val - (-1) - 1 == 0
    cpu.set_reg_value(A, 0x50);
    cpu.set_reg_value(B, 0xFF);
    flag!(cpu; set Carry);
    cpu.sub_a_reg_carry(B);
    assert_eq!(0x50, cpu.reg_value(A));
    assert_eq!(false, cpu.flag(OverflowParity));
    assert_eq!(false, cpu.flag(Zero));
    assert_eq!(false, cpu.flag(Sign));
    assert_eq!(false, cpu.flag(HalfCarry));
    assert_eq!(false, cpu.flag(Carry));
    assert_eq!(true, cpu.flag(Subtract));
}

#[test]
fn test_sbc_reg16_reg16() {
    let mut cpu = get_cpu();

    cpu.set_flag(Flags::Carry, true);
}
