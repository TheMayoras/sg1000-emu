extern crate num;

use crate::cpu::{Cpu, RegisterCode, RegisterCode16};

#[repr(u8)]
#[derive(FromPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
pub enum Extnd {
    SbcHLBC = 0x42,
    SbcHLDE = 0x52,
    SbcHLHL = 0x62,
    SbcHLSP = 0x72,

    LdLitBC = 0x43,
    LdLitDE = 0x53,
    LdLitHL = 0x63,
    LdLitSP = 0x73,

    Neg0 = 0x44,
    Neg1 = 0x54,
    Neg2 = 0x64,
    Neg3 = 0x74,

    Retn0 = 0x45,
    Retn1 = 0x55,
    Retn2 = 0x65,
    Retn3 = 0x75,
    Retn4 = 0x5D,
    Retn5 = 0x6D,
    Retn6 = 0x7D,

    Reti = 0x4D,

    // Interupt Modes
    // NOTE: SG-1000 only supports interrupt mode 1
    Im00 = 0x46,
    Im01 = 0x66,

    Im10 = 0x56,
    Im11 = 0x76,

    LdAI = 0x57,
    LdIA = 0x47,
    LdAR = 0x5F,
    LdRA = 0x4F,

    RRD = 0x67,
    RLD = 0x6F,

    AdcHLBC = 0x4A,
    AdcHLDE = 0x5A,
    AdcHLHL = 0x6A,
    AdcHLSP = 0x7A,

    LdBCLit = 0x4B,
    LdDELit = 0x5B,
    LdHLLit = 0x6B,
    LdSPLit = 0x7B,

    Ldi = 0xA0,
    Ldir = 0xB0,

    Ldd = 0xA8,
    Lddr = 0xB8,

    Cpi = 0xA1,
    Cpir = 0xB1,

    Cpd = 0xA9,
    Cpdr = 0xB9,

    // Out functions
    OutCB = 0x41,
    OutCD = 0x51,
    OutCH = 0x61,
    OutC0 = 0x71,
    OutCC = 0x49,
    OutCE = 0x59,
    OutCL = 0x69,
    OutCA = 0x79,

    OutI = 0xA3,
    OutD = 0xAB,
    OutIR = 0xB3,
    OutDR = 0xBB,

    // In Functions
    InCB = 0x40,
    InCD = 0x50,
    InCH = 0x60,
    InC0 = 0x70,
    InCC = 0x48,
    InCE = 0x58,
    InCL = 0x68,
    InCA = 0x78,
    InI = 0xA2,
    InD = 0xAA,
    InIR = 0xB2,
    InDR = 0xBA,
}

impl Extnd {
    pub fn from_u8(value: u8) -> Extnd {
        //println!("Getting Extended for: 0x{:x} | ", value);
        let op = num::FromPrimitive::from_u8(value).unwrap();
        //println!("Opcode: {:?}", op);
        op
    }

    pub fn operate_u8(cpu: &mut Cpu, value: u8) {
        Extnd::operate(cpu, Extnd::from_u8(value));
    }

    pub fn operate(cpu: &mut Cpu, opcode: Extnd) {
        use Extnd::*;
        // println!("Found Extnd Opcode: {:?}", opcode);
        match opcode {
            SbcHLBC => cpu.sbc_reg16_reg16(RegisterCode16::HL, RegisterCode16::BC),
            SbcHLDE => cpu.sbc_reg16_reg16(RegisterCode16::HL, RegisterCode16::DE),
            SbcHLHL => cpu.sbc_reg16_reg16(RegisterCode16::HL, RegisterCode16::HL),
            SbcHLSP => cpu.sbc_reg16_reg16(RegisterCode16::HL, RegisterCode16::SP),

            LdLitBC => {
                cpu.queue_clock_tick(4);
                let addr = cpu.imm_addr_ex();
                cpu.ld_addr_reg16(addr, RegisterCode16::BC);
            }
            LdLitDE => {
                cpu.queue_clock_tick(4);
                let addr = cpu.imm_addr_ex();
                cpu.ld_addr_reg16(addr, RegisterCode16::DE);
            }
            LdLitHL => {
                cpu.queue_clock_tick(4);
                let addr = cpu.imm_addr_ex();
                cpu.ld_addr_reg16(addr, RegisterCode16::HL);
            }
            LdLitSP => {
                cpu.queue_clock_tick(4);
                let addr = cpu.imm_addr_ex();
                cpu.ld_addr_reg16(addr, RegisterCode16::SP);
            }

            Neg0 => cpu.neg(),
            Neg1 => cpu.neg(),
            Neg2 => cpu.neg(),
            Neg3 => cpu.neg(),

            Retn0 => cpu.retn(),
            Retn1 => cpu.retn(),
            Retn2 => cpu.retn(),
            Retn3 => cpu.retn(),
            Retn4 => cpu.retn(),
            Retn5 => cpu.retn(),
            Retn6 => cpu.retn(),

            Im00 => cpu.interrupt_0(),
            Im01 => cpu.interrupt_0(),

            Im10 => cpu.interrupt_1(),
            Im11 => cpu.interrupt_1(),

            LdAI => {
                cpu.queue_clock_tick(5);
                cpu.ld_reg_reg(RegisterCode::A, RegisterCode::I);
            }
            LdIA => {
                cpu.queue_clock_tick(5);
                cpu.ld_reg_reg(RegisterCode::I, RegisterCode::A);
            }
            LdAR => {
                cpu.queue_clock_tick(5);
                cpu.ld_reg_reg(RegisterCode::A, RegisterCode::R);
            }
            LdRA => {
                cpu.queue_clock_tick(5);
                cpu.ld_reg_reg(RegisterCode::R, RegisterCode::A);
            }

            RRD => cpu.rrd(),
            RLD => cpu.rld(),

            AdcHLBC => cpu.adc_reg16_reg16(RegisterCode16::HL, RegisterCode16::BC),
            AdcHLDE => cpu.adc_reg16_reg16(RegisterCode16::HL, RegisterCode16::DE),
            AdcHLHL => cpu.adc_reg16_reg16(RegisterCode16::HL, RegisterCode16::HL),
            AdcHLSP => cpu.adc_reg16_reg16(RegisterCode16::HL, RegisterCode16::SP),

            LdBCLit => {
                let val = cpu.imm_addr_ex();
                cpu.ld_reg16_lit(RegisterCode16::BC, val);
            }
            LdDELit => {
                let val = cpu.imm_addr_ex();
                cpu.ld_reg16_lit(RegisterCode16::BC, val);
            }
            LdHLLit => {
                let val = cpu.imm_addr_ex();
                cpu.ld_reg16_lit(RegisterCode16::BC, val);
            }
            LdSPLit => {
                let val = cpu.imm_addr_ex();
                cpu.ld_reg16_lit(RegisterCode16::BC, val);
            }

            Reti => cpu.reti(),

            Ldi => cpu.ld_id(true),
            Ldir => cpu.ld_id_r(true),

            Ldd => cpu.ld_id(false),
            Lddr => cpu.ld_id_r(false),

            Cpi => cpu.cp_id(true),
            Cpir => cpu.cp_id_r(true),

            Cpd => cpu.cp_id(false),
            Cpdr => cpu.cp_id_r(false),

            OutCB => cpu.out_c_reg(Some(RegisterCode::B)),
            OutCD => cpu.out_c_reg(Some(RegisterCode::D)),
            OutCH => cpu.out_c_reg(Some(RegisterCode::H)),
            OutC0 => cpu.out_c_reg(None),
            OutCC => cpu.out_c_reg(Some(RegisterCode::C)),
            OutCE => cpu.out_c_reg(Some(RegisterCode::E)),
            OutCL => cpu.out_c_reg(Some(RegisterCode::L)),
            OutCA => cpu.out_c_reg(Some(RegisterCode::A)),

            OutI => cpu.out_id(true),
            OutD => cpu.out_id(false),
            OutIR => cpu.out_id_rep(true),
            OutDR => cpu.out_id_rep(false),

            InCB => cpu.in_reg_c(Some(RegisterCode::B)),
            InCD => cpu.in_reg_c(Some(RegisterCode::D)),
            InCH => cpu.in_reg_c(Some(RegisterCode::H)),
            InC0 => cpu.in_reg_c(None),
            InCC => cpu.in_reg_c(Some(RegisterCode::C)),
            InCE => cpu.in_reg_c(Some(RegisterCode::E)),
            InCL => cpu.in_reg_c(Some(RegisterCode::L)),
            InCA => cpu.in_reg_c(Some(RegisterCode::A)),

            InI => cpu.in_id(true),
            InD => cpu.in_id(false),
            InIR => cpu.in_id_rep(true),
            InDR => cpu.in_id_rep(false),
        }
    }
}
