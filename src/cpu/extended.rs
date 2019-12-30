extern crate num;

use super::{Cpu, RegisterCode16};

#[repr(u8)]
#[derive(FromPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Extnd {
    SbcHLBC = 0x42,
    SbcHLDE = 0x52,
    SbcHLHL = 0x62,
    SbcHLSP = 0x72,
}

impl Extnd {
    pub fn from_u8(value: u8) -> Extnd {
        num::FromPrimitive::from_u8(value).unwrap()
    }

    pub fn operate_u8(cpu: &mut Cpu, value: u8) {
        Extnd::operate(cpu, Extnd::from_u8(value));
    }

    pub fn operate(cpu: &mut Cpu, opcode: Extnd) {
        use Extnd::*;
        println!("Found Extnd Opcode: {:?}", opcode);
        match opcode {
            SbcHLBC => cpu.sbc_reg16_reg16(RegisterCode16::HL, RegisterCode16::BC),
            SbcHLDE => cpu.sbc_reg16_reg16(RegisterCode16::HL, RegisterCode16::DE),
            SbcHLHL => cpu.sbc_reg16_reg16(RegisterCode16::HL, RegisterCode16::HL),
            SbcHLSP => cpu.sbc_reg16_reg16(RegisterCode16::HL, RegisterCode16::SP),
        }
    }
}
