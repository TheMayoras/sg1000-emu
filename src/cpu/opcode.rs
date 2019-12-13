extern crate num;

use super::Cpu;
use super::RegisterCode;
use super::RegisterCode16;
use std::collections::HashMap;

pub type OpcodeMap = HashMap<u8, Opcode>;

#[repr(u8)]
#[derive(FromPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
    NoOp = 0x00,

    LdBB = 0x40,
    LdBC,
    LdBD,
    LdBE,
    LdBH,
    LdBL,
    LdBA = 0x47,
    // ld B, literal
    LdBLit = 0x06,
    // ld B, (HL)
    LdBHLptr = 0x46,

    // ld D, reg
    LdDB = 0x50,
    LdDC,
    LdDD,
    LdDE,
    LdDH,
    LdDL,
    LdDA = 0x57,
    // ld D, literal
    LdDLit = 0x16,
    // ld B, (HL)
    LdDHLptr = 0x56,

    // ld H, reg
    LdHB = 0x60,
    LdHC,
    LdHD,
    LdHE,
    LdHH,
    LdHL,
    LdHA = 0x67,
    // ld H, literal
    LdHLit = 0x26,
    // ld H, (HL)
    LdHHLptr = 0x66,

    // ld C, reg
    LdCB = 0x48,
    LdCC,
    LdCD,
    LdCE,
    LdCH,
    LdCL,
    LdCA = 0x4F,
    // ld C, literal
    LdCLit = 0x0E,
    // ld C, (HL)
    LdCHLptr = 0x4E,

    // ld E, reg
    LdEB = 0x58,
    LdEC,
    LdED,
    LdEE,
    LdEH,
    LdEL,
    LdEA = 0x5F,
    // ld E, literal
    LdELit = 0x1E,
    // ld E, (HL)
    LdEHLptr = 0x5E,

    // ld L, reg
    LdLB = 0x68,
    LdLC,
    LdLD,
    LdLE,
    LdLH,
    LdLL,
    LdLA = 0x6F,
    // ld L, literal
    LdLLit = 0x2E,
    // ld L, (HL)
    LdLHLptr = 0x6E,

    // ld A, reg
    LdAB = 0x78,
    LdAC,
    LdAD,
    LdAE,
    LdAH,
    LdAL,
    LdAA = 0x7F,
    // ld A, literal
    LdALit = 0x3E,
    // ld A, (16 bit reg)
    LdAHLptr = 0x7E,
    LdABCptr = 0x0A,
    LdADEptr = 0x1A,

    // Extended Opcodes
    Ix = 0xDD,
    Iy = 0xFD,
    Bits = 0xCB,
    Extd = 0xED,

    // ld (hl), reg
    LdHLptrB = 0x70,
    LdHLptrC,
    LdHLptrD,
    LdHLptrE,
    LdHLptrH,
    LdHLptrL,
    LdHLptrA = 0x77,

    // ld (HL), literal
    LdHlptrLit = 0x36,

    // ld (Literal), Reg
    LdLitptrH = 0x22,
    LdLitptrA = 0x32,

    // ld (16 bit pair), reg
    LdBCptrA = 0x02,
    LdDEptrA = 0x12,

    // ld (16 bit reg), literal
    LdBCLit = 0x01,
    LdDELit = 0x11,
    LdHLLit = 0x21,
    LdSpLit = 0x31,
    LdSpHL = 0xF9,

    IncB = 0x04,
    IncD = 0x14,
    IncH = 0x24,
    IncC = 0x0C,
    IncE = 0x1C,
    IncL = 0x2C,
    IncA = 0x3C,

    // Inc (HL)
    IncHLptr = 0x34,

    // Inc (16 bit pair)
    IncBC = 0x03,
    IncDE = 0x13,
    IncHL = 0x23,
    IncSP = 0x33,

    // Dec reg
    DecB = 0x05,
    DecD = 0x15,
    DecH = 0x25,
    DecC = 0x0D,
    DecE = 0x1D,
    DecL = 0x2D,
    DecA = 0x3D,

    // Dec (HL)
    DecHLptr = 0x035,

    // Dec (16 bit pair)
    DecBC = 0x0B,
    DecDE = 0x1B,
    DecHL = 0x2B,
    DecSP = 0x3B,

    // ADD Acc, Reg
    AddAB = 0x80,
    AddAC,
    AddAD,
    AddAE,
    AddAH,
    AddAL,
    AddAA = 0x87,

    // ADD Acc, (HL)
    AddAHLptr = 0x86,

    // ADD Acc, Literal
    AddALit = 0xC6,

    // ADD Acc, Reg
    AdcAB = 0x88,
    AdcAC,
    AdcAD,
    AdcAE,
    AdcAH,
    AdcAL,
    AdcAA = 0x8F,

    // ADD Acc, (HL)
    AdcAHLptr = 0x8E,

    // ADD Acc, Literal
    AdcALit = 0xCE,

    // Sub Acc, Reg
    SubAB = 0x90,
    SubAC,
    SubAD,
    SubAE,
    SubAH,
    SubAL,
    SubAA = 0x97,

    // Sub Acc, (HL)
    SubAHLptr = 0x96,

    // Sub Acc, Literal
    SubALit = 0xD6,

    // Sub Acc, Reg
    SubcAB = 0x98,
    SubcAC,
    SubcAD,
    SubcAE,
    SubcAH,
    SubcAL,
    SubcAA = 0x9F,

    // Sub Acc, (HL)
    SubcAHLptr = 0x9E,

    // Sub Acc, Literal
    SubcALit = 0xDE,

    // AND A, reg
    AndAB = 0xA0,
    AndAC,
    AndAD,
    AndAE,
    AndAH,
    AndAL = 0xA5,
    AndAA = 0xA7,
    // AND A, (HL)
    AndAHLptr = 0xA6,
    // AND A, Lit
    AndALit = 0xE6,

    // Or A, reg
    OrAB = 0xB0,
    OrAC,
    OrAD,
    OrAE,
    OrAH,
    OrAL = 0xB5,
    OrAA = 0xB7,
    // Or A, (HL)
    OrAHLptr = 0xB6,
    // Or A, Lit
    OrALit = 0xF6,
}

impl Opcode {
    pub fn from_u8(value: u8) -> Opcode {
        num::FromPrimitive::from_u8(value).unwrap()
    }

    pub fn operate_u8(cpu: &mut Cpu, value: u8) {
        Opcode::operate(cpu, Opcode::from_u8(value));
    }

    pub fn operate(cpu: &mut Cpu, opcode: Opcode) {
        match opcode {
            // ld Reg, Reg
            Opcode::LdBB => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::B),
            Opcode::LdBC => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::C),
            Opcode::LdBD => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::D),
            Opcode::LdBE => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::E),
            Opcode::LdBH => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::H),
            Opcode::LdBL => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::L),
            Opcode::LdBA => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::A),
            Opcode::LdDB => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::B),
            Opcode::LdDC => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::C),
            Opcode::LdDD => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::D),
            Opcode::LdDE => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::E),
            Opcode::LdDH => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::H),
            Opcode::LdDL => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::L),
            Opcode::LdDA => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::A),
            Opcode::LdHB => cpu.ld_reg_reg(RegisterCode::H, RegisterCode::B),
            Opcode::LdHC => cpu.ld_reg_reg(RegisterCode::H, RegisterCode::C),
            Opcode::LdHD => cpu.ld_reg_reg(RegisterCode::H, RegisterCode::D),
            Opcode::LdHE => cpu.ld_reg_reg(RegisterCode::H, RegisterCode::E),
            Opcode::LdHH => cpu.ld_reg_reg(RegisterCode::H, RegisterCode::H),
            Opcode::LdHL => cpu.ld_reg_reg(RegisterCode::H, RegisterCode::L),
            Opcode::LdHA => cpu.ld_reg_reg(RegisterCode::H, RegisterCode::A),
            Opcode::LdCB => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::B),
            Opcode::LdCC => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::C),
            Opcode::LdCD => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::D),
            Opcode::LdCE => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::E),
            Opcode::LdCH => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::H),
            Opcode::LdCL => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::L),
            Opcode::LdCA => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::A),
            Opcode::LdEB => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::B),
            Opcode::LdEC => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::C),
            Opcode::LdED => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::D),
            Opcode::LdEE => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::E),
            Opcode::LdEH => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::H),
            Opcode::LdEL => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::L),
            Opcode::LdEA => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::A),
            Opcode::LdLB => cpu.ld_reg_reg(RegisterCode::L, RegisterCode::B),
            Opcode::LdLC => cpu.ld_reg_reg(RegisterCode::L, RegisterCode::C),
            Opcode::LdLD => cpu.ld_reg_reg(RegisterCode::L, RegisterCode::D),
            Opcode::LdLE => cpu.ld_reg_reg(RegisterCode::L, RegisterCode::E),
            Opcode::LdLH => cpu.ld_reg_reg(RegisterCode::L, RegisterCode::H),
            Opcode::LdLL => cpu.ld_reg_reg(RegisterCode::L, RegisterCode::L),
            Opcode::LdLA => cpu.ld_reg_reg(RegisterCode::L, RegisterCode::A),
            Opcode::LdAB => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::B),
            Opcode::LdAC => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::C),
            Opcode::LdAD => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::D),
            Opcode::LdAE => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::E),
            Opcode::LdAH => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::H),
            Opcode::LdAL => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::L),
            Opcode::LdAA => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::A),

            // Load Reg, Literal
            Opcode::LdBLit => cpu.ld_reg_lit(RegisterCode::C),
            Opcode::LdDLit => cpu.ld_reg_lit(RegisterCode::D),
            Opcode::LdHLit => cpu.ld_reg_lit(RegisterCode::H),
            Opcode::LdCLit => cpu.ld_reg_lit(RegisterCode::C),
            Opcode::LdELit => cpu.ld_reg_lit(RegisterCode::E),
            Opcode::LdLLit => cpu.ld_reg_lit(RegisterCode::L),
            Opcode::LdALit => cpu.ld_reg_lit(RegisterCode::A),

            // Load Reg, (16 Bit Pair)
            Opcode::LdBHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_reg_addr(RegisterCode::B, addr)
            }
            Opcode::LdDHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_reg_addr(RegisterCode::D, addr)
            }
            Opcode::LdHHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_reg_addr(RegisterCode::H, addr)
            }
            Opcode::LdCHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_reg_addr(RegisterCode::C, addr)
            }
            Opcode::LdEHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_reg_addr(RegisterCode::E, addr)
            }
            Opcode::LdLHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_reg_addr(RegisterCode::L, addr)
            }
            Opcode::LdAHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_reg_addr(RegisterCode::A, addr)
            }
            Opcode::LdABCptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::BC);
                cpu.ld_reg_addr(RegisterCode::A, addr)
            }
            Opcode::LdADEptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::DE);
                cpu.ld_reg_addr(RegisterCode::A, addr)
            }

            // Load (HL), Reg
            Opcode::LdHLptrB => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_addr_reg(addr, RegisterCode::B);
            }
            Opcode::LdHLptrC => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_addr_reg(addr, RegisterCode::C);
            }
            Opcode::LdHLptrD => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_addr_reg(addr, RegisterCode::D);
            }
            Opcode::LdHLptrE => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_addr_reg(addr, RegisterCode::E);
            }
            Opcode::LdHLptrH => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_addr_reg(addr, RegisterCode::H);
            }
            Opcode::LdHLptrL => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_addr_reg(addr, RegisterCode::L);
            }
            Opcode::LdHLptrA => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_addr_reg(addr, RegisterCode::A);
            }

            // Ld (HL), literal
            Opcode::LdHlptrLit => {
                let val = cpu.imm_addr();
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.ld_addr_lit(addr, val);
            }

            // ld (literal), Reg
            Opcode::LdLitptrH => {
                let addr = cpu.imm_addr_ex();
                cpu.ld_addr_reg(addr, RegisterCode::H);
            }

            Opcode::LdLitptrA => {
                let addr = cpu.imm_addr_ex();
                cpu.ld_addr_reg(addr, RegisterCode::A);
            }

            // ld (16 bit pair), reg
            Opcode::LdBCptrA => {
                let addr = cpu.reg_value_16(RegisterCode16::BC);
                cpu.ld_addr_reg(addr, RegisterCode::A);
            }
            Opcode::LdDEptrA => {
                let addr = cpu.reg_value_16(RegisterCode16::DE);
                cpu.ld_addr_reg(addr, RegisterCode::A);
            }

            Opcode::LdBCLit => {
                let val = cpu.imm_addr_ex();
                cpu.ld_reg16_lit(RegisterCode16::BC, val);
            }

            Opcode::LdDELit => {
                let val = cpu.imm_addr_ex();
                cpu.ld_reg16_lit(RegisterCode16::DE, val);
            }

            Opcode::LdHLLit => {
                let val = cpu.imm_addr_ex();
                cpu.ld_reg16_lit(RegisterCode16::HL, val);
            }

            Opcode::LdSpLit => {
                let val = cpu.imm_addr_ex();
                cpu.ld_reg16_lit(RegisterCode16::SP, val);
            }

            Opcode::LdSpHL => {
                cpu.ld_reg16_reg16(RegisterCode16::SP, RegisterCode16::HL);
            }

            /* ------------- inc Reg ------------- */
            Opcode::IncB => cpu.inc_reg(RegisterCode::B),
            Opcode::IncD => cpu.inc_reg(RegisterCode::D),
            Opcode::IncH => cpu.inc_reg(RegisterCode::H),
            Opcode::IncC => cpu.inc_reg(RegisterCode::C),
            Opcode::IncE => cpu.inc_reg(RegisterCode::E),
            Opcode::IncL => cpu.inc_reg(RegisterCode::L),
            Opcode::IncA => cpu.inc_reg(RegisterCode::A),

            Opcode::IncHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.inc_addr(addr);
            }

            Opcode::IncBC => cpu.inc_reg16(RegisterCode16::BC),
            Opcode::IncDE => cpu.inc_reg16(RegisterCode16::DE),
            Opcode::IncHL => cpu.inc_reg16(RegisterCode16::HL),
            Opcode::IncSP => cpu.inc_reg16(RegisterCode16::SP),

            Opcode::DecB => cpu.dec_reg(RegisterCode::B),
            Opcode::DecD => cpu.dec_reg(RegisterCode::D),
            Opcode::DecH => cpu.dec_reg(RegisterCode::H),
            Opcode::DecC => cpu.dec_reg(RegisterCode::C),
            Opcode::DecE => cpu.dec_reg(RegisterCode::E),
            Opcode::DecL => cpu.dec_reg(RegisterCode::L),
            Opcode::DecA => cpu.dec_reg(RegisterCode::A),

            Opcode::DecHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.dec_addr(addr);
            }

            Opcode::DecBC => cpu.dec_reg16(RegisterCode16::BC),
            Opcode::DecDE => cpu.dec_reg16(RegisterCode16::DE),
            Opcode::DecHL => cpu.dec_reg16(RegisterCode16::HL),
            Opcode::DecSP => cpu.dec_reg16(RegisterCode16::SP),

            Opcode::AddAB => cpu.add_a_reg(RegisterCode::B),
            Opcode::AddAC => cpu.add_a_reg(RegisterCode::C),
            Opcode::AddAD => cpu.add_a_reg(RegisterCode::D),
            Opcode::AddAE => cpu.add_a_reg(RegisterCode::E),
            Opcode::AddAH => cpu.add_a_reg(RegisterCode::H),
            Opcode::AddAL => cpu.add_a_reg(RegisterCode::L),
            Opcode::AddAA => cpu.add_a_reg(RegisterCode::A),
            Opcode::AddAHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.add_a_addr(addr);
            }

            Opcode::AddALit => {
                let lit = cpu.imm_addr();
                cpu.add_a_lit(lit);
            }

            // ADD Acc, Reg
            Opcode::AdcAB => cpu.add_a_reg_carry(RegisterCode::A),
            Opcode::AdcAC => cpu.add_a_reg_carry(RegisterCode::C),
            Opcode::AdcAD => cpu.add_a_reg_carry(RegisterCode::D),
            Opcode::AdcAE => cpu.add_a_reg_carry(RegisterCode::E),
            Opcode::AdcAH => cpu.add_a_reg_carry(RegisterCode::H),
            Opcode::AdcAL => cpu.add_a_reg_carry(RegisterCode::L),
            Opcode::AdcAA => cpu.add_a_reg_carry(RegisterCode::A),
            Opcode::AdcAHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.add_a_addr_carry(addr);
            }
            Opcode::AdcALit => {
                let val = cpu.imm_addr();
                cpu.add_a_lit_carry(val);
            }
            Opcode::SubAB => cpu.sub_a_reg(RegisterCode::B),
            Opcode::SubAC => cpu.sub_a_reg(RegisterCode::C),
            Opcode::SubAD => cpu.sub_a_reg(RegisterCode::D),
            Opcode::SubAE => cpu.sub_a_reg(RegisterCode::E),
            Opcode::SubAH => cpu.sub_a_reg(RegisterCode::H),
            Opcode::SubAL => cpu.sub_a_reg(RegisterCode::L),
            Opcode::SubAA => cpu.sub_a_reg(RegisterCode::A),
            Opcode::SubAHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.sub_a_addr(addr);
            }

            Opcode::SubALit => {
                let lit = cpu.imm_addr();
                cpu.sub_a_lit(lit);
            }

            Opcode::SubcAB => cpu.sub_a_reg_carry(RegisterCode::A),
            Opcode::SubcAC => cpu.sub_a_reg_carry(RegisterCode::C),
            Opcode::SubcAD => cpu.sub_a_reg_carry(RegisterCode::D),
            Opcode::SubcAE => cpu.sub_a_reg_carry(RegisterCode::E),
            Opcode::SubcAH => cpu.sub_a_reg_carry(RegisterCode::H),
            Opcode::SubcAL => cpu.sub_a_reg_carry(RegisterCode::L),
            Opcode::SubcAA => cpu.sub_a_reg_carry(RegisterCode::A),
            Opcode::SubcAHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.sub_a_addr_carry(addr);
            }
            Opcode::SubcALit => {
                let val = cpu.imm_addr();
                cpu.sub_a_lit_carry(val);
            }
            Opcode::AndAB => cpu.and_a_reg(RegisterCode::B),
            Opcode::AndAC => cpu.and_a_reg(RegisterCode::C),
            Opcode::AndAD => cpu.and_a_reg(RegisterCode::D),
            Opcode::AndAE => cpu.and_a_reg(RegisterCode::E),
            Opcode::AndAH => cpu.and_a_reg(RegisterCode::H),
            Opcode::AndAL => cpu.and_a_reg(RegisterCode::L),
            Opcode::AndAA => cpu.and_a_reg(RegisterCode::A),
            Opcode::AndAHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.and_a_addr(addr);
            }
            Opcode::AndALit => {
                let val = cpu.imm_addr();
                cpu.and_a_lit(val);
            }

            Opcode::OrAB => cpu.or_a_reg(RegisterCode::B),
            Opcode::OrAC => cpu.or_a_reg(RegisterCode::C),
            Opcode::OrAD => cpu.or_a_reg(RegisterCode::D),
            Opcode::OrAE => cpu.or_a_reg(RegisterCode::E),
            Opcode::OrAH => cpu.or_a_reg(RegisterCode::H),
            Opcode::OrAL => cpu.or_a_reg(RegisterCode::L),
            Opcode::OrAA => cpu.or_a_reg(RegisterCode::A),
            Opcode::OrAHLptr => {
                let addr = cpu.reg_indirect_addr(RegisterCode16::HL);
                cpu.or_a_addr(addr);
            }
            Opcode::OrALit => {
                let val = cpu.imm_addr();
                cpu.or_a_lit(val);
            }
            // Extended Opcodes
            Opcode::Ix => panic!("Unimplemented!"),
            Opcode::Iy => panic!("Unimplemented!"),
            Opcode::Bits => panic!("Unimplemented!"),
            Opcode::Extd => panic!("Unimplemented!"),
            _ => panic!("Unimplemented!"),
        }
    }
}
