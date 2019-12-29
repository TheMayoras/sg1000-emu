extern crate num;

use super::{Cpu, RegisterCode, RegisterCode16};

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
}

impl BitsOpcode {
    pub fn from_u8(value: u8) -> BitsOpcode {
        num::FromPrimitive::from_u8(value).unwrap()
    }

    pub fn operate_u8(cpu: &mut Cpu, value: u8) {
        BitsOpcode::operate(cpu, BitsOpcode::from_u8(value));
    }

    pub fn operate(cpu: &mut Cpu, opcode: BitsOpcode) {
        use super::Flags;
    }
}
