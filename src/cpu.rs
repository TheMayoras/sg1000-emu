#![allow(dead_code)]

// DONE:
// 1). LD for main group
// 2). INC for main group
// 3). Dec for main group

mod opcode;

use opcode::Opcode;

// define registers

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TriStateLogic {
    On,
    Off,
    Disconnect,
}

// register codes
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum RegisterCode {
    Flags = 0,
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

// register codes for 16 bit registers and 16 bit register pairs
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(usize)]
pub enum RegisterCode16 {
    I = 0,
    R,
    IX,
    IY,
    SP,
    PC,
    BC,
    DE,
    HL,
}

#[repr(u8)]
enum Flags {
    Carry = 0,
    AddSubtract,
    OverflowParity,
    NotUsed1,
    HalfCarry,
    NotUsed2,
    Zero,
    Sign,
}

#[rustfmt::skip]
// NOTE: all addresses are byte based, so the program counter points to a byte
pub struct Cpu {
    clock:                u64,
    read_write:           TriStateLogic,
    interrupt_enable:     bool,
    iff2:                 bool,
    reg:                  [u16; 8], // register values
    alt_reg:              [u16; 8],
    spec_reg:             [u32; 6],
    buffer:               Vec<u8>,
    stack:                Vec<u8>,
}

impl Cpu {
    #[rustfmt::skip]
    /// TODO: set buffer to point to a vector of binary file data 
    pub fn new(buf: Vec<u8>) -> Cpu {
        let mut buf = buf;
        buf.resize(256*256, 0);
        Cpu {
            clock:                0,
            read_write:           TriStateLogic::Disconnect,
            interrupt_enable:     false,
            iff2:                 false,
            reg:                  [0; 8],
            alt_reg:              [0; 8],
            spec_reg:             [0; 6],
            buffer:               buf,
            stack:                Vec::new(),
        }
    }

    //
    // getters
    //
    pub fn clock(&self) -> u64 {
        self.clock
    }

    pub fn read_write(&self) -> TriStateLogic {
        self.read_write
    }

    pub fn get_pc(&self) -> u16 {
        self.reg_value_16(RegisterCode16::PC)
    }

    pub fn reg_value(&self, code: RegisterCode) -> u8 {
        self.reg[code as usize] as u8
    }

    fn set_reg_value_16(&mut self, code: RegisterCode16, val: u16) {
        let reg_high: RegisterCode;
        let reg_low: RegisterCode;
        match code {
            RegisterCode16::BC => {
                reg_high = RegisterCode::B;
                reg_low = RegisterCode::C;
            }
            RegisterCode16::DE => {
                reg_high = RegisterCode::D;
                reg_low = RegisterCode::E;
            }
            RegisterCode16::HL => {
                reg_high = RegisterCode::H;
                reg_low = RegisterCode::L;
            }

            _ => {
                self.spec_reg[code as usize] = val as u32;
                return;
            }
        }

        self.reg[reg_high as usize] = (val >> 8) & 0xFF;
        self.reg[reg_low as usize] = val & 0xFF;
    }

    fn reg_value_16(&self, code: RegisterCode16) -> u16 {
        let reg_high: RegisterCode;
        let reg_low: RegisterCode;
        match code {
            RegisterCode16::BC => {
                reg_high = RegisterCode::B;
                reg_low = RegisterCode::C;
            }
            RegisterCode16::DE => {
                reg_high = RegisterCode::D;
                reg_low = RegisterCode::E;
            }
            RegisterCode16::HL => {
                reg_high = RegisterCode::H;
                reg_low = RegisterCode::L;
            }
            _ => return self.spec_reg[code as usize] as u16,
        }

        let high_byte = self.reg[reg_high as usize];
        let low_byte = self.reg[reg_low as usize];

        (high_byte << 8) | (low_byte & 0xFF)
    }

    fn set_flag(&mut self, f: Flags, set: bool) {
        let mut flag = self.reg[RegisterCode::Flags as usize];
        if set {
            flag |= 1 << f as u8;
        } else {
            flag &= !(1 << f as u8);
        }

        self.reg[RegisterCode::Flags as usize] = flag;
    }

    /// Get the value of a flag
    fn flag(&self, f: Flags) -> bool {
        // get the registers value.  shift that right so that the least significant bit is the flag.
        //  & with 1 to remove all other values.  return true if result is 1
        ((self.reg[RegisterCode::Flags as usize]) >> f as u8) & 1 == 1
    }

    pub fn inc_clock_n(&mut self, n: u64) {
        self.clock += n
    }

    pub fn inc_clock(&mut self) {
        self.inc_clock_n(1)
    }

    fn set_reg_value(&mut self, code: RegisterCode, value: u16) {
        self.reg[code as usize] = value;
    }

    // to test if we can set the read_write
    #[cfg(test)]
    pub fn set_read_write(&mut self, rw: TriStateLogic) {
        self.read_write = rw;
    }

    // fn push(&mut self, value: u8) {
    //     self.stack.push(value);
    //     self.reg[RegisterCode::SP as usize] += 8; // inc by a byte
    // }

    fn fetch(&self, addr: u16) -> u8 {
        self.buffer[addr as usize]
    }

    fn set_mem(&mut self, addr: u16, val: u8) {
        self.buffer[addr as usize] = val;
    }
}

// The do_operation and related functions
impl Cpu {
    /// Get the byte from the current position of program counter
    ///
    /// This function does _not_ increment the program counter.  
    /// Incrementing this value is left up to the calling function
    fn next_byte(&mut self) -> u8 {
        let byte = self.next_byte_no_inc();
        self.inc_pc();

        byte
    }

    /// Get the next byte from the current position of program counter.
    ///
    /// This function does not increment the program counter after retrieval
    fn next_byte_no_inc(&self) -> u8 {
        let pc = self.get_pc();

        // get the byte at the given buffer
        self.fetch(pc)
    }

    /// Increment the program counter by 1
    fn inc_pc(&mut self) {
        self.inc_pc_n(1);
    }

    /// Increment the program counter by the number of _bits_ supplied
    fn inc_pc_n(&mut self, bits: u16) {
        self.spec_reg[RegisterCode16::PC as usize] += bits as u32;
    }

    /* --------------------------------- ADDRESSING MODES --------------------------------- */
    /// Implied Addressing
    ///
    /// Returns 0.  This type of addressing is used when the address is implied in the opcode
    fn impl_addr(&self) -> u16 {
        0
    }

    /// Immediate Addressing Mode
    ///
    /// Returns the address of the next byte in the buffer.  The function increments the
    /// program counter to account for the fact that it used the byte.
    fn imm_addr(&mut self) -> u8 {
        self.next_byte()
    }

    /// Extended Immediate Addressing Mode
    ///
    /// Returns the address of the next _two_ bytes in the buffer.  The function
    /// incrememnts the program counter by 16 to account for the fact that is used
    /// two bytes.  The values are stored in little endian, so the first byte is
    /// smaller byte
    fn imm_addr_ex(&mut self) -> u16 {
        // load the parts of the address.
        // stored as:
        //      ...., low-byte, high-byte, ....
        let addr_lower: u16 = self.next_byte() as u16;
        let addr_higher = self.next_byte() as u16;

        (addr_higher << 8) | (addr_lower & 0xFF)
    }

    /// Relative Addressing
    ///
    /// Returns the address of the next byte in memory added to the current program counter
    /// (the program counter is currently pointing to the instruction after the one using relative addressing).
    /// This means that from the opcode using relative addressing we can move +129 to -126 bytes
    fn rel_addr(&mut self) -> u16 {
        // cast to signed 16 bit
        let byte = self.next_byte() as i8 as i16; // cast to i8 to convert to negative.  cast to i16 so we can add to the pc value

        // cast to signed 16 bit
        let cur_pc = self.get_pc() as i16;

        // cast back to unsigned 16 bit.
        // this accurately performs the addition with a potentially negative `byte`
        (cur_pc + byte) as u16
    }

    /// Extended Addressing
    ///
    /// Returns the address specified by the next _two_ bytes of memory starting at the current program counter.
    /// The function increments the program counter by two
    fn ext_addr(&mut self) -> u16 {
        let addr_lower = self.next_byte() as u16;
        let addr_higher = self.next_byte() as u16;

        (addr_higher << 8) | (addr_lower & 0xFF)
    }

    /// Indexed Addressing
    ///
    /// This form of addressing adds an offset designated by the next byte
    /// in memory to one of the index registers.
    /// The index registers are IX and IY.
    /// This function will panic if an invalid register is supplied.
    fn index_addr(&mut self, register: RegisterCode16) -> u16 {
        if register != RegisterCode16::IX && register != RegisterCode16::IY {
            panic!("Attempting to use register '{:?}' to perform Indexed Addressing.  Only Registers 'IX' and 'IY' are able to be used for this addressing mode!", register);
        }

        let reg_val = self.reg_value_16(register) as i16 as i32;

        let displacement = self.next_byte() as i8 as i32;

        // in case we wrap around.  this will
        ((reg_val + displacement) % 0xFFFF) as u16
    }

    /// Indirect Register Addressing
    ///
    /// This form of addressing uses the value stored in one the 16 bit
    /// registers pairs
    fn reg_indirect_addr(&mut self, register: RegisterCode16) -> u16 {
        self.reg_value_16(register)
    }

    /* --------------------------------- LOOP --------------------------------- */
    /// Perform the next operation
    ///
    /// Performs the next operation taken from the buffer supplied to the cpu.  
    /// This function will handle changing all internal Cpu values and will write to
    /// any necessary busses
    pub fn do_operation(&mut self) {
        unimplemented!();
    }
}

// Operations
impl Cpu {
    /// No Op
    fn noop(&self) {}

    /// load the dest reg with the src reg value
    /// LD A, B
    fn ld_reg_reg(&mut self, dst: RegisterCode, src: RegisterCode) {
        self.reg[dst as usize] = self.reg[src as usize];
    }

    /// load the dest reg with the literal value
    /// LD A, 5
    fn ld_reg_lit(&mut self, dst: RegisterCode) {
        let literal = self.next_byte();

        self.reg[dst as usize] = literal as u16;
    }

    /// load the dest reg with value pointed to by the address passed in
    fn ld_reg_addr(&mut self, dst: RegisterCode, addr: u16) {
        let value = self.buffer[addr as usize];

        self.reg[dst as usize] = value as u16
    }

    fn ld_addr_reg(&mut self, addr: u16, src: RegisterCode) {
        let value = self.reg[src as usize];

        self.buffer[addr as usize] = value as u8;
    }

    fn ld_addr_lit(&mut self, addr: u16, lit: u8) {
        self.buffer[addr as usize] = lit;
    }

    fn ld_reg16_lit(&mut self, reg: RegisterCode16, lit: u16) {
        self.set_reg_value_16(reg, lit);
    }

    fn ld_reg16_reg16(&mut self, dst: RegisterCode16, src: RegisterCode16) {
        let val = self.reg_value_16(src);
        self.set_reg_value_16(dst, val);
    }

    /* ---------------------- Incrementing ----------------- */
    fn inc_reg(&mut self, reg: RegisterCode) {
        let mut val = self.reg[reg as usize];

        self.set_flag(Flags::OverflowParity, val == 0x7F);
        self.set_flag(Flags::HalfCarry, val & 0b1111 == 0b1111);

        // increment and wrap around to 0
        val += 1;
        val %= 0xFF + 1;

        self.set_flag(Flags::Sign, (val & 0x80) > 0);
        self.set_flag(Flags::Zero, val == 0);
        self.set_flag(Flags::AddSubtract, false);

        self.reg[reg as usize] = val;
    }

    fn inc_addr(&mut self, addr: u16) {
        let mut val = self.fetch(addr) as u16;

        self.set_flag(Flags::OverflowParity, val == 0x7F);
        self.set_flag(Flags::HalfCarry, val & 0b1111 == 0b1111);

        // increment and wrap around to 0
        val += 1;
        val %= 0xFF + 1;

        self.set_flag(Flags::Sign, (val as i8) < 0);
        self.set_flag(Flags::Zero, val == 0);
        self.set_flag(Flags::AddSubtract, false);

        self.set_mem(addr, val as u8);
    }

    /// Increment a 16 bit register or register pair.  
    ///
    /// This function does not change any flags even on overflow/zero/etc.
    fn inc_reg16(&mut self, reg: RegisterCode16) {
        let mut val = self.reg_value_16(reg) as u32;
        val += 1;
        val %= 0xFFFF + 1;

        self.set_reg_value_16(reg, val as u16);
    }

    /// Decrement the register by 1
    fn dec_reg(&mut self, reg: RegisterCode) {
        let mut val = self.reg[reg as usize];

        self.set_flag(Flags::AddSubtract, true);
        self.set_flag(Flags::OverflowParity, val == 0x80);
        // set half carry if there is a borrow from bit 4 to 3
        self.set_flag(Flags::HalfCarry, val & 0b1111 == 0);

        if val == 0 {
            // wrap around
            val = 0xFF;
        } else {
            val -= 1;
        }

        // set the sign flag is the first bit is set
        self.set_flag(Flags::Sign, val > 0x80);
        self.set_flag(Flags::Zero, val == 0);

        self.reg[reg as usize] = val;
    }

    /// Decrement the register by 1
    fn dec_addr(&mut self, addr: u16) {
        let mut val = self.fetch(addr);

        self.set_flag(Flags::AddSubtract, true);
        self.set_flag(Flags::OverflowParity, val == 0x80);
        // set half carry if there is a borrow from bit 4 to 3
        self.set_flag(Flags::HalfCarry, val & 0b1111 == 0);

        if val == 0 {
            // wrap around
            val = 0xFF;
        } else {
            val -= 1;
        }

        // set the sign flag is the first bit is set
        self.set_flag(Flags::Sign, val > 0x80);
        self.set_flag(Flags::Zero, val == 0);

        self.set_mem(addr, val);
    }

    /// Decrement a 16 bit register or register pair.  
    ///
    /// This function does not change any flags even on overflow/zero/etc.
    fn dec_reg16(&mut self, reg: RegisterCode16) {
        let mut val = self.reg_value_16(reg) as u32;

        if val == 0 {
            val = 0xFFFF;
        } else {
            val -= 1;
        }

        self.set_reg_value_16(reg, val as u16);
    }

    /// Add 8 bit values
    ///
    /// This function sets the necessary flags for the addition and returns the result
    fn add_val_val(&mut self, acc: u16, operand: u16) -> u16 {
        // add the lower 4 bits of the two operands.
        // If the result > 4 bits then we have half carry
        self.set_flag(Flags::HalfCarry, (operand & 0x0F) + (acc & 0x0F) > 0x0F);

        let result = acc + operand;
        // set the carry flag before we wrap around.  We have a carry if we wrapped around to 0
        // (i.e.) went 0xFF to 0
        self.set_flag(Flags::Carry, result > 0xFF);

        let result: u16 = result % (0xFF + 1);

        // set overflow flag if:
        // 1). the signs are the same for the number being added
        // 2). the result sign differs from the operand signs
        self.set_flag(
            Flags::OverflowParity,
            acc >> 7 == operand >> 7 && acc >> 7 != result >> 7,
        );
        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::AddSubtract, false);

        result
    }

    /// Add the contents of the specified register to the accumlator register
    fn add_a_reg(&mut self, reg: RegisterCode) {
        let operand = self.reg_value(reg) as u16;
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.add_val_val(acc, operand);
        self.set_reg_value(RegisterCode::A, result);
    }

    fn add_a_addr(&mut self, addr: u16) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let operand = self.fetch(addr) as u16;

        let result = self.add_val_val(acc, operand);
        self.set_reg_value(RegisterCode::A, result);
    }

    fn add_a_lit(&mut self, lit: u8) {
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.add_val_val(acc, lit as u16);
        self.set_reg_value(RegisterCode::A, result);
    }

    /// Add the contents of the specified register to the accumlator register
    fn add_a_reg_carry(&mut self, reg: RegisterCode) {
        let operand = self.reg_value(reg) as u16;
        let acc = self.reg_value(RegisterCode::A) as u16;
        let carry = if self.flag(Flags::Carry) { 1 } else { 0 };

        let result = self.add_val_val(acc + carry, operand);
        self.set_reg_value(RegisterCode::A, result);
    }

    fn add_a_addr_carry(&mut self, addr: u16) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let operand = self.fetch(addr) as u16;
        let carry = if self.flag(Flags::Carry) { 1 } else { 0 };

        let result = self.add_val_val(acc + carry, operand);
        self.set_reg_value(RegisterCode::A, result);
    }

    fn add_a_lit_carry(&mut self, lit: u8) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let carry = if self.flag(Flags::Carry) { 1 } else { 0 };

        let result = self.add_val_val(acc + carry, lit as u16);
        self.set_reg_value(RegisterCode::A, result);
    }

    /// Subtract the value in the operand from the Accumulator
    ///
    /// This function sets all of the necessary flags the subtraction and returns the result
    /// to the caller.
    fn sub_val_val(&mut self, acc: u16, operand: u16, carry: bool) -> u16 {
        let carry = if carry { 1 } else { 0 };
        let result = if acc >= (operand + carry) {
            self.set_flag(Flags::Carry, true);
            acc - operand - carry
        } else {
            self.set_flag(Flags::Carry, true);
            let remainder = (carry + operand) - acc;
            0xFF - remainder + 1
        };

        self.set_flag(Flags::Sign, result > 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::AddSubtract, true);
        // set half carry if 4th bit of acc == 0 and 4th bit of result != 0
        self.set_flag(Flags::HalfCarry, acc & 0x0F < operand & 0x0F);
        self.set_flag(
            Flags::OverflowParity,
            acc & 0x80 != operand & 0x80 && acc & 0x80 != result & 0x80,
        );

        result
    }

    fn sub_a_reg(&mut self, reg: RegisterCode) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let val = self.reg_value(reg) as u16;

        let result = self.sub_val_val(acc, val, false);

        self.set_reg_value(RegisterCode::A, result);
    }

    fn sub_a_addr(&mut self, addr: u16) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let operand = self.fetch(addr) as u16;

        let result = self.sub_val_val(acc, operand, false);
        self.set_reg_value(RegisterCode::A, result);
    }

    fn sub_a_lit(&mut self, lit: u8) {
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.sub_val_val(acc, lit as u16, false);
        self.set_reg_value(RegisterCode::A, result);
    }

    fn sub_a_reg_carry(&mut self, reg: RegisterCode) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let val = self.reg_value(reg) as u16;

        let result = self.sub_val_val(acc, val, self.flag(Flags::Carry));

        self.set_reg_value(RegisterCode::A, result);
    }

    fn sub_a_addr_carry(&mut self, addr: u16) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let operand = self.fetch(addr) as u16;

        let result = self.sub_val_val(acc, operand, self.flag(Flags::Carry));
        self.set_reg_value(RegisterCode::A, result);
    }

    fn sub_a_lit_carry(&mut self, lit: u8) {
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.sub_val_val(acc, lit as u16, self.flag(Flags::Carry));
        self.set_reg_value(RegisterCode::A, result);
    }

    fn and_val_val(&mut self, acc: u8, operand: u8) -> u8 {
        let result = acc & operand;
        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::HalfCarry, true);
        self.set_flag(Flags::AddSubtract, false);
        self.set_flag(Flags::Carry, false);

        let mut parity = 0;
        let mut val = result;
        while val > 0 {
            parity ^= val & 1;
            val >>= 1;
        }
        self.set_flag(Flags::OverflowParity, parity == 0);

        result
    }

    fn and_a_reg(&mut self, reg: RegisterCode) {
        let val = self.reg_value(reg);
        let acc = self.reg_value(RegisterCode::A);

        self.and_val_val(acc, val);
    }

    fn and_a_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let acc = self.reg_value(RegisterCode::A);

        self.and_val_val(acc, val);
    }

    fn and_a_lit(&mut self, val: u8) {
        let acc = self.reg_value(RegisterCode::A);

        self.and_val_val(acc, val);
    }

    fn or_val_val(&mut self, acc: u8, operand: u8) -> u8 {
        let result = acc | operand;
        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::AddSubtract, false);
        self.set_flag(Flags::Carry, false);

        let mut parity = 0;
        let mut val = result;
        while val > 0 {
            parity ^= val & 1;
            val >>= 1;
        }
        self.set_flag(Flags::OverflowParity, parity == 0);

        result
    }

    fn or_a_reg(&mut self, reg: RegisterCode) {
        let val = self.reg_value(reg);
        let acc = self.reg_value(RegisterCode::A);

        self.or_val_val(acc, val);
    }

    fn or_a_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let acc = self.reg_value(RegisterCode::A);

        self.or_val_val(acc, val);
    }

    fn or_a_lit(&mut self, val: u8) {
        let acc = self.reg_value(RegisterCode::A);

        self.or_val_val(acc, val);
    }

    fn xor_val_val(&mut self, acc: u8, operand: u8) -> u8 {
        let result = acc ^ operand;
        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::AddSubtract, false);
        self.set_flag(Flags::Carry, false);

        let mut parity = 0;
        let mut val = result;
        while val > 0 {
            parity ^= val & 1;
            val >>= 1;
        }
        self.set_flag(Flags::OverflowParity, parity == 0);

        result
    }

    fn xor_a_reg(&mut self, reg: RegisterCode) {
        let val = self.reg_value(reg);
        let acc = self.reg_value(RegisterCode::A);

        self.xor_val_val(acc, val);
    }

    fn xor_a_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let acc = self.reg_value(RegisterCode::A);

        self.xor_val_val(acc, val);
    }

    fn xor_a_lit(&mut self, val: u8) {
        let acc = self.reg_value(RegisterCode::A);

        self.xor_val_val(acc, val);
    }

    fn cp_a_val(&mut self, val: u8) -> bool {
        let a = self.reg_value(RegisterCode::A);
        let result = a == val;

        self.set_flag(Flags::Zero, result);
        self.set_flag(Flags::OverflowParity, val > a);

        self.set_flag(Flags::AddSubtract, true);
        self.set_flag(Flags::HalfCarry, (val & 0x0F) > (a & 0x0F));

        result
    }

    fn cp_a_reg(&mut self, reg: RegisterCode) {
        let val = self.reg_value(reg);

        self.cp_a_val(val);
    }

    fn cp_a_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);

        self.cp_a_val(val);
    }

    fn cp_a_lit(&mut self, val: u8) {
        self.cp_a_val(val);
    }

    fn rlca(&mut self) {
        let a = self.reg_value(RegisterCode::A);
        let carry = a >> 7;

        self.set_flag(Flags::Carry, carry > 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::AddSubtract, false);

        self.set_reg_value(RegisterCode::A, ((a << 1) | carry) as u16);
    }

    fn rla(&mut self) {
        let a = self.reg_value(RegisterCode::A);
        let carry_out = a >> 7;
        let carry_in = if self.flag(Flags::Carry) { 1 } else { 0 };

        self.set_flag(Flags::Carry, carry_out > 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::AddSubtract, false);

        self.set_reg_value(RegisterCode::A, ((a << 1) | carry_in) as u16);
    }

    fn rrca(&mut self) {
        let a = self.reg_value(RegisterCode::A);
        let carry = a & 1;

        self.set_flag(Flags::Carry, carry > 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::AddSubtract, false);

        self.set_reg_value(RegisterCode::A, ((a >> 1) | (carry << 7)) as u16);
    }

    fn rra(&mut self) {
        let a = self.reg_value(RegisterCode::A);
        let carry_out = a & 1;
        let carry_in = if self.flag(Flags::Carry) { 1 } else { 0 };

        self.set_flag(Flags::Carry, carry_in > 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::AddSubtract, false);

        self.set_reg_value(RegisterCode::A, ((a >> 1) | (carry_out << 7)) as u16);
    }

    /* ============================ JUMP INSTRUCTIONS ============================= */
    /// Jump to the specified address
    fn jmp(&mut self, addr: u16) {
        self.set_reg_value_16(RegisterCode16::PC, addr);
    }

    /// Jump to the offset specified by the next byte
    fn jmp_rel(&mut self) {
        let addr = self.rel_addr();

        self.set_reg_value_16(RegisterCode16::PC, addr);
    }

    /// Execute a jump to the specified address if the flag matches the condition passed in
    fn jmp_cond(&mut self, addr: u16, flag: Flags, is_set: bool) {
        if self.flag(flag) == is_set {
            self.jmp(addr);
        }
    }
}

/* --------------------------------- TESTING --------------------------------- */

#[cfg(test)]
impl Cpu {
    fn set_pc(&mut self, pc: u16) {
        self.spec_reg[RegisterCode16::PC as usize] = pc as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_cpu() -> Cpu {
        Cpu::new(vec![0xab, 0xcd, 0xef])
    }

    #[test]
    fn test_inc_clock() {
        let mut cpu = Cpu::new(Vec::new());
        assert_eq!(0, cpu.clock());

        cpu.inc_clock();
        assert_eq!(1, cpu.clock());

        cpu.inc_clock_n(5);
        assert_eq!(6, cpu.clock());
    }

    #[test]
    fn test_read_write() {
        let mut cpu = Cpu::new(Vec::new());
        assert_eq!(TriStateLogic::Disconnect, cpu.read_write());

        cpu.set_read_write(TriStateLogic::On);
        assert_eq!(TriStateLogic::On, cpu.read_write());
    }

    #[test]
    fn test_set_reg_a() {
        let mut cpu = Cpu::new(Vec::new());
        cpu.set_reg_value(RegisterCode::A, 10);
        assert_eq!(10, cpu.reg_value(RegisterCode::A));
    }

    #[test]
    fn test_register_16() {
        let mut cpu = Cpu::new(Vec::new());
        cpu.set_reg_value(RegisterCode::B, 0xBB);
        cpu.set_reg_value(RegisterCode::C, 0xCC);
        assert_eq!(0xBBCC, cpu.reg_value_16(RegisterCode16::BC));
    }

    #[test]
    fn test_immediate_addressing() {
        let mut cpu = Cpu::new(vec![0xab, 0xbc, 0xde]);

        assert_eq!(0xab, cpu.imm_addr());
    }

    #[test]
    // note that this uses two bytes and we are in little endian order
    fn test_immediate_extended_addressing() {
        let mut cpu = Cpu::new(vec![0xab, 0xcd, 0xef]);

        assert_eq!(0xcdab, cpu.imm_addr_ex());
    }

    #[test]
    fn test_relative_addressing() {
        let mut cpu = Cpu::new(vec![0xff, 0xff, 0]);
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

        let mut cpu = Cpu::new(vec);
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
    }

    #[test]
    fn test_reg_indirect_addr() {
        let mut cpu = get_cpu();
        cpu.set_reg_value(RegisterCode::H, 0xab);
        cpu.set_reg_value(RegisterCode::L, 0xcd);

        assert_eq!(0xabcd, cpu.reg_indirect_addr(RegisterCode16::HL));
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

        cpu.reg[RegisterCode::Flags as usize] = 0b10; //< AddSubtract is now set
        assert_eq!(true, cpu.flag(Flags::AddSubtract));

        cpu.reg[RegisterCode::Flags as usize] = 0b11110;
        assert_eq!(false, cpu.flag(Flags::Carry));
    }

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
        assert_eq!(false, cpu.flag(Flags::AddSubtract));

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
    fn test_dec_reg() {
        let mut cpu = get_cpu();

        // test normal dec
        cpu.set_reg_value(RegisterCode::A, 1);
        cpu.dec_reg(RegisterCode::A);
        assert_eq!(0, cpu.reg[RegisterCode::A as usize]);
        assert_eq!(false, cpu.flag(Flags::OverflowParity));
        assert_eq!(true, cpu.flag(Flags::Zero));
        assert_eq!(false, cpu.flag(Flags::Sign));
        assert_eq!(false, cpu.flag(Flags::HalfCarry));
        assert_eq!(true, cpu.flag(Flags::AddSubtract));

        // test wrap around
        cpu.set_reg_value(RegisterCode::A, 0);
        cpu.dec_reg(RegisterCode::A);
        assert_eq!(0xFF, cpu.reg[RegisterCode::A as usize]);
        assert_eq!(false, cpu.flag(Flags::OverflowParity));
        assert_eq!(false, cpu.flag(Flags::Zero));
        assert_eq!(true, cpu.flag(Flags::Sign));
        assert_eq!(true, cpu.flag(Flags::HalfCarry));
        assert_eq!(true, cpu.flag(Flags::AddSubtract));

        // test wrap around
        cpu.set_reg_value(RegisterCode::A, 0x80);
        cpu.dec_reg(RegisterCode::A);
        assert_eq!(0x7F, cpu.reg[RegisterCode::A as usize]);
        assert_eq!(true, cpu.flag(Flags::OverflowParity));
        assert_eq!(false, cpu.flag(Flags::Zero));
        assert_eq!(false, cpu.flag(Flags::Sign));
        assert_eq!(true, cpu.flag(Flags::HalfCarry));
        assert_eq!(true, cpu.flag(Flags::AddSubtract));

        // test half adder
        cpu.set_reg_value(RegisterCode::A, 0b1011_0000);
        cpu.dec_reg(RegisterCode::A);
        assert_eq!(0b1010_1111, cpu.reg[RegisterCode::A as usize]);
        assert_eq!(false, cpu.flag(Flags::OverflowParity));
        assert_eq!(false, cpu.flag(Flags::Zero));
        assert_eq!(true, cpu.flag(Flags::Sign));
        assert_eq!(true, cpu.flag(Flags::HalfCarry));
        assert_eq!(true, cpu.flag(Flags::AddSubtract));
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
        assert_eq!(false, cpu.flag(Flags::AddSubtract));

        cpu.set_reg_value(RegisterCode::A, 0b11110110); // -10
        cpu.set_reg_value(RegisterCode::B, 15);
        cpu.add_a_reg(RegisterCode::B);
        assert_eq!(5, cpu.reg_value(RegisterCode::A));
        assert_eq!(false, cpu.flag(Flags::OverflowParity));
        assert_eq!(false, cpu.flag(Flags::Zero));
        assert_eq!(false, cpu.flag(Flags::Sign));
        assert_eq!(true, cpu.flag(Flags::HalfCarry));
        assert_eq!(false, cpu.flag(Flags::AddSubtract));

        cpu.set_reg_value(RegisterCode::A, 0b10011100); // -100
        cpu.set_reg_value(RegisterCode::B, 15);
        cpu.add_a_reg(RegisterCode::B);
        assert_eq!(-85, cpu.reg_value(RegisterCode::A) as i8);
        assert_eq!(false, cpu.flag(Flags::OverflowParity));
        assert_eq!(false, cpu.flag(Flags::Zero));
        assert_eq!(true, cpu.flag(Flags::Sign));
        assert_eq!(true, cpu.flag(Flags::HalfCarry));
        assert_eq!(false, cpu.flag(Flags::AddSubtract));

        cpu.set_reg_value(RegisterCode::A, 0x7F); // 127
        cpu.set_reg_value(RegisterCode::B, 1); // -> should wrap around and overflow
        cpu.add_a_reg(RegisterCode::B);
        assert_eq!(-128, cpu.reg_value(RegisterCode::A) as i8);
        assert_eq!(true, cpu.flag(Flags::OverflowParity));
        assert_eq!(false, cpu.flag(Flags::Zero));
        assert_eq!(true, cpu.flag(Flags::Sign));
        assert_eq!(true, cpu.flag(Flags::HalfCarry));
        assert_eq!(false, cpu.flag(Flags::AddSubtract));
    }

    #[test]
    fn test_sub_val_val() {
        let mut cpu = get_cpu();

        let result = cpu.sub_val_val(127, 0xC0, false);
        assert_eq!(191, result);
        assert_eq!(true, cpu.flag(Flags::OverflowParity));

        let result = cpu.sub_val_val(127, 5, false);
        assert_eq!(122, result);
        assert_eq!(false, cpu.flag(Flags::OverflowParity));

        let result = cpu.sub_val_val(1, 0xFF, false);
        assert_eq!(2, result);
        assert_eq!(false, cpu.flag(Flags::Zero));

        let result = cpu.sub_val_val(0xC0, 0xFF, false);
        assert_eq!(-63, result as i8);
        assert_eq!(false, cpu.flag(Flags::Zero));
    }

    #[test]
    fn test_sub_val_val_carry() {
        let mut cpu = get_cpu();

        let result = cpu.sub_val_val(127, 0xC0, true);
        assert_eq!(191 - 1, result);
        assert_eq!(true, cpu.flag(Flags::OverflowParity));

        let result = cpu.sub_val_val(127, 5, true);
        assert_eq!(121, result);
        assert_eq!(false, cpu.flag(Flags::OverflowParity));

        let result = cpu.sub_val_val(1, 0xFF, true);
        assert_eq!(1, result);
        assert_eq!(false, cpu.flag(Flags::OverflowParity));
    }
}
