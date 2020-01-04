extern crate num;

use super::{
    bits::BitsOpcode, extended::Extnd, BitsOperator, BitsOperatorDefault, Cpu, IndexedBitsOperator,
    RegisterCode, RegisterCode16,
};

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

    // 16 bit Add
    AddHLBC = 0x09,
    AddHLDE = 0x19,
    AddHLHL = 0x29,
    AddHLSP = 0x39,

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

    // Xor A, reg
    XorAB = 0xA8,
    XorAC,
    XorAD,
    XorAE,
    XorAH,
    XorAL,
    XorAA = 0xAF,
    // Xor A, (HL)
    XorAHLptr = 0xAE,
    // Xor A, Lit
    XorALit = 0xEE,

    // cp A, reg
    CpAB = 0xB8,
    CpAC,
    CpAD,
    CpAE,
    CpAH,
    CpAL,
    CpAA = 0xBF,
    // Cp A, (HL)
    CpAHLptr = 0xBE,
    // Cp A, Lit
    CpALit = 0xFE,

    Rlca = 0x07,
    Rla = 0x17,

    Rrca = 0x0F,
    Rra = 0x1F,

    // JP Instructions
    JpLit = 0xC3,
    JpNzLit = 0xC2,
    JpNcLit = 0xD2,
    JpPoLit = 0xE2,
    JpPLit = 0xF2,
    JpZLit = 0xCA,
    JpCLit = 0xDA,
    JpPeLit = 0xEA,
    JpMLit = 0xFA,

    // Relative Jump Instructions
    JrLit = 0x18,
    JrCLit = 0x38,
    JrNcLit = 0x30,
    JrZLit = 0x28,
    JrNzLit = 0x20,

    JpHLptr = 0xE9,
    DJNz = 0x10,

    // Ex opcodes
    ExAfAf = 0x08,
    ExSPptrHL = 0xE3,
    Exx = 0xD9,
    ExDEHL = 0xEB,

    // Interrupts
    Ei = 0xFB,
    Di = 0xF3,

    Daa = 0x27,

    // Carry flag set and reset
    Scf = 0x37,
    Ccf = 0x3F,

    // Call instructions
    CallLit = 0xCD,
    CallNz = 0xC4,
    CallNc = 0xD4,
    CallPo = 0xE4,
    CallP = 0xF4,
    CallZ = 0xCC,
    CallC = 0xDC,
    CallPe = 0xEC,
    CallM = 0xFC,

    // Return Instructions
    Ret = 0xC9,
    RetNz = 0xC0,
    RetNc = 0xD0,
    RetPo = 0xE0,
    RetP = 0xF0,
    RetZ = 0xC8,
    RetC = 0xD8,
    RetPe = 0xE8,
    RetM = 0xF8,

    // Halt
    Halt = 0x76,

    // RST
    Rst00 = 0xC7,
    Rst10 = 0xD7,
    Rst20 = 0xE7,
    Rst30 = 0xF7,
    Rst08 = 0xCF,
    Rst18 = 0xDF,
    Rst28 = 0xEF,
    Rst38 = 0xFF,
}

impl Opcode {
    pub fn from_u8(value: u8) -> Opcode {
        num::FromPrimitive::from_u8(value).unwrap()
    }

    pub fn operate_u8(cpu: &mut Cpu, value: u8) {
        Opcode::operate(cpu, Opcode::from_u8(value));
    }

    pub fn operate(cpu: &mut Cpu, opcode: Opcode) {
        Opcode::operate_with(
            cpu,
            opcode,
            RegisterCode16::HL,
            RegisterCode::H,
            RegisterCode::L,
            Opcode::rel_addr(RegisterCode16::HL),
            BitsOperatorDefault {},
        );
    }

    fn rel_addr(reg: RegisterCode16) -> impl FnMut(&mut Cpu) -> u16 {
        move |cpu| cpu.indirect_reg_addr(reg)
    }

    fn index_addr<F>(reg: RegisterCode16) -> impl FnMut(&mut Cpu) -> u16 {
        move |cpu| cpu.index_addr(reg)
    }

    fn operate_with<T, U>(
        cpu: &mut Cpu,
        opcode: Opcode,
        reg: RegisterCode16,
        upper: RegisterCode,
        lower: RegisterCode,
        mut pointer: T,
        bits_op: U,
    ) where
        T: FnMut(&mut Cpu) -> u16,
        U: BitsOperator,
    {
        use super::Flags;
        println!("Found opcode: {:?}", opcode);
        match opcode {
            Opcode::NoOp => {}

            // ld Reg, Reg
            Opcode::LdBB => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::B),
            Opcode::LdBC => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::C),
            Opcode::LdBD => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::D),
            Opcode::LdBE => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::E),
            Opcode::LdBH => cpu.ld_reg_reg(RegisterCode::B, upper),
            Opcode::LdBL => cpu.ld_reg_reg(RegisterCode::B, lower),
            Opcode::LdBA => cpu.ld_reg_reg(RegisterCode::B, RegisterCode::A),
            Opcode::LdDB => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::B),
            Opcode::LdDC => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::C),
            Opcode::LdDD => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::D),
            Opcode::LdDE => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::E),
            Opcode::LdDH => cpu.ld_reg_reg(RegisterCode::D, upper),
            Opcode::LdDL => cpu.ld_reg_reg(RegisterCode::D, lower),
            Opcode::LdDA => cpu.ld_reg_reg(RegisterCode::D, RegisterCode::A),
            Opcode::LdHB => cpu.ld_reg_reg(upper, RegisterCode::B),
            Opcode::LdHC => cpu.ld_reg_reg(upper, RegisterCode::C),
            Opcode::LdHD => cpu.ld_reg_reg(upper, RegisterCode::D),
            Opcode::LdHE => cpu.ld_reg_reg(upper, RegisterCode::E),
            Opcode::LdHH => cpu.ld_reg_reg(upper, RegisterCode::H),
            Opcode::LdHL => cpu.ld_reg_reg(upper, lower),
            Opcode::LdHA => cpu.ld_reg_reg(upper, RegisterCode::A),
            Opcode::LdCB => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::B),
            Opcode::LdCC => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::C),
            Opcode::LdCD => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::D),
            Opcode::LdCE => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::E),
            Opcode::LdCH => cpu.ld_reg_reg(RegisterCode::C, upper),
            Opcode::LdCL => cpu.ld_reg_reg(RegisterCode::C, lower),
            Opcode::LdCA => cpu.ld_reg_reg(RegisterCode::C, RegisterCode::A),
            Opcode::LdEB => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::B),
            Opcode::LdEC => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::C),
            Opcode::LdED => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::D),
            Opcode::LdEE => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::E),
            Opcode::LdEH => cpu.ld_reg_reg(RegisterCode::E, upper),
            Opcode::LdEL => cpu.ld_reg_reg(RegisterCode::E, lower),
            Opcode::LdEA => cpu.ld_reg_reg(RegisterCode::E, RegisterCode::A),
            Opcode::LdLB => cpu.ld_reg_reg(lower, RegisterCode::B),
            Opcode::LdLC => cpu.ld_reg_reg(lower, RegisterCode::C),
            Opcode::LdLD => cpu.ld_reg_reg(lower, RegisterCode::D),
            Opcode::LdLE => cpu.ld_reg_reg(lower, RegisterCode::E),
            Opcode::LdLH => cpu.ld_reg_reg(lower, upper),
            Opcode::LdLL => cpu.ld_reg_reg(lower, RegisterCode::L),
            Opcode::LdLA => cpu.ld_reg_reg(lower, RegisterCode::A),
            Opcode::LdAB => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::B),
            Opcode::LdAC => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::C),
            Opcode::LdAD => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::D),
            Opcode::LdAE => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::E),
            Opcode::LdAH => cpu.ld_reg_reg(RegisterCode::A, upper),
            Opcode::LdAL => cpu.ld_reg_reg(RegisterCode::A, lower),
            Opcode::LdAA => cpu.ld_reg_reg(RegisterCode::A, RegisterCode::A),

            // Load Reg, Literal
            Opcode::LdBLit => cpu.ld_reg_lit(RegisterCode::B),
            Opcode::LdDLit => cpu.ld_reg_lit(RegisterCode::D),
            Opcode::LdHLit => cpu.ld_reg_lit(upper),
            Opcode::LdCLit => cpu.ld_reg_lit(RegisterCode::C),
            Opcode::LdELit => cpu.ld_reg_lit(RegisterCode::E),
            Opcode::LdLLit => cpu.ld_reg_lit(lower),
            Opcode::LdALit => cpu.ld_reg_lit(RegisterCode::A),

            // Load Reg, (16 Bit Pair)
            Opcode::LdBHLptr => {
                let addr = pointer(cpu);
                cpu.ld_reg_addr(RegisterCode::B, addr)
            }
            Opcode::LdDHLptr => {
                let addr = pointer(cpu);
                cpu.ld_reg_addr(RegisterCode::D, addr)
            }
            Opcode::LdHHLptr => {
                let addr = pointer(cpu);
                cpu.ld_reg_addr(upper, addr)
            }
            Opcode::LdCHLptr => {
                let addr = pointer(cpu);
                cpu.ld_reg_addr(RegisterCode::C, addr)
            }
            Opcode::LdEHLptr => {
                let addr = pointer(cpu);
                cpu.ld_reg_addr(RegisterCode::E, addr)
            }
            Opcode::LdLHLptr => {
                let addr = pointer(cpu);
                cpu.ld_reg_addr(lower, addr)
            }
            Opcode::LdAHLptr => {
                let addr = pointer(cpu);
                cpu.ld_reg_addr(RegisterCode::A, addr)
            }
            Opcode::LdABCptr => {
                let addr = cpu.indirect_reg_addr(RegisterCode16::BC);
                cpu.ld_reg_addr(RegisterCode::A, addr)
            }
            Opcode::LdADEptr => {
                let addr = cpu.indirect_reg_addr(RegisterCode16::DE);
                cpu.ld_reg_addr(RegisterCode::A, addr)
            }

            // Load (HL), Reg
            Opcode::LdHLptrB => {
                let addr = pointer(cpu);
                cpu.ld_addr_reg(addr, RegisterCode::B);
            }
            Opcode::LdHLptrC => {
                let addr = pointer(cpu);
                cpu.ld_addr_reg(addr, RegisterCode::C);
            }
            Opcode::LdHLptrD => {
                let addr = pointer(cpu);
                cpu.ld_addr_reg(addr, RegisterCode::D);
            }
            Opcode::LdHLptrE => {
                let addr = pointer(cpu);
                cpu.ld_addr_reg(addr, RegisterCode::E);
            }
            Opcode::LdHLptrH => {
                let addr = pointer(cpu);
                cpu.ld_addr_reg(addr, upper);
            }
            Opcode::LdHLptrL => {
                let addr = pointer(cpu);
                cpu.ld_addr_reg(addr, lower);
            }
            Opcode::LdHLptrA => {
                let addr = pointer(cpu);
                cpu.ld_addr_reg(addr, RegisterCode::A);
            }

            // Ld (HL), literal
            Opcode::LdHlptrLit => {
                let val = cpu.imm_addr();
                let addr = pointer(cpu);
                cpu.ld_addr_lit(addr, val);
            }

            // ld (literal), Reg
            Opcode::LdLitptrH => {
                let addr = cpu.imm_addr_ex();
                cpu.ld_addr_reg(addr, upper);
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
            Opcode::IncH => cpu.inc_reg(upper),
            Opcode::IncC => cpu.inc_reg(RegisterCode::C),
            Opcode::IncE => cpu.inc_reg(RegisterCode::E),
            Opcode::IncL => cpu.inc_reg(lower),
            Opcode::IncA => cpu.inc_reg(RegisterCode::A),

            Opcode::IncHLptr => {
                let addr = pointer(cpu);
                cpu.inc_addr(addr);
            }

            Opcode::IncBC => cpu.inc_reg16(RegisterCode16::BC),
            Opcode::IncDE => cpu.inc_reg16(RegisterCode16::DE),
            Opcode::IncHL => cpu.inc_reg16(RegisterCode16::HL),
            Opcode::IncSP => cpu.inc_reg16(RegisterCode16::SP),

            Opcode::DecB => cpu.dec_reg(RegisterCode::B),
            Opcode::DecD => cpu.dec_reg(RegisterCode::D),
            Opcode::DecH => cpu.dec_reg(upper),
            Opcode::DecC => cpu.dec_reg(RegisterCode::C),
            Opcode::DecE => cpu.dec_reg(RegisterCode::E),
            Opcode::DecL => cpu.dec_reg(lower),
            Opcode::DecA => cpu.dec_reg(RegisterCode::A),

            Opcode::DecHLptr => {
                let addr = pointer(cpu);
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
            Opcode::AddAH => cpu.add_a_reg(upper),
            Opcode::AddAL => cpu.add_a_reg(lower),
            Opcode::AddAA => cpu.add_a_reg(RegisterCode::A),
            Opcode::AddAHLptr => {
                let addr = pointer(cpu);
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
            Opcode::AdcAH => cpu.add_a_reg_carry(upper),
            Opcode::AdcAL => cpu.add_a_reg_carry(lower),
            Opcode::AdcAA => cpu.add_a_reg_carry(RegisterCode::A),
            Opcode::AdcAHLptr => {
                let addr = pointer(cpu);
                cpu.add_a_addr_carry(addr);
            }
            Opcode::AdcALit => {
                let val = cpu.imm_addr();
                cpu.add_a_lit_carry(val);
            }

            Opcode::AddHLBC => cpu.add_reg16_reg16(RegisterCode16::HL, RegisterCode16::BC),
            Opcode::AddHLDE => cpu.add_reg16_reg16(RegisterCode16::HL, RegisterCode16::DE),
            Opcode::AddHLHL => cpu.add_reg16_reg16(RegisterCode16::HL, RegisterCode16::HL),
            Opcode::AddHLSP => cpu.add_reg16_reg16(RegisterCode16::HL, RegisterCode16::SP),

            Opcode::SubAB => cpu.sub_a_reg(RegisterCode::B),
            Opcode::SubAC => cpu.sub_a_reg(RegisterCode::C),
            Opcode::SubAD => cpu.sub_a_reg(RegisterCode::D),
            Opcode::SubAE => cpu.sub_a_reg(RegisterCode::E),
            Opcode::SubAH => cpu.sub_a_reg(upper),
            Opcode::SubAL => cpu.sub_a_reg(lower),
            Opcode::SubAA => cpu.sub_a_reg(RegisterCode::A),
            Opcode::SubAHLptr => {
                let addr = pointer(cpu);
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
            Opcode::SubcAH => cpu.sub_a_reg_carry(upper),
            Opcode::SubcAL => cpu.sub_a_reg_carry(lower),
            Opcode::SubcAA => cpu.sub_a_reg_carry(RegisterCode::A),
            Opcode::SubcAHLptr => {
                let addr = pointer(cpu);
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
            Opcode::AndAH => cpu.and_a_reg(upper),
            Opcode::AndAL => cpu.and_a_reg(lower),
            Opcode::AndAA => cpu.and_a_reg(RegisterCode::A),
            Opcode::AndAHLptr => {
                let addr = pointer(cpu);
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
            Opcode::OrAH => cpu.or_a_reg(upper),
            Opcode::OrAL => cpu.or_a_reg(lower),
            Opcode::OrAA => cpu.or_a_reg(RegisterCode::A),
            Opcode::OrAHLptr => {
                let addr = pointer(cpu);
                cpu.or_a_addr(addr);
            }
            Opcode::OrALit => {
                let val = cpu.imm_addr();
                cpu.or_a_lit(val);
            }

            Opcode::XorAB => cpu.xor_a_reg(RegisterCode::B),
            Opcode::XorAC => cpu.xor_a_reg(RegisterCode::C),
            Opcode::XorAD => cpu.xor_a_reg(RegisterCode::D),
            Opcode::XorAE => cpu.xor_a_reg(RegisterCode::E),
            Opcode::XorAH => cpu.xor_a_reg(upper),
            Opcode::XorAL => cpu.xor_a_reg(lower),
            Opcode::XorAA => cpu.xor_a_reg(RegisterCode::A),
            Opcode::XorAHLptr => {
                let addr = pointer(cpu);
                cpu.xor_a_addr(addr);
            }
            Opcode::XorALit => {
                let val = cpu.imm_addr();
                cpu.xor_a_lit(val);
            }

            Opcode::CpAB => cpu.cp_a_reg(RegisterCode::B),
            Opcode::CpAC => cpu.cp_a_reg(RegisterCode::C),
            Opcode::CpAD => cpu.cp_a_reg(RegisterCode::D),
            Opcode::CpAE => cpu.cp_a_reg(RegisterCode::E),
            Opcode::CpAH => cpu.cp_a_reg(upper),
            Opcode::CpAL => cpu.cp_a_reg(lower),
            Opcode::CpAA => cpu.cp_a_reg(RegisterCode::A),
            Opcode::CpAHLptr => {
                let addr = pointer(cpu);
                cpu.cp_a_addr(addr);
            }
            Opcode::CpALit => {
                let val = cpu.imm_addr();
                cpu.cp_a_lit(val);
            }
            Opcode::Rlca => cpu.rlca(),
            Opcode::Rla => cpu.rla(),
            Opcode::Rrca => cpu.rrca(),
            Opcode::Rra => cpu.rra(),

            Opcode::JpLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp(addr);
            }
            Opcode::JpNzLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp_cond(addr, Flags::Zero, false);
            }
            Opcode::JpNcLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp_cond(addr, Flags::Carry, false);
            }
            Opcode::JpPoLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp_cond(addr, Flags::OverflowParity, false);
            }
            Opcode::JpPLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp_cond(addr, Flags::Sign, false);
            }
            Opcode::JpZLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp_cond(addr, Flags::Zero, true);
            }
            Opcode::JpCLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp_cond(addr, Flags::Carry, true);
            }
            Opcode::JpPeLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp_cond(addr, Flags::OverflowParity, true);
            }
            Opcode::JpMLit => {
                let addr = cpu.imm_addr_ex();
                cpu.jmp_cond(addr, Flags::Sign, true);
            }

            Opcode::JrLit => {
                let addr = cpu.rel_addr();
                cpu.jmp(addr);
            }

            Opcode::JrCLit => {
                let addr = cpu.rel_addr();
                cpu.jmp_cond(addr, Flags::Carry, true);
            }
            Opcode::JrNcLit => {
                let addr = cpu.rel_addr();
                cpu.jmp_cond(addr, Flags::Carry, false);
            }
            Opcode::JrZLit => {
                let addr = cpu.rel_addr();
                cpu.jmp_cond(addr, Flags::Zero, true);
            }
            Opcode::JrNzLit => {
                let addr = cpu.rel_addr();
                cpu.jmp_cond(addr, Flags::Zero, false);
            }
            Opcode::JpHLptr => {
                let addr = pointer(cpu);
                cpu.jmp(addr);
            }

            Opcode::DJNz => cpu.djnz(),
            Opcode::ExAfAf => cpu.ex_af_altaf(),
            Opcode::ExSPptrHL => cpu.ex_spptr_reg(reg),
            Opcode::Exx => cpu.exx(),
            Opcode::ExDEHL => cpu.ex_de_hl(),

            Opcode::Ei => cpu.enable_intrpt(),
            Opcode::Di => cpu.disable_intrpt(),

            Opcode::Daa => cpu.daa(),

            Opcode::Scf => cpu.scf(),
            Opcode::Ccf => cpu.ccf(),

            Opcode::CallLit => {
                let addr = cpu.imm_addr_ex();
                cpu.call_addr(addr);
            }
            Opcode::CallNz => {
                let addr = cpu.imm_addr_ex();
                cpu.call_cond_addr(addr, Flags::Zero, false);
            }
            Opcode::CallNc => {
                let addr = cpu.imm_addr_ex();
                cpu.call_cond_addr(addr, Flags::Carry, false);
            }
            Opcode::CallPo => {
                let addr = cpu.imm_addr_ex();
                cpu.call_cond_addr(addr, Flags::OverflowParity, false);
            }
            Opcode::CallP => {
                let addr = cpu.imm_addr_ex();
                cpu.call_cond_addr(addr, Flags::Sign, false);
            }
            Opcode::CallZ => {
                let addr = cpu.imm_addr_ex();
                cpu.call_cond_addr(addr, Flags::Zero, true);
            }
            Opcode::CallC => {
                let addr = cpu.imm_addr_ex();
                cpu.call_cond_addr(addr, Flags::Carry, true);
            }
            Opcode::CallPe => {
                let addr = cpu.imm_addr_ex();
                cpu.call_cond_addr(addr, Flags::OverflowParity, true);
            }
            Opcode::CallM => {
                let addr = cpu.imm_addr_ex();
                cpu.call_cond_addr(addr, Flags::Sign, true);
            }

            Opcode::Ret => cpu.ret(),
            Opcode::RetNz => cpu.ret_cond(Flags::Zero, false),
            Opcode::RetNc => cpu.ret_cond(Flags::Carry, false),
            Opcode::RetPo => cpu.ret_cond(Flags::OverflowParity, false),
            Opcode::RetP => cpu.ret_cond(Flags::Sign, false),
            Opcode::RetZ => cpu.ret_cond(Flags::Zero, true),
            Opcode::RetC => cpu.ret_cond(Flags::Carry, true),
            Opcode::RetPe => cpu.ret_cond(Flags::OverflowParity, true),
            Opcode::RetM => cpu.ret_cond(Flags::Sign, true),

            Opcode::Halt => cpu.halt(),

            Opcode::Rst00 => cpu.rst_lit(0x00),
            Opcode::Rst10 => cpu.rst_lit(0x10),
            Opcode::Rst20 => cpu.rst_lit(0x20),
            Opcode::Rst30 => cpu.rst_lit(0x30),
            Opcode::Rst08 => cpu.rst_lit(0x08),
            Opcode::Rst18 => cpu.rst_lit(0x18),
            Opcode::Rst28 => cpu.rst_lit(0x28),
            Opcode::Rst38 => cpu.rst_lit(0x38),

            // Extended Opcodes
            Opcode::Ix => {
                cpu.queue_clock_tick(4);
                let opcode = Opcode::from_u8(cpu.imm_addr());
                Opcode::operate_with(
                    cpu,
                    opcode,
                    RegisterCode16::IX,
                    RegisterCode::IXh,
                    RegisterCode::IXl,
                    |cpu| cpu.index_addr(RegisterCode16::IX),
                    IndexedBitsOperator::new(RegisterCode16::IX),
                );
            }

            Opcode::Iy => {
                cpu.queue_clock_tick(4);
                let opcode = Opcode::from_u8(cpu.imm_addr());
                Opcode::operate_with(
                    cpu,
                    opcode,
                    RegisterCode16::IY,
                    RegisterCode::IYh,
                    RegisterCode::IYl,
                    |cpu| cpu.index_addr(RegisterCode16::IY),
                    IndexedBitsOperator::new(RegisterCode16::IY),
                );
            }
            Opcode::Bits => {
                let bits_opcode = cpu.imm_addr();
                BitsOpcode::operate_u8(cpu, bits_opcode, bits_op);
            }
            Opcode::Extd => {
                let extd_opcode = cpu.imm_addr();
                Extnd::operate_u8(cpu, extd_opcode);
            }
            _ => panic!("Unimplemented for Opcode {:?}!", opcode),
        }
    }

    fn decode(opcode: Opcode) -> String {
        match opcode {
            // ld Reg, Reg
            Opcode::LdBB => String::from("Ld B, B"),
            Opcode::LdBC => String::from("Ld B, C"),
            Opcode::LdBD => String::from("Ld B, D"),
            Opcode::LdBE => String::from("Ld B, E"),
            Opcode::LdBH => String::from("Ld B, H"),
            Opcode::LdBL => String::from("Ld B, L"),
            Opcode::LdBA => String::from("Ld B, A"),
            Opcode::LdDB => String::from("Ld D, B"),
            Opcode::LdDC => String::from("Ld D, C"),
            Opcode::LdDD => String::from("Ld D, D"),
            Opcode::LdDE => String::from("Ld D, E"),
            Opcode::LdDH => String::from("Ld D, H"),
            Opcode::LdDL => String::from("Ld D, L"),
            Opcode::LdDA => String::from("Ld D, A"),
            Opcode::LdHB => String::from("Ld H, B"),
            Opcode::LdHC => String::from("Ld H, C"),
            Opcode::LdHD => String::from("Ld H, D"),
            Opcode::LdHE => String::from("Ld H, E"),
            Opcode::LdHH => String::from("Ld H, H"),
            Opcode::LdHL => String::from("Ld H, L"),
            Opcode::LdHA => String::from("Ld H, A"),
            Opcode::LdCB => String::from("Ld C, B"),
            Opcode::LdCC => String::from("Ld C, C"),
            Opcode::LdCD => String::from("Ld C, D"),
            Opcode::LdCE => String::from("Ld C, E"),
            Opcode::LdCH => String::from("Ld C, H"),
            Opcode::LdCL => String::from("Ld C, L"),
            Opcode::LdCA => String::from("Ld C, A"),
            Opcode::LdEB => String::from("Ld E, B"),
            Opcode::LdEC => String::from("Ld E, C"),
            Opcode::LdED => String::from("Ld E, D"),
            Opcode::LdEE => String::from("Ld E, E"),
            Opcode::LdEH => String::from("Ld E, H"),
            Opcode::LdEL => String::from("Ld E, L"),
            Opcode::LdEA => String::from("Ld E, A"),
            Opcode::LdLB => String::from("Ld L, B"),
            Opcode::LdLC => String::from("Ld L, C"),
            Opcode::LdLD => String::from("Ld L, D"),
            Opcode::LdLE => String::from("Ld L, E"),
            Opcode::LdLH => String::from("Ld L, H"),
            Opcode::LdLL => String::from("Ld L, L"),
            Opcode::LdLA => String::from("Ld L, A"),
            Opcode::LdAB => String::from("Ld A, B"),
            Opcode::LdAC => String::from("Ld A, C"),
            Opcode::LdAD => String::from("Ld A, D"),
            Opcode::LdAE => String::from("Ld A, E"),
            Opcode::LdAH => String::from("Ld A, H"),
            Opcode::LdAL => String::from("Ld A, L"),
            Opcode::LdAA => String::from("Ld A, A"),

            // Load Reg, Literal
            Opcode::LdBLit => String::from("Ld, B, *"),
            Opcode::LdDLit => String::from("Ld, D, *"),
            Opcode::LdHLit => String::from("Ld, H, *"),
            Opcode::LdCLit => String::from("Ld, C, *"),
            Opcode::LdELit => String::from("Ld, E, *"),
            Opcode::LdLLit => String::from("Ld, L, *"),
            Opcode::LdALit => String::from("Ld, A, *"),

            // Load Reg, (16 Bit Pair)
            Opcode::LdBHLptr => String::from("Ld B, (HL)"),
            Opcode::LdDHLptr => String::from("Ld D, (HL)"),
            Opcode::LdHHLptr => String::from("Ld H, (HL)"),
            Opcode::LdCHLptr => String::from("Ld C, (HL)"),
            Opcode::LdEHLptr => String::from("Ld E, (HL)"),
            Opcode::LdLHLptr => String::from("Ld L, (HL)"),
            Opcode::LdAHLptr => String::from("Ld A, (HL)"),
            Opcode::LdABCptr => String::from("Ld A, (BC)"),
            Opcode::LdADEptr => String::from("Ld A, (DE)"),

            // Load (HL), Reg
            Opcode::LdHLptrB => String::from("Ld (HL), B"),
            Opcode::LdHLptrC => String::from("Ld AHL), C"),
            Opcode::LdHLptrD => String::from("Ld (HL), D"),
            Opcode::LdHLptrE => String::from("Ld (HL), E"),
            Opcode::LdHLptrH => String::from("Ld (HL), H"),
            Opcode::LdHLptrL => String::from("Ld (HL), L"),
            Opcode::LdHLptrA => String::from("Ld (HL), A"),

            // Ld (HL), literal
            Opcode::LdHlptrLit => String::from("Ld (HL), *"),

            // ld (literal), Reg
            Opcode::LdLitptrH => String::from("Ld (**), H"),
            Opcode::LdLitptrA => String::from("Ld (**), A"),
            // ld (16 bit pair), reg
            Opcode::LdBCptrA => String::from("Ld (BC), A"),
            Opcode::LdDEptrA => String::from("Ld (DE), A"),

            Opcode::LdBCLit => String::from("Ld, BC, **"),
            Opcode::LdDELit => String::from("Ld, DE, **"),
            Opcode::LdHLLit => String::from("Ld, HL, **"),
            Opcode::LdSpLit => String::from("Ld, Sp, **"),

            Opcode::LdSpHL => String::from("Ld, SP, HL"),

            /* ------------- inc Reg ------------- */
            Opcode::IncB => String::from("Inc B"),
            Opcode::IncD => String::from("Inc D"),
            Opcode::IncH => String::from("Inc H"),
            Opcode::IncC => String::from("Inc C"),
            Opcode::IncE => String::from("Inc E"),
            Opcode::IncL => String::from("Inc L"),
            Opcode::IncA => String::from("Inc A"),

            Opcode::IncHLptr => String::from("Inc (HL)"),

            Opcode::IncBC => String::from("Inc BC"),
            Opcode::IncDE => String::from("Inc DE"),
            Opcode::IncHL => String::from("Inc HL"),
            Opcode::IncSP => String::from("Inc SP"),

            Opcode::DecB => String::from("Dec B"),
            Opcode::DecD => String::from("Dec D"),
            Opcode::DecH => String::from("Dec H"),
            Opcode::DecC => String::from("Dec C"),
            Opcode::DecE => String::from("Dec E"),
            Opcode::DecL => String::from("Dec L"),
            Opcode::DecA => String::from("Dec A"),

            Opcode::DecHLptr => String::from("Dec (HL)"),

            Opcode::DecBC => String::from("Dec BC"),
            Opcode::DecDE => String::from("Dec DE"),
            Opcode::DecHL => String::from("Dec HL"),
            Opcode::DecSP => String::from("Dec SP"),

            Opcode::AddAB => String::from("Add A, B"),
            Opcode::AddAC => String::from("Add A, C"),
            Opcode::AddAD => String::from("Add A, D"),
            Opcode::AddAE => String::from("Add A, E"),
            Opcode::AddAH => String::from("Add A, H"),
            Opcode::AddAL => String::from("Add A, L"),
            Opcode::AddAA => String::from("Add A, A"),
            Opcode::AddAHLptr => String::from("Add A, (HL)"),

            Opcode::AddALit => String::from("Add A, *"),

            // ADD Acc, Reg
            Opcode::AdcAB => String::from("AdC, A, B"),
            Opcode::AdcAC => String::from("AdC, A, C"),
            Opcode::AdcAD => String::from("AdC, A, D"),
            Opcode::AdcAE => String::from("AdC, A, E"),
            Opcode::AdcAH => String::from("AdC, A, H"),
            Opcode::AdcAL => String::from("AdC, A, L"),
            Opcode::AdcAA => String::from("AdC, A, A"),
            Opcode::AdcAHLptr => String::from("AdC A, (HL)"),
            Opcode::AdcALit => String::from("AdC A, *"),
            Opcode::SubAB => String::from("Sub A, B"),
            Opcode::SubAC => String::from("Sub A, C"),
            Opcode::SubAD => String::from("Sub A, D"),
            Opcode::SubAE => String::from("Sub A, E"),
            Opcode::SubAH => String::from("Sub A, H"),
            Opcode::SubAL => String::from("Sub A, L"),
            Opcode::SubAA => String::from("Sub A, A"),
            Opcode::SubAHLptr => String::from("Sub A, (HL)"),
            Opcode::SubALit => String::from("A, *"),

            Opcode::SubcAB => String::from("SbC A, B"),
            Opcode::SubcAC => String::from("SbC A, C"),
            Opcode::SubcAD => String::from("SbC A, D"),
            Opcode::SubcAE => String::from("SbC A, E"),
            Opcode::SubcAH => String::from("SbC A, H"),
            Opcode::SubcAL => String::from("SbC A, L"),
            Opcode::SubcAA => String::from("SbC A, A"),
            Opcode::SubcAHLptr => String::from("SbC A, (HL)"),
            Opcode::SubcALit => String::from("SbC A, *"),
            Opcode::AndAB => String::from("And A, B"),
            Opcode::AndAC => String::from("And A, C"),
            Opcode::AndAD => String::from("And A, D"),
            Opcode::AndAE => String::from("And A, E"),
            Opcode::AndAH => String::from("And A, H"),
            Opcode::AndAL => String::from("And A, L"),
            Opcode::AndAA => String::from("And A, A"),
            Opcode::AndAHLptr => String::from("And A, (HL)"),
            Opcode::AndALit => String::from("And A, *"),

            Opcode::OrAB => String::from("Or A, B"),
            Opcode::OrAC => String::from("Or A, C"),
            Opcode::OrAD => String::from("Or A, D"),
            Opcode::OrAE => String::from("Or A, E"),
            Opcode::OrAH => String::from("Or A, H"),
            Opcode::OrAL => String::from("Or A, L"),
            Opcode::OrAA => String::from("Or A, A"),
            Opcode::OrAHLptr => String::from("Or A, (HL)"),
            Opcode::OrALit => String::from("Or A, *"),

            Opcode::XorAB => String::from("Xor A, B"),
            Opcode::XorAC => String::from("Xor A, C"),
            Opcode::XorAD => String::from("Xor A, D"),
            Opcode::XorAE => String::from("Xor A, E"),
            Opcode::XorAH => String::from("Xor A, H"),
            Opcode::XorAL => String::from("Xor A, L"),
            Opcode::XorAA => String::from("Xor A, A"),
            Opcode::XorAHLptr => String::from("Xor A, (HL)"),
            Opcode::XorALit => String::from("Xor A, *"),

            Opcode::CpAB => String::from("Cp A, B"),
            Opcode::CpAC => String::from("Cp A, C"),
            Opcode::CpAD => String::from("Cp A, D"),
            Opcode::CpAE => String::from("Cp A, E"),
            Opcode::CpAH => String::from("Cp A, H"),
            Opcode::CpAL => String::from("Cp A, L"),
            Opcode::CpAA => String::from("Cp A, A"),
            Opcode::CpAHLptr => String::from("Cp A, (HL)"),
            Opcode::CpALit => String::from("Cp A, *"),

            Opcode::Rlca => String::from("Rlca"),
            Opcode::Rla => String::from("Rla"),
            Opcode::Rrca => String::from("Rrca"),
            Opcode::Rra => String::from("Rra"),

            Opcode::JpLit => String::from("Jmp"),
            Opcode::JpNzLit => String::from("Jmp nz"),
            Opcode::JpNcLit => String::from("Jmp nc"),
            Opcode::JpPoLit => String::from("Jmp po"),
            Opcode::JpPLit => String::from("Jmp p"),
            Opcode::JpZLit => String::from("Jmp z"),
            Opcode::JpCLit => String::from("Jmp c"),
            Opcode::JpPeLit => String::from("Jmp pe"),
            Opcode::JpMLit => String::from("Jmp m"),

            Opcode::JrLit => String::from("Jr *"),

            // Extended Opcodes
            Opcode::Ix => panic!("Unimplemented!"),
            Opcode::Iy => panic!("Unimplemented!"),
            Opcode::Bits => panic!("Unimplemented!"),
            Opcode::Extd => panic!("Unimplemented!"),
            _ => panic!("Unimplemented!"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_decode() {
        let op = Opcode::from_u8(0x47);
        assert_eq!("Ld B, A", Opcode::decode(op));
    }
}
