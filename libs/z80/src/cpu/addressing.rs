use crate::cpu::{Cpu, RegisterCode16};

pub trait AddressMode<T> {
    /// get the address value
    fn get(&mut self, cpu: &mut Cpu) -> T;

    /// peek the address value at addr
    ///
    /// # Arguments
    /// * `cpu` - the cpu being used  
    /// * `addr` - the address to use as the base address for the
    fn peek(&self, cpu: &Cpu, addr: u16) -> T;
}

pub struct ImmediateAddressing {}

impl ImmediateAddressing {
    pub fn new() -> ImmediateAddressing {
        ImmediateAddressing {}
    }
}

impl AddressMode<u8> for ImmediateAddressing {
    fn get(&mut self, cpu: &mut Cpu) -> u8 {
        cpu.next_byte()
    }

    fn peek(&self, cpu: &Cpu, start: u16) -> u8 {
        cpu.fetch(start)
    }
}

/// Extended Addressing
///
/// Returns the address specified by the next _two_ bytes of memory
/// starting at the current program counter.
/// The function increments the program counter by two
pub struct ImmediateAddressingExt {}

impl ImmediateAddressingExt {
    pub fn new() -> ImmediateAddressingExt {
        ImmediateAddressingExt {}
    }

    fn combine(&self, higher: u8, lower: u8) -> u16 {
        ((higher as u16) << 8) | (lower as u16) & 0xFF
    }
}

impl AddressMode<u16> for ImmediateAddressingExt {
    fn get(&mut self, cpu: &mut Cpu) -> u16 {
        let lower = cpu.next_byte();
        let higher = cpu.next_byte();

        self.combine(higher, lower)
    }

    fn peek(&self, cpu: &Cpu, start: u16) -> u16 {
        let imm_addr = ImmediateAddressing::new();
        let lower = imm_addr.peek(cpu, start);
        let higher = imm_addr.peek(cpu, start + 1);

        self.combine(higher, lower)
    }
}

/// Relative Addressing
///
/// Returns the address of the next byte in memory added to the current program counter
/// (the program counter is currently pointing to the instruction after the one using
/// relative addressing). This means that from the opcode using relative addressing we
/// can move +129 to -126 bytes
pub struct RelativeAddressing {}

impl RelativeAddressing {
    pub fn new() -> RelativeAddressing {
        RelativeAddressing {}
    }

    pub fn combine(&self, pc: u16, offset: u8) -> u16 {
        ((pc as i16) + (offset as i8 as i16)) as u16
    }
}

impl AddressMode<u16> for RelativeAddressing {
    fn get(&mut self, cpu: &mut Cpu) -> u16 {
        self.combine(cpu.get_pc(), cpu.next_byte())
    }

    fn peek(&self, cpu: &Cpu, start: u16) -> u16 {
        self.combine(start + 1, ImmediateAddressing::new().peek(cpu, start))
    }
}

/// Indexed Addressing
///
/// This form of addressing adds an offset designated by the next byte
/// in memory to one of the index registers.
/// The index registers are IX and IY.
/// This function will panic if an invalid register is supplied.
pub struct IndexedAddressing {
    register: RegisterCode16,
}

impl IndexedAddressing {
    pub fn new(register: RegisterCode16) -> IndexedAddressing {
        if register != RegisterCode16::IX && register != RegisterCode16::IY {
            panic!(
                "Attempting to use register '{:?}' to perform Indexed Addressing. \
                 Only Registers 'IX' and 'IY' are able to be used for this addressing mode!",
                register
            );
        }
        IndexedAddressing { register }
    }

    pub fn reg(&self, cpu: &Cpu) -> u16 {
        cpu.reg_value_16(self.register)
    }

    fn combine(&self, reg: u16, offset: u8) -> u16 {
        let offset = offset as u32;
        let reg = reg as u32;

        ((reg + offset) % 0xFFFF) as u16
    }
}

impl AddressMode<u16> for IndexedAddressing {
    fn get(&mut self, cpu: &mut Cpu) -> u16 {
        let reg = self.reg(cpu);
        let offset = cpu.next_byte();

        self.combine(reg, offset)
    }

    fn peek(&self, cpu: &Cpu, start: u16) -> u16 {
        let reg = self.reg(cpu);
        let offset = cpu.fetch(start);

        self.combine(reg, offset)
    }
}

/// Indirect Register Addressing
///
/// This form of addressing uses the value stored in one the 16 bit
/// registers pairs
pub struct IndirectRegisterAddressing {
    register: RegisterCode16,
}

impl IndirectRegisterAddressing {
    pub fn new(register: RegisterCode16) -> IndirectRegisterAddressing {
        IndirectRegisterAddressing { register }
    }
}
impl AddressMode<u16> for IndirectRegisterAddressing {
    fn get(&mut self, cpu: &mut Cpu) -> u16 {
        cpu.reg_value_16(self.register)
    }

    fn peek(&self, cpu: &Cpu, _start: u16) -> u16 {
        cpu.reg_value_16(self.register)
    }
}
