extern crate num;

use super::{BitsOperator, Cpu, RegisterCode};

#[repr(u8)]
#[derive(FromPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
pub enum BitsOpcode {
    RlcB,
    RlcC,
    RlcD,
    RlcE,
    RlcH,
    RlcL,
    RlcA = 0x07,
    RlcHLptr = 0x06,

    RlB = 0x10,
    RlC,
    RlD,
    RlE,
    RlH,
    RlL,
    RlA = 0x17,
    RlHLptr = 0x16,

    SlaB = 0x20,
    SlaC,
    SlaD,
    SlaE,
    SlaH,
    SlaL,
    SlaA = 0x27,
    SlaHLptr = 0x26,

    SllB = 0x30,
    SllC,
    SllD,
    SllE,
    SllH,
    SllL,
    SllA = 0x37,
    SllHLptr = 0x36,

    RrcB = 0x08,
    RrcC,
    RrcD,
    RrcE,
    RrcH,
    RrcL,
    RrcA = 0x0F,
    RrcHLptr = 0x0E,

    RrB = 0x18,
    RrC,
    RrD,
    RrE,
    RrH,
    RrL,
    RrA = 0x1F,
    RrHLptr = 0x1E,

    SraB = 0x28,
    SraC,
    SraD,
    SraE,
    SraH,
    SraL,
    SraA = 0x2F,
    SraFLptr = 0x2E,

    SrlB = 0x38,
    SrlC,
    SrlD,
    SrlE,
    SrlH,
    SrlL,
    SrlA = 0x3F,
    SrlHLptr = 0x3E,

    // Test Bits of B
    Bit0B = 0x40,
    Bit1B = 0x48,
    Bit2B = 0x50,
    Bit3B = 0x58,
    Bit4B = 0x60,
    Bit5B = 0x68,
    Bit6B = 0x70,
    Bit7B = 0x78,

    // Test Bits of C
    Bit0C = 0x41,
    Bit1C = 0x49,
    Bit2C = 0x51,
    Bit3C = 0x59,
    Bit4C = 0x61,
    Bit5C = 0x69,
    Bit6C = 0x71,
    Bit7C = 0x79,

    // Test Bits of D
    Bit0D = 0x42,
    Bit1D = 0x4A,
    Bit2D = 0x52,
    Bit3D = 0x5A,
    Bit4D = 0x62,
    Bit5D = 0x6A,
    Bit6D = 0x72,
    Bit7D = 0x7A,

    // Test Bits of E
    Bit0E = 0x43,
    Bit1E = 0x4B,
    Bit2E = 0x53,
    Bit3E = 0x5B,
    Bit4E = 0x63,
    Bit5E = 0x6B,
    Bit6E = 0x73,
    Bit7E = 0x7B,

    // Test Bits of H
    Bit0H = 0x44,
    Bit1H = 0x4C,
    Bit2H = 0x54,
    Bit3H = 0x5C,
    Bit4H = 0x64,
    Bit5H = 0x6C,
    Bit6H = 0x74,
    Bit7H = 0x7C,

    // Test Bits of L
    Bit0L = 0x45,
    Bit1L = 0x4D,
    Bit2L = 0x55,
    Bit3L = 0x5D,
    Bit4L = 0x65,
    Bit5L = 0x6D,
    Bit6L = 0x75,
    Bit7L = 0x7D,

    // Test Bits of (HL)
    Bit0HLptr = 0x46,
    Bit1HLptr = 0x4E,
    Bit2HLptr = 0x56,
    Bit3HLptr = 0x5E,
    Bit4HLptr = 0x66,
    Bit5HLptr = 0x6E,
    Bit6HLptr = 0x76,
    Bit7HLptr = 0x7E,

    // Test Bits of A
    Bit0A = 0x47,
    Bit1A = 0x4F,
    Bit2A = 0x57,
    Bit3A = 0x5F,
    Bit4A = 0x67,
    Bit5A = 0x6F,
    Bit6A = 0x77,
    Bit7A = 0x7F,

    // Reset Bits of B
    Res0B = 0x80,
    Res1B = 0x88,
    Res2B = 0x90,
    Res3B = 0x98,
    Res4B = 0xa0,
    Res5B = 0xa8,
    Res6B = 0xb0,
    Res7B = 0xb8,

    // Reset Bits of C
    Res0C = 0x81,
    Res1C = 0x89,
    Res2C = 0x91,
    Res3C = 0x99,
    Res4C = 0xa1,
    Res5C = 0xa9,
    Res6C = 0xb1,
    Res7C = 0xb9,

    // Reset Bits of D
    Res0D = 0x82,
    Res1D = 0x8A,
    Res2D = 0x92,
    Res3D = 0x9A,
    Res4D = 0xa2,
    Res5D = 0xaA,
    Res6D = 0xb2,
    Res7D = 0xbA,

    // Reset Bits of E
    Res0E = 0x83,
    Res1E = 0x8B,
    Res2E = 0x93,
    Res3E = 0x9B,
    Res4E = 0xa3,
    Res5E = 0xaB,
    Res6E = 0xb3,
    Res7E = 0xbB,

    // Reset Bits of H
    Res0H = 0x84,
    Res1H = 0x8C,
    Res2H = 0x94,
    Res3H = 0x9C,
    Res4H = 0xa4,
    Res5H = 0xaC,
    Res6H = 0xb4,
    Res7H = 0xbC,

    // Reset Bits of L
    Res0L = 0x85,
    Res1L = 0x8D,
    Res2L = 0x95,
    Res3L = 0x9D,
    Res4L = 0xa5,
    Res5L = 0xaD,
    Res6L = 0xb5,
    Res7L = 0xbD,

    // Reset Bits of (HL)
    Res0HLptr = 0x86,
    Res1HLptr = 0x8E,
    Res2HLptr = 0x96,
    Res3HLptr = 0x9E,
    Res4HLptr = 0xA6,
    Res5HLptr = 0xAE,
    Res6HLptr = 0xB6,
    Res7HLptr = 0xBE,

    // Reset BITs of A
    Res0A = 0x87,
    Res1A = 0x8F,
    Res2A = 0x97,
    Res3A = 0x9F,
    Res4A = 0xA7,
    Res5A = 0xAF,
    Res6A = 0xB7,
    Res7A = 0xBF,

    // Set Bits of B
    Set0B = 0xC0,
    Set1B = 0xC8,
    Set2B = 0xD0,
    Set3B = 0xD8,
    Set4B = 0xE0,
    Set5B = 0xE8,
    Set6B = 0xF0,
    Set7B = 0xF8,

    // Set Bits of C
    Set0C = 0xC1,
    Set1C = 0xC9,
    Set2C = 0xD1,
    Set3C = 0xD9,
    Set4C = 0xE1,
    Set5C = 0xE9,
    Set6C = 0xF1,
    Set7C = 0xF9,

    // Set Bits of D
    Set0D = 0xC2,
    Set1D = 0xCA,
    Set2D = 0xD2,
    Set3D = 0xDA,
    Set4D = 0xE2,
    Set5D = 0xEA,
    Set6D = 0xF2,
    Set7D = 0xFA,

    // Set Bits of E
    Set0E = 0xC3,
    Set1E = 0xCB,
    Set2E = 0xD3,
    Set3E = 0xDB,
    Set4E = 0xE3,
    Set5E = 0xEB,
    Set6E = 0xF3,
    Set7E = 0xFB,

    // Set Bits of H
    Set0H = 0xC4,
    Set1H = 0xCC,
    Set2H = 0xD4,
    Set3H = 0xDC,
    Set4H = 0xE4,
    Set5H = 0xEC,
    Set6H = 0xF4,
    Set7H = 0xFC,

    // Set Bits of L
    Set0L = 0xC5,
    Set1L = 0xCD,
    Set2L = 0xD5,
    Set3L = 0xDD,
    Set4L = 0xE5,
    Set5L = 0xED,
    Set6L = 0xF5,
    Set7L = 0xFD,

    // Set Bits of (HL)
    Set0HLptr = 0xC6,
    Set1HLptr = 0xCE,
    Set2HLptr = 0xD6,
    Set3HLptr = 0xDE,
    Set4HLptr = 0xE6,
    Set5HLptr = 0xEE,
    Set6HLptr = 0xF6,
    Set7HLptr = 0xFE,

    // Set Bits of A
    Set0A = 0xC7,
    Set1A = 0xCF,
    Set2A = 0xD7,
    Set3A = 0xDF,
    Set4A = 0xE7,
    Set5A = 0xEF,
    Set6A = 0xF7,
    Set7A = 0xFF,
}

impl BitsOpcode {
    pub fn from_u8(value: u8) -> BitsOpcode {
        num::FromPrimitive::from_u8(value).unwrap()
    }

    pub fn operate_u8<U>(cpu: &mut Cpu, value: u8, bits_op: &mut U)
    where
        U: BitsOperator,
    {
        BitsOpcode::operate(cpu, BitsOpcode::from_u8(value), bits_op);
    }

    pub fn operate<U>(cpu: &mut Cpu, opcode: BitsOpcode, bits_op: &mut U)
    where
        U: BitsOperator,
    {
        println!("Found Bits Opcode: {:?}", opcode);

        bits_op.prepare(cpu);

        use BitsOpcode::*;
        match opcode {
            RlcB => {
                bits_op.pre_operate(cpu, RegisterCode::B);
                cpu.rlc_reg(RegisterCode::B);
                bits_op.post_operate(cpu, RegisterCode::B);
            }
            RlcC => {
                bits_op.pre_operate(cpu, RegisterCode::C);
                cpu.rlc_reg(RegisterCode::C);
                bits_op.post_operate(cpu, RegisterCode::C);
            }
            RlcD => {
                bits_op.pre_operate(cpu, RegisterCode::D);
                cpu.rlc_reg(RegisterCode::D);
                bits_op.post_operate(cpu, RegisterCode::D);
            }
            RlcE => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.rlc_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            RlcH => {
                bits_op.pre_operate(cpu, RegisterCode::H);
                cpu.rlc_reg(RegisterCode::H);
                bits_op.post_operate(cpu, RegisterCode::H);
            }
            RlcL => {
                bits_op.pre_operate(cpu, RegisterCode::L);
                cpu.rlc_reg(RegisterCode::L);
                bits_op.post_operate(cpu, RegisterCode::L);
            }
            RlcA => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.rlc_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            RlcHLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.rlc_addr(addr);
            }

            RlB => {
                bits_op.pre_operate(cpu, RegisterCode::B);
                cpu.rl_reg(RegisterCode::B);
                bits_op.post_operate(cpu, RegisterCode::B);
            }
            RlC => {
                bits_op.pre_operate(cpu, RegisterCode::D);
                cpu.rl_reg(RegisterCode::D);
                bits_op.post_operate(cpu, RegisterCode::D);
            }
            RlD => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.rl_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            RlE => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.rl_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            RlH => {
                bits_op.pre_operate(cpu, RegisterCode::H);
                cpu.rl_reg(RegisterCode::H);
                bits_op.post_operate(cpu, RegisterCode::H);
            }
            RlL => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.rl_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            RlA => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.rl_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            RlHLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.rl_addr(addr);
            }

            SlaB => {
                bits_op.pre_operate(cpu, RegisterCode::B);
                cpu.sla_reg(RegisterCode::B);
                bits_op.post_operate(cpu, RegisterCode::B);
            }
            SlaC => {
                bits_op.pre_operate(cpu, RegisterCode::C);
                cpu.sla_reg(RegisterCode::C);
                bits_op.post_operate(cpu, RegisterCode::C);
            }
            SlaD => {
                bits_op.pre_operate(cpu, RegisterCode::D);
                cpu.sla_reg(RegisterCode::D);
                bits_op.post_operate(cpu, RegisterCode::D);
            }
            SlaE => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.sla_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            SlaH => {
                bits_op.pre_operate(cpu, RegisterCode::H);
                cpu.sla_reg(RegisterCode::H);
                bits_op.post_operate(cpu, RegisterCode::H);
            }
            SlaL => {
                bits_op.pre_operate(cpu, RegisterCode::L);
                cpu.sla_reg(RegisterCode::L);
                bits_op.post_operate(cpu, RegisterCode::L);
            }
            SlaA => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.sla_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            SlaHLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.sla_addr(addr);
            }

            SllB => {
                bits_op.pre_operate(cpu, RegisterCode::B);
                cpu.sll_reg(RegisterCode::B);
                bits_op.post_operate(cpu, RegisterCode::B);
            }
            SllC => {
                bits_op.pre_operate(cpu, RegisterCode::C);
                cpu.sll_reg(RegisterCode::C);
                bits_op.post_operate(cpu, RegisterCode::C);
            }
            SllD => {
                bits_op.pre_operate(cpu, RegisterCode::D);
                cpu.sll_reg(RegisterCode::D);
                bits_op.post_operate(cpu, RegisterCode::D);
            }
            SllE => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.sll_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            SllH => {
                bits_op.pre_operate(cpu, RegisterCode::H);
                cpu.sll_reg(RegisterCode::H);
                bits_op.post_operate(cpu, RegisterCode::H);
            }
            SllL => {
                bits_op.pre_operate(cpu, RegisterCode::L);
                cpu.sll_reg(RegisterCode::L);
                bits_op.post_operate(cpu, RegisterCode::L);
            }
            SllA => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.sll_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            SllHLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.sll_addr(addr);
            }

            RrcB => {
                bits_op.pre_operate(cpu, RegisterCode::B);
                cpu.rrc_reg(RegisterCode::B);
                bits_op.post_operate(cpu, RegisterCode::B);
            }
            RrcC => {
                bits_op.pre_operate(cpu, RegisterCode::C);
                cpu.rrc_reg(RegisterCode::C);
                bits_op.post_operate(cpu, RegisterCode::C);
            }
            RrcD => {
                bits_op.pre_operate(cpu, RegisterCode::D);
                cpu.rrc_reg(RegisterCode::D);
                bits_op.post_operate(cpu, RegisterCode::D);
            }
            RrcE => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.rrc_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            RrcH => {
                bits_op.pre_operate(cpu, RegisterCode::H);
                cpu.rrc_reg(RegisterCode::H);
                bits_op.post_operate(cpu, RegisterCode::H);
            }
            RrcL => {
                bits_op.pre_operate(cpu, RegisterCode::L);
                cpu.rrc_reg(RegisterCode::L);
                bits_op.post_operate(cpu, RegisterCode::L);
            }
            RrcA => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.rrc_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            RrcHLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.rrc_addr(addr);
            }

            RrB => {
                bits_op.pre_operate(cpu, RegisterCode::B);
                cpu.rr_reg(RegisterCode::B);
                bits_op.post_operate(cpu, RegisterCode::B);
            }
            RrC => {
                bits_op.pre_operate(cpu, RegisterCode::C);
                cpu.rr_reg(RegisterCode::C);
                bits_op.post_operate(cpu, RegisterCode::C);
            }
            RrD => {
                bits_op.pre_operate(cpu, RegisterCode::D);
                cpu.rr_reg(RegisterCode::D);
                bits_op.post_operate(cpu, RegisterCode::D);
            }
            RrE => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.rr_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            RrH => {
                bits_op.pre_operate(cpu, RegisterCode::H);
                cpu.rr_reg(RegisterCode::H);
                bits_op.post_operate(cpu, RegisterCode::H);
            }
            RrL => {
                bits_op.pre_operate(cpu, RegisterCode::L);
                cpu.rr_reg(RegisterCode::L);
                bits_op.post_operate(cpu, RegisterCode::L);
            }
            RrA => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.rr_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            RrHLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.rr_addr(addr);
            }

            SraB => {
                bits_op.pre_operate(cpu, RegisterCode::B);
                cpu.sra_reg(RegisterCode::B);
                bits_op.post_operate(cpu, RegisterCode::B);
            }
            SraC => {
                bits_op.pre_operate(cpu, RegisterCode::C);
                cpu.sra_reg(RegisterCode::C);
                bits_op.post_operate(cpu, RegisterCode::C);
            }
            SraD => {
                bits_op.pre_operate(cpu, RegisterCode::D);
                cpu.sra_reg(RegisterCode::D);
                bits_op.post_operate(cpu, RegisterCode::D);
            }
            SraE => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.sra_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            SraH => {
                bits_op.pre_operate(cpu, RegisterCode::H);
                cpu.sra_reg(RegisterCode::H);
                bits_op.post_operate(cpu, RegisterCode::H);
            }
            SraL => {
                bits_op.pre_operate(cpu, RegisterCode::L);
                cpu.sra_reg(RegisterCode::L);
                bits_op.post_operate(cpu, RegisterCode::L);
            }
            SraA => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.sra_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            SraFLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.sra_addr(addr);
            }

            SrlB => {
                bits_op.pre_operate(cpu, RegisterCode::B);
                cpu.srl_reg(RegisterCode::B);
                bits_op.post_operate(cpu, RegisterCode::B);
            }
            SrlC => {
                bits_op.pre_operate(cpu, RegisterCode::C);
                cpu.srl_reg(RegisterCode::C);
                bits_op.post_operate(cpu, RegisterCode::C);
            }
            SrlD => {
                bits_op.pre_operate(cpu, RegisterCode::D);
                cpu.srl_reg(RegisterCode::D);
                bits_op.post_operate(cpu, RegisterCode::D);
            }
            SrlE => {
                bits_op.pre_operate(cpu, RegisterCode::E);
                cpu.srl_reg(RegisterCode::E);
                bits_op.post_operate(cpu, RegisterCode::E);
            }
            SrlH => {
                bits_op.pre_operate(cpu, RegisterCode::H);
                cpu.srl_reg(RegisterCode::H);
                bits_op.post_operate(cpu, RegisterCode::H);
            }
            SrlL => {
                bits_op.pre_operate(cpu, RegisterCode::L);
                cpu.srl_reg(RegisterCode::L);
                bits_op.post_operate(cpu, RegisterCode::L);
            }
            SrlA => {
                bits_op.pre_operate(cpu, RegisterCode::A);
                cpu.srl_reg(RegisterCode::A);
                bits_op.post_operate(cpu, RegisterCode::A);
            }
            SrlHLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.srl_addr(addr);
            }

            Bit0B => cpu.test_bit_reg(RegisterCode::B, 0),
            Bit1B => cpu.test_bit_reg(RegisterCode::B, 1),
            Bit2B => cpu.test_bit_reg(RegisterCode::B, 2),
            Bit3B => cpu.test_bit_reg(RegisterCode::B, 3),
            Bit4B => cpu.test_bit_reg(RegisterCode::B, 4),
            Bit5B => cpu.test_bit_reg(RegisterCode::B, 5),
            Bit6B => cpu.test_bit_reg(RegisterCode::B, 6),
            Bit7B => cpu.test_bit_reg(RegisterCode::B, 7),

            Bit0C => cpu.test_bit_reg(RegisterCode::C, 0),
            Bit1C => cpu.test_bit_reg(RegisterCode::C, 1),
            Bit2C => cpu.test_bit_reg(RegisterCode::C, 2),
            Bit3C => cpu.test_bit_reg(RegisterCode::C, 3),
            Bit4C => cpu.test_bit_reg(RegisterCode::C, 4),
            Bit5C => cpu.test_bit_reg(RegisterCode::C, 5),
            Bit6C => cpu.test_bit_reg(RegisterCode::C, 6),
            Bit7C => cpu.test_bit_reg(RegisterCode::C, 7),

            Bit0D => cpu.test_bit_reg(RegisterCode::D, 0),
            Bit1D => cpu.test_bit_reg(RegisterCode::D, 1),
            Bit2D => cpu.test_bit_reg(RegisterCode::D, 2),
            Bit3D => cpu.test_bit_reg(RegisterCode::D, 3),
            Bit4D => cpu.test_bit_reg(RegisterCode::D, 4),
            Bit5D => cpu.test_bit_reg(RegisterCode::D, 5),
            Bit6D => cpu.test_bit_reg(RegisterCode::D, 6),
            Bit7D => cpu.test_bit_reg(RegisterCode::D, 7),

            Bit0E => cpu.test_bit_reg(RegisterCode::E, 0),
            Bit1E => cpu.test_bit_reg(RegisterCode::E, 1),
            Bit2E => cpu.test_bit_reg(RegisterCode::E, 2),
            Bit3E => cpu.test_bit_reg(RegisterCode::E, 3),
            Bit4E => cpu.test_bit_reg(RegisterCode::E, 4),
            Bit5E => cpu.test_bit_reg(RegisterCode::E, 5),
            Bit6E => cpu.test_bit_reg(RegisterCode::E, 6),
            Bit7E => cpu.test_bit_reg(RegisterCode::E, 7),

            Bit0H => cpu.test_bit_reg(RegisterCode::H, 0),
            Bit1H => cpu.test_bit_reg(RegisterCode::H, 1),
            Bit2H => cpu.test_bit_reg(RegisterCode::H, 2),
            Bit3H => cpu.test_bit_reg(RegisterCode::H, 3),
            Bit4H => cpu.test_bit_reg(RegisterCode::H, 4),
            Bit5H => cpu.test_bit_reg(RegisterCode::H, 5),
            Bit6H => cpu.test_bit_reg(RegisterCode::H, 6),
            Bit7H => cpu.test_bit_reg(RegisterCode::H, 7),

            Bit0L => cpu.test_bit_reg(RegisterCode::L, 0),
            Bit1L => cpu.test_bit_reg(RegisterCode::L, 1),
            Bit2L => cpu.test_bit_reg(RegisterCode::L, 2),
            Bit3L => cpu.test_bit_reg(RegisterCode::L, 3),
            Bit4L => cpu.test_bit_reg(RegisterCode::L, 4),
            Bit5L => cpu.test_bit_reg(RegisterCode::L, 5),
            Bit6L => cpu.test_bit_reg(RegisterCode::L, 6),
            Bit7L => cpu.test_bit_reg(RegisterCode::L, 7),

            Bit0HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.test_bit_addr(addr, 0);
            }
            Bit1HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.test_bit_addr(addr, 1);
            }
            Bit2HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.test_bit_addr(addr, 2);
            }
            Bit3HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.test_bit_addr(addr, 3);
            }
            Bit4HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.test_bit_addr(addr, 4);
            }
            Bit5HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.test_bit_addr(addr, 5);
            }
            Bit6HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.test_bit_addr(addr, 6);
            }
            Bit7HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.test_bit_addr(addr, 7);
            }

            Bit0A => cpu.test_bit_reg(RegisterCode::A, 0),
            Bit1A => cpu.test_bit_reg(RegisterCode::A, 1),
            Bit2A => cpu.test_bit_reg(RegisterCode::A, 2),
            Bit3A => cpu.test_bit_reg(RegisterCode::A, 3),
            Bit4A => cpu.test_bit_reg(RegisterCode::A, 4),
            Bit5A => cpu.test_bit_reg(RegisterCode::A, 5),
            Bit6A => cpu.test_bit_reg(RegisterCode::A, 6),
            Bit7A => cpu.test_bit_reg(RegisterCode::A, 7),

            Res0B => cpu.change_bit_reg(RegisterCode::B, 0, false),
            Res1B => cpu.change_bit_reg(RegisterCode::B, 1, false),
            Res2B => cpu.change_bit_reg(RegisterCode::B, 2, false),
            Res3B => cpu.change_bit_reg(RegisterCode::B, 3, false),
            Res4B => cpu.change_bit_reg(RegisterCode::B, 4, false),
            Res5B => cpu.change_bit_reg(RegisterCode::B, 5, false),
            Res6B => cpu.change_bit_reg(RegisterCode::B, 6, false),
            Res7B => cpu.change_bit_reg(RegisterCode::B, 7, false),

            Res0C => cpu.change_bit_reg(RegisterCode::C, 0, false),
            Res1C => cpu.change_bit_reg(RegisterCode::C, 1, false),
            Res2C => cpu.change_bit_reg(RegisterCode::C, 2, false),
            Res3C => cpu.change_bit_reg(RegisterCode::C, 3, false),
            Res4C => cpu.change_bit_reg(RegisterCode::C, 4, false),
            Res5C => cpu.change_bit_reg(RegisterCode::C, 5, false),
            Res6C => cpu.change_bit_reg(RegisterCode::C, 6, false),
            Res7C => cpu.change_bit_reg(RegisterCode::C, 7, false),

            Res0D => cpu.change_bit_reg(RegisterCode::D, 0, false),
            Res1D => cpu.change_bit_reg(RegisterCode::D, 1, false),
            Res2D => cpu.change_bit_reg(RegisterCode::D, 2, false),
            Res3D => cpu.change_bit_reg(RegisterCode::D, 3, false),
            Res4D => cpu.change_bit_reg(RegisterCode::D, 4, false),
            Res5D => cpu.change_bit_reg(RegisterCode::D, 5, false),
            Res6D => cpu.change_bit_reg(RegisterCode::D, 6, false),
            Res7D => cpu.change_bit_reg(RegisterCode::D, 7, false),

            Res0E => cpu.change_bit_reg(RegisterCode::E, 0, false),
            Res1E => cpu.change_bit_reg(RegisterCode::E, 1, false),
            Res2E => cpu.change_bit_reg(RegisterCode::E, 2, false),
            Res3E => cpu.change_bit_reg(RegisterCode::E, 3, false),
            Res4E => cpu.change_bit_reg(RegisterCode::E, 4, false),
            Res5E => cpu.change_bit_reg(RegisterCode::E, 5, false),
            Res6E => cpu.change_bit_reg(RegisterCode::E, 6, false),
            Res7E => cpu.change_bit_reg(RegisterCode::E, 7, false),

            Res0H => cpu.change_bit_reg(RegisterCode::H, 0, false),
            Res1H => cpu.change_bit_reg(RegisterCode::H, 1, false),
            Res2H => cpu.change_bit_reg(RegisterCode::H, 2, false),
            Res3H => cpu.change_bit_reg(RegisterCode::H, 3, false),
            Res4H => cpu.change_bit_reg(RegisterCode::H, 4, false),
            Res5H => cpu.change_bit_reg(RegisterCode::H, 5, false),
            Res6H => cpu.change_bit_reg(RegisterCode::H, 6, false),
            Res7H => cpu.change_bit_reg(RegisterCode::H, 7, false),

            Res0L => cpu.change_bit_reg(RegisterCode::L, 0, false),
            Res1L => cpu.change_bit_reg(RegisterCode::L, 1, false),
            Res2L => cpu.change_bit_reg(RegisterCode::L, 2, false),
            Res3L => cpu.change_bit_reg(RegisterCode::L, 3, false),
            Res4L => cpu.change_bit_reg(RegisterCode::L, 4, false),
            Res5L => cpu.change_bit_reg(RegisterCode::L, 5, false),
            Res6L => cpu.change_bit_reg(RegisterCode::L, 6, false),
            Res7L => cpu.change_bit_reg(RegisterCode::L, 7, false),

            Res0HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 0, false);
            }
            Res1HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 1, false);
            }
            Res2HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 2, false);
            }
            Res3HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 3, false);
            }
            Res4HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 4, false);
            }
            Res5HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 5, false);
            }
            Res6HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 6, false);
            }
            Res7HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 7, false);
            }

            Res0A => cpu.change_bit_reg(RegisterCode::A, 0, true),
            Res1A => cpu.change_bit_reg(RegisterCode::A, 1, true),
            Res2A => cpu.change_bit_reg(RegisterCode::A, 2, true),
            Res3A => cpu.change_bit_reg(RegisterCode::A, 3, true),
            Res4A => cpu.change_bit_reg(RegisterCode::A, 4, true),
            Res5A => cpu.change_bit_reg(RegisterCode::A, 5, true),
            Res6A => cpu.change_bit_reg(RegisterCode::A, 6, true),
            Res7A => cpu.change_bit_reg(RegisterCode::A, 7, true),

            Set0B => cpu.change_bit_reg(RegisterCode::B, 0, true),
            Set1B => cpu.change_bit_reg(RegisterCode::B, 1, true),
            Set2B => cpu.change_bit_reg(RegisterCode::B, 2, true),
            Set3B => cpu.change_bit_reg(RegisterCode::B, 3, true),
            Set4B => cpu.change_bit_reg(RegisterCode::B, 4, true),
            Set5B => cpu.change_bit_reg(RegisterCode::B, 5, true),
            Set6B => cpu.change_bit_reg(RegisterCode::B, 6, true),
            Set7B => cpu.change_bit_reg(RegisterCode::B, 7, true),

            Set0C => cpu.change_bit_reg(RegisterCode::C, 0, true),
            Set1C => cpu.change_bit_reg(RegisterCode::C, 1, true),
            Set2C => cpu.change_bit_reg(RegisterCode::C, 2, true),
            Set3C => cpu.change_bit_reg(RegisterCode::C, 3, true),
            Set4C => cpu.change_bit_reg(RegisterCode::C, 4, true),
            Set5C => cpu.change_bit_reg(RegisterCode::C, 5, true),
            Set6C => cpu.change_bit_reg(RegisterCode::C, 6, true),
            Set7C => cpu.change_bit_reg(RegisterCode::C, 7, true),

            Set0D => cpu.change_bit_reg(RegisterCode::D, 0, true),
            Set1D => cpu.change_bit_reg(RegisterCode::D, 1, true),
            Set2D => cpu.change_bit_reg(RegisterCode::D, 2, true),
            Set3D => cpu.change_bit_reg(RegisterCode::D, 3, true),
            Set4D => cpu.change_bit_reg(RegisterCode::D, 4, true),
            Set5D => cpu.change_bit_reg(RegisterCode::D, 5, true),
            Set6D => cpu.change_bit_reg(RegisterCode::D, 6, true),
            Set7D => cpu.change_bit_reg(RegisterCode::D, 7, true),

            Set0E => cpu.change_bit_reg(RegisterCode::E, 0, true),
            Set1E => cpu.change_bit_reg(RegisterCode::E, 1, true),
            Set2E => cpu.change_bit_reg(RegisterCode::E, 2, true),
            Set3E => cpu.change_bit_reg(RegisterCode::E, 3, true),
            Set4E => cpu.change_bit_reg(RegisterCode::E, 4, true),
            Set5E => cpu.change_bit_reg(RegisterCode::E, 5, true),
            Set6E => cpu.change_bit_reg(RegisterCode::E, 6, true),
            Set7E => cpu.change_bit_reg(RegisterCode::E, 7, true),

            Set0H => cpu.change_bit_reg(RegisterCode::H, 0, true),
            Set1H => cpu.change_bit_reg(RegisterCode::H, 1, true),
            Set2H => cpu.change_bit_reg(RegisterCode::H, 2, true),
            Set3H => cpu.change_bit_reg(RegisterCode::H, 3, true),
            Set4H => cpu.change_bit_reg(RegisterCode::H, 4, true),
            Set5H => cpu.change_bit_reg(RegisterCode::H, 5, true),
            Set6H => cpu.change_bit_reg(RegisterCode::H, 6, true),
            Set7H => cpu.change_bit_reg(RegisterCode::H, 7, true),

            Set0L => cpu.change_bit_reg(RegisterCode::L, 0, true),
            Set1L => cpu.change_bit_reg(RegisterCode::L, 1, true),
            Set2L => cpu.change_bit_reg(RegisterCode::L, 2, true),
            Set3L => cpu.change_bit_reg(RegisterCode::L, 3, true),
            Set4L => cpu.change_bit_reg(RegisterCode::L, 4, true),
            Set5L => cpu.change_bit_reg(RegisterCode::L, 5, true),
            Set6L => cpu.change_bit_reg(RegisterCode::L, 6, true),
            Set7L => cpu.change_bit_reg(RegisterCode::L, 7, true),

            Set0HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 0, true);
            }
            Set1HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 1, true);
            }
            Set2HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 2, true);
            }
            Set3HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 3, true);
            }
            Set4HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 4, true);
            }
            Set5HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 5, true);
            }
            Set6HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 6, true);
            }
            Set7HLptr => {
                let addr = bits_op.pointer(cpu);
                cpu.change_bit_addr(addr, 7, true);
            }

            Set0A => cpu.change_bit_reg(RegisterCode::A, 0, true),
            Set1A => cpu.change_bit_reg(RegisterCode::A, 1, true),
            Set2A => cpu.change_bit_reg(RegisterCode::A, 2, true),
            Set3A => cpu.change_bit_reg(RegisterCode::A, 3, true),
            Set4A => cpu.change_bit_reg(RegisterCode::A, 4, true),
            Set5A => cpu.change_bit_reg(RegisterCode::A, 5, true),
            Set6A => cpu.change_bit_reg(RegisterCode::A, 6, true),
            Set7A => cpu.change_bit_reg(RegisterCode::A, 7, true),
        }
    }
}
