#![allow(dead_code)]
extern crate bus;

use bus::{bus::Bus, MutRef};
use opcode::Opcode;
use std::io::Write;
use std::{mem, rc::Rc};

mod bits;
mod extended;
mod opcode;
#[cfg(test)]
mod tests;

const RESET: bool = false;
const SET: bool = true;

// macro to set the flag on a cpu
#[macro_export]
macro_rules! flag {
    ($cpu:expr; set $flag:expr) => {
        $cpu.set_flag($flag, true);
    };
    ($cpu:expr; unset $flag:expr) => {
        $cpu.set_flag($flag, false);
    };
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
    I,
    R,
    IXh,
    IXl,
    IYh,
    IYl,
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
    AF,
}

#[repr(u8)]
pub enum Flags {
    Carry = 0,
    Subtract,
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
    clock_queue:          i64,
    iff1:                 bool,
    iff2:                 bool,
    interrupt_count:      u8,
    reg:                  [u16; 8], // contains A, F, B, C, D, E, H, L
    alt_reg:              [u16; 8], // contains alternate A, F, B, C, D, E, H, L
    spec_reg:             [u32; 6], // contains I, R, IX, IY, PC, SP 
    halted:               bool,
    pub reset_req:        bool,
    data_bus:             MutRef<Bus>,
    io_bus:               MutRef<Bus>,
    pub nomask_interrupt: bool,
    pub mask_interrupt:   bool,
}

impl Cpu {
    #[rustfmt::skip]
    /// TODO: set buffer to point to a vector of binary file data 
    pub fn new(data: &MutRef<Bus>, io: &MutRef<Bus>) -> Cpu {
        Cpu {
            clock:                0,
            clock_queue:          0,
            iff1:                 false,
            iff2:                 false,
            reg:                  [0; 8],
            alt_reg:              [0; 8],
            spec_reg:             [0; 6],
            halted:               false,
            reset_req:            false,
            interrupt_count:      0,
            data_bus:             Rc::clone(data),
            io_bus:               Rc::clone(io),
            nomask_interrupt:     false,
            mask_interrupt:       false,

        }
    }

    pub fn with_pc(data: &MutRef<Bus>, io: &MutRef<Bus>, pc: u16) -> Cpu {
        let mut cpu = Cpu::new(data, io);
        cpu.set_reg_value_16(RegisterCode16::PC, pc);

        cpu
    }

    //
    // getters
    //
    #[inline]
    pub fn clock(&self) -> u64 {
        self.clock
    }

    #[inline]
    pub fn get_pc(&self) -> u16 {
        self.reg_value_16(RegisterCode16::PC)
    }

    #[inline]
    fn set_pc(&mut self, val: u16) {
        self.set_reg_value_16(RegisterCode16::PC, val);
    }

    pub fn reg_value(&self, code: RegisterCode) -> u8 {
        use RegisterCode::*;
        match code {
            I => self.reg_value_16(RegisterCode16::I) as u8,
            R => self.reg_value_16(RegisterCode16::R) as u8,
            IXh => (self.reg_value_16(RegisterCode16::IX) >> 8) as u8,
            IYh => (self.reg_value_16(RegisterCode16::IX) >> 8) as u8,
            IXl => (self.reg_value_16(RegisterCode16::IX) & 0x0F) as u8,
            IYl => (self.reg_value_16(RegisterCode16::IY) & 0x0F) as u8,
            _ => self.reg[code as usize] as u8,
        }
    }

    /// set the value for the register pair.  The value will be split into
    /// the upper and lower bytes.  In general, if the register pair is made up
    /// of two 8 bit registers (i.e. HL, BC, DE, AF) the upper byte is placed
    /// in the first register.
    ///
    /// # Example
    ///
    /// ```
    /// cpu.set_reg_value_16(RegisterCRegisterCode16::HL, 0xF00F);
    /// /*
    /// register H == 0xF0
    /// register L == 0x0F
    /// */
    /// ```
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
            RegisterCode16::AF => {
                reg_high = RegisterCode::A;
                reg_low = RegisterCode::Flags;
            }

            _ => {
                self.spec_reg[code as usize] = val as u32;
                return;
            }
        }

        self.reg[reg_high as usize] = (val >> 8) & 0xFF;
        self.reg[reg_low as usize] = val & 0xFF;
    }

    pub fn reg_value_16(&self, code: RegisterCode16) -> u16 {
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
            RegisterCode16::AF => {
                reg_high = RegisterCode::A;
                reg_low = RegisterCode::Flags;
            }
            _ => return self.spec_reg[code as usize] as u16,
        }

        let high_byte = self.reg[reg_high as usize];
        let low_byte = self.reg[reg_low as usize];

        (high_byte << 8) | (low_byte & 0xFF)
    }

    #[inline]
    fn set_flag(&mut self, f: Flags, set: bool) {
        let mut flag = self.reg_value(RegisterCode::Flags);
        if set {
            flag |= 1 << f as u8;
        } else {
            flag &= !(1 << f as u8);
        }

        self.set_reg_value(RegisterCode::Flags, flag as u16);
    }

    /// Get the value of a flag
    #[inline]
    pub fn flag(&self, f: Flags) -> bool {
        // get the registers value.  shift that right so that the least significant bit is the flag.
        //  & with 1 to remove all other values.  return true if result is 1
        ((self.reg[RegisterCode::Flags as usize]) >> f as u8) & 1 == 1
    }

    pub fn tick_clock(&mut self, n: u64) {
        if self.clock_queue > 0 {
            self.clock += self.clock_queue as u64;
            self.clock_queue = 0;
        }
        self.clock += n
    }

    #[inline]
    fn queue_clock_tick(&mut self, n: i64) {
        self.clock_queue += n;
    }

    fn set_reg_value(&mut self, code: RegisterCode, value: u16) {
        use RegisterCode::*;
        match code {
            I => self.set_reg_value_16(RegisterCode16::I, value),
            R => self.set_reg_value_16(RegisterCode16::R, value),
            IXh => {
                let val = self.reg_value_16(RegisterCode16::IX);
                self.set_reg_value_16(RegisterCode16::IX, (val & 0x0F) | value << 8);
            }

            IXl => {
                let val = self.reg_value_16(RegisterCode16::IX);
                self.set_reg_value_16(RegisterCode16::IX, (val & 0xF0) | value);
            }
            IYh => {
                let val = self.reg_value_16(RegisterCode16::IY);
                self.set_reg_value_16(RegisterCode16::IY, (val & 0x0F) | value << 8);
            }

            IYl => {
                let val = self.reg_value_16(RegisterCode16::IY);
                self.set_reg_value_16(RegisterCode16::IY, (val & 0xF0) | value);
            }

            _ => self.reg[code as usize] = value,
        }
    }

    /// Push the value onto the stack.
    ///
    /// This function decrements the stack pointer and pushes the value onto the stack
    fn push(&mut self, value: u8) {
        let mut sp = self.reg_value_16(RegisterCode16::SP);

        sp = if sp == 0 { 0xFFFF } else { sp - 1 };

        self.set_reg_value_16(RegisterCode16::SP, sp);

        self.store(sp, value);
    }

    /// Pop the value off the stack
    ///
    /// This function pops the value off of the stack and increments the stack pointer.
    fn pop(&mut self) -> u8 {
        let val = self.fetch(self.reg_value_16(RegisterCode16::SP));
        let mut pc = self.reg_value_16(RegisterCode16::SP);
        pc = ((pc as u32 + 1) % 0xFFFF) as u16;
        self.set_reg_value_16(RegisterCode16::SP, pc);

        val
    }

    /// Push the contents of the PC onto the stack
    fn push_pc(&mut self) {
        let pc = self.reg_value_16(RegisterCode16::PC);
        self.push((pc >> 8) as u8 & 0xFF);
        self.push(pc as u8 & 0xFF);
    }

    /// Pop the contents of the PC off of the stack and places them into the PC register
    fn pop_pc(&mut self) {
        let low = self.pop() as u16;
        let high = self.pop() as u16;
        let val = (high << 8) | low;

        self.set_reg_value_16(RegisterCode16::PC, val);
    }

    fn fetch(&self, addr: u16) -> u8 {
        self.data_bus.borrow().cpu_read(addr).expect(&format!(
            "Attempted to fetch value at {} but found nothing!",
            addr,
        ))
    }

    fn store(&mut self, addr: u16, val: u8) {
        self.data_bus.borrow_mut().cpu_write(addr, val);
    }

    pub fn is_halted(&self) -> bool {
        self.halted
    }

    pub fn reset_halt(&mut self) {
        self.halted = false;
    }

    pub fn log(&self, mut log: impl Write) -> std::io::Result<()> {
        use RegisterCode::*;
        use RegisterCode16::*;
        writeln!(
            log,
            "A:  0x{:02x}    F:  0x{:02x}",
            self.reg_value(A),
            self.reg_value(Flags),
        )?;

        writeln!(
            log,
            "B:  0x{:02x}    C:  0x{:02x}",
            self.reg_value(B),
            self.reg_value(C),
        )?;

        writeln!(
            log,
            "D:  0x{:02x}    E:  0x{:02x}",
            self.reg_value(D),
            self.reg_value(E),
        )?;

        writeln!(
            log,
            "H:  0x{:02x}    L:  0x{:02x}",
            self.reg_value(H),
            self.reg_value(L),
        )?;

        writeln!(
            log,
            "IX: 0x{:04x}  IY: 0x{:04x}",
            self.reg_value_16(IX),
            self.reg_value_16(IY),
        )?;

        writeln!(
            log,
            "PC: 0x{:04x}  SP: 0x{:04x}",
            self.reg_value_16(PC),
            self.reg_value_16(SP),
        )?;

        writeln!(log, "Halted? {}", self.halted)?;

        Ok(())
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

    fn parity_even(mut val: u32) -> bool {
        let mut result = 0;
        while val > 0 {
            result ^= val & 1;
            val >>= 1;
        }

        // Parity is even the result is 0
        //   (e.g.) 101101 would be 1 xor 1 xor 1 xor 1
        //          all 1's cancel out
        result == 0
    }

    /// Get the next byte from the current position of program counter.
    ///
    /// This function does not increment the program counter after retrieval
    pub fn next_byte_no_inc(&self) -> u8 {
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
    /// (the program counter is currently pointing to the instruction after the one using
    /// relative addressing). This means that from the opcode using relative addressing we
    /// can move +129 to -126 bytes
    fn rel_addr(&mut self) -> u16 {
        // cast to signed 16 bit
        let byte = self.next_byte() as i8 as i32; // cast to i8 to convert to negative.
                                                  // cast to i16 so we can add to the pc value

        // cast to signed 16 bit
        let cur_pc = self.get_pc() as i16 as i32;

        // cast back to unsigned 16 bit.
        // this accurately performs the addition with a potentially negative `byte`
        (cur_pc + byte) as u16
    }

    /// Extended Addressing
    ///
    /// Returns the address specified by the next _two_ bytes of memory
    /// starting at the current program counter.
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
            panic!(
                "Attempting to use register '{:?}' to perform Indexed Addressing. \
                 Only Registers 'IX' and 'IY' are able to be used for this addressing mode!",
                register
            );
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
    fn indirect_reg_addr(&mut self, register: RegisterCode16) -> u16 {
        self.reg_value_16(register)
    }

    /* --------------------------------- LOOP --------------------------------- */
    /// Perform the next operation
    ///
    /// Performs the next operation taken from the buffer supplied to the cpu.  
    /// This function will handle changing all internal Cpu values and will write to
    /// any necessary busses
    ///
    /// Returns the number of T-state the operation took
    pub fn do_operation(&mut self) -> u64 {
        let initial_clock = self.clock;
        // print!("PC: {}  |  ", self.reg_value_16(RegisterCode16::PC));
        // println!("Byte 0x{:x}", opcode);

        if self.reset_req {
            self.reset_req = false;
            self.halted = false;
            self.reset();
        } else if self.nomask_interrupt {
            self.nomask_interrupt = false;
            self.halted = false;
            self.interrupt_nomask();
        } else if self.mask_interrupt && self.interrupt_count == 0 {
            self.mask_interrupt = false;
            self.halted = false;
            self.interrupt_1();
        } else if self.halted {
            Opcode::operate(self, opcode::Opcode::NoOp);
        } else {
            let opcode = self.next_byte();
            Opcode::operate_u8(self, opcode);
        }

        if self.interrupt_count > 0 {
            self.interrupt_count -= 1;
        }

        self.clock - initial_clock
    }
}

/* ==========================================================================
 * ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^Addressing^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
 *
 *
 * vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvOperationsvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
 * =========================================================================*/
impl Cpu {
    /// No Op
    fn noop(&mut self) {
        //println!("No-Op");
        self.tick_clock(4);
    }

    /// load the dest reg with the src reg value
    /// LD A, B
    fn ld_reg_reg(&mut self, dst: RegisterCode, src: RegisterCode) {
        let val = self.reg_value(src);
        self.set_reg_value(dst, val as u16);

        // if the opcode is:
        //      Ld A, I
        //      Ld A, R
        // then iff2 is copied to the parity flag
        match (src, dst) {
            (RegisterCode::A, RegisterCode::I) => {
                self.set_flag(Flags::OverflowParity, self.iff2);
            }
            (RegisterCode::A, RegisterCode::R) => {
                self.set_flag(Flags::OverflowParity, self.iff2);
            }
            _ => {}
        }
        self.tick_clock(4);
    }

    /// load the dest reg with the literal value
    /// LD A, 5
    fn ld_reg_lit(&mut self, dst: RegisterCode) {
        let literal = self.next_byte();

        //println!("Loading {:?} with {}", dst, literal);

        self.reg[dst as usize] = literal as u16;
        self.tick_clock(7);
    }

    /// load the dest reg with value pointed to by the address passed in
    fn ld_reg_addr(&mut self, dst: RegisterCode, addr: u16) {
        let value = self.fetch(addr);

        self.set_reg_value(dst, value as u16);

        self.tick_clock(7);
    }

    fn ld_addr_reg(&mut self, addr: u16, src: RegisterCode) {
        let value = self.reg[src as usize];

        self.store(addr, value as u8);
        self.tick_clock(7);
    }

    fn ld_addr_lit(&mut self, addr: u16, lit: u8) {
        self.store(addr, lit);
        self.tick_clock(10);
    }

    fn ld_reg16_lit(&mut self, reg: RegisterCode16, lit: u16) {
        self.set_reg_value_16(reg, lit);
        self.tick_clock(10);
    }

    fn ld_reg16_addr(&mut self, dst: RegisterCode16, addr: u16) {
        let addr_low = self.fetch(addr) as u16;
        let addr_high = self.fetch(addr + 1) as u16;

        self.set_reg_value_16(dst, (addr_high << 8) | addr_low);
        self.tick_clock(16);
    }

    fn ld_reg16_reg16(&mut self, dst: RegisterCode16, src: RegisterCode16) {
        let val = self.reg_value_16(src);
        self.set_reg_value_16(dst, val);
        self.tick_clock(6);
    }

    fn ld_addr_reg16(&mut self, addr: u16, src: RegisterCode16) {
        let val = self.reg_value_16(src);

        self.store(addr, (val & 0xFF) as u8);
        self.store(addr + 1, ((val >> 8) & 0xFF) as u8);
        self.tick_clock(16);
    }

    fn push_reg16(&mut self, src: RegisterCode16) {
        let val = self.reg_value_16(src);

        // high order byte is stored first
        self.push(((val >> 8) & 0xFF) as u8);
        self.push((val & 0xFF) as u8);

        self.tick_clock(11);
    }

    fn pop_reg16(&mut self, dst: RegisterCode16) {
        let low = self.pop() as u16;
        let high = self.pop() as u16;
        let val = (high << 8) | low as u16;

        self.set_reg_value_16(dst, val);
        self.tick_clock(10);
    }

    /* ---------------------- Incrementing ----------------- */
    /// increment register and set appropriate flags
    fn inc_reg(&mut self, reg: RegisterCode) {
        let mut val = self.reg_value(reg) as u16;

        self.set_flag(Flags::OverflowParity, val == 0x7F);
        self.set_flag(Flags::HalfCarry, val & 0b1111 == 0b1111);

        // increment and wrap around to 0
        val += 1;
        val %= 0xFF + 1;

        self.set_flag(Flags::Sign, (val & 0x80) > 0);
        self.set_flag(Flags::Zero, val == 0);
        self.set_flag(Flags::Subtract, false);

        self.set_reg_value(reg, val);
        self.tick_clock(4);
    }

    /// increment the value stored at an address and set appropriate flags
    fn inc_addr(&mut self, addr: u16) {
        let mut val = self.fetch(addr) as u16;

        self.set_flag(Flags::OverflowParity, val == 0x7F);
        self.set_flag(Flags::HalfCarry, val & 0b1111 == 0b1111);

        // increment and wrap around to 0
        val += 1;
        val %= 0xFF + 1;

        self.set_flag(Flags::Sign, (val as i8) < 0);
        self.set_flag(Flags::Zero, val == 0);
        self.set_flag(Flags::Subtract, false);

        self.store(addr, val as u8);
        self.tick_clock(11);
    }

    /// Increment a 16 bit register or register pair.  
    ///
    /// This function does not change any flags even on overflow/zero/etc.
    fn inc_reg16(&mut self, reg: RegisterCode16) {
        let mut val = self.reg_value_16(reg) as u32;
        val += 1;
        val %= 0xFFFF + 1;

        self.set_reg_value_16(reg, val as u16);
        self.tick_clock(6);
    }

    /// Decrement the register by 1
    fn dec_reg(&mut self, reg: RegisterCode) {
        let mut val = self.reg[reg as usize];

        self.set_flag(Flags::Subtract, true);
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
        self.tick_clock(4);
    }

    /// Decrement the value at address by 1.  Sets flags
    ///
    /// # Parameters
    ///
    /// *addr* - the address of the value
    fn dec_addr(&mut self, addr: u16) {
        let mut val = self.fetch(addr);

        self.set_flag(Flags::Subtract, true);
        self.set_flag(Flags::OverflowParity, val == 0x80);
        // set half carry if there is a borrow from bit 4 to 3
        self.set_flag(Flags::HalfCarry, (val & 0xF) == 0);

        if val == 0 {
            // wrap around
            val = 0xFF;
        } else {
            val -= 1;
        }

        // set the sign flag is the first bit is set
        self.set_flag(Flags::Sign, val > 0x80);
        self.set_flag(Flags::Zero, val == 0);

        self.store(addr, val);
        self.tick_clock(11);
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
        self.tick_clock(6);
    }

    /// Add 8 bit values
    ///
    /// This function sets the necessary flags for the addition and returns the result
    fn add_val_val(&mut self, acc: u16, operand: u16, carry: bool) -> u16 {
        let carry = if carry && self.flag(Flags::Carry) {
            1
        } else {
            0
        };

        // add the lower 4 bits of the two operands.
        // If the result > 4 bits then we have half carry
        self.set_flag(
            Flags::HalfCarry,
            (operand & 0x0F) + (acc & 0x0F) + carry > 0x0F,
        );

        let result = acc + operand + carry;
        // set the carry flag before we wrap around.  We have a carry if we wrapped around to 0
        // (i.e.) went 0xFF to 0
        self.set_flag(Flags::Carry, result > 0xFF);

        let result: u16 = result % (0x100);

        // set overflow flag if:
        // 1). the signs are the same for the number being added
        // 2). the result sign differs from the operand signs
        self.set_flag(
            Flags::OverflowParity,
            acc >> 7 == operand >> 7 && acc >> 7 != result >> 7,
        );
        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::Subtract, false);

        result
    }

    /// Add the contents of the specified register to the accumlator register
    fn add_a_reg(&mut self, reg: RegisterCode) {
        let operand = self.reg_value(reg) as u16;
        let acc = self.reg_value(RegisterCode::A) as u16;

        //println!("Adding {:?} to {:?}", operand, acc);

        let result = self.add_val_val(acc, operand, false);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(4);
    }

    fn add_a_addr(&mut self, addr: u16) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let operand = self.fetch(addr) as u16;

        let result = self.add_val_val(acc, operand, false);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(7);
    }

    fn add_a_lit(&mut self, lit: u8) {
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.add_val_val(acc, lit as u16, false);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(7);
    }

    /// Add the contents of the specified register to the accumlator register
    fn add_a_reg_carry(&mut self, reg: RegisterCode) {
        let operand = self.reg_value(reg) as u16;
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.add_val_val(acc, operand, true);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(4);
    }

    fn add_a_addr_carry(&mut self, addr: u16) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let operand = self.fetch(addr) as u16;

        let result = self.add_val_val(acc, operand, true);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(7);
    }

    fn add_a_lit_carry(&mut self, lit: u8) {
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.add_val_val(acc, lit as u16, true);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(7);
    }

    fn add_reg16_reg16(&mut self, to: RegisterCode16, operand: RegisterCode16) {
        let add_to = self.reg_value_16(to) as u32;
        let with = self.reg_value_16(operand) as u32;

        let mut result = add_to + with;
        self.set_flag(Flags::Carry, result > 0xFFFF);
        result %= 0xFFFF;

        self.set_flag(Flags::Subtract, false);
        // set the flag with the first 3 nibble overlow into the highnibble
        self.set_flag(Flags::HalfCarry, (add_to & 0xFFF) + (with & 0xFFF) > 0xFFF);

        self.set_reg_value_16(to, result as u16);
        self.tick_clock(11);
    }

    fn adc_reg16_reg16(&mut self, src: RegisterCode16, op: RegisterCode16) {
        let add_to = self.reg_value_16(src) as u32;
        let with = self.reg_value_16(op) as u32;
        let carry = if self.flag(Flags::Carry) { 1 } else { 0 } as u32;

        let mut result = add_to + with + carry;
        self.set_flag(Flags::Carry, result > 0xFFFF);
        result %= 0xFFFF;

        self.set_flag(Flags::Sign, result >= 0x8000);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(
            Flags::HalfCarry,
            (add_to & 0x0FFF) + (with & 0x0FFF) + carry > 0x0FFF,
        );
        self.set_flag(
            Flags::OverflowParity,
            add_to >> 15 == (carry + with) >> 15 && add_to >> 15 != result >> 15,
        );
        self.set_flag(Flags::Subtract, false);

        self.set_reg_value_16(src, result as u16);
        self.tick_clock(15);
    }

    fn sbc_reg16_reg16(&mut self, to: RegisterCode16, operand: RegisterCode16) {
        let src = self.reg_value_16(to);
        let op = self.reg_value_16(operand);
        let carry = if self.flag(Flags::Carry) { 1 } else { 0 };

        let result = if src >= op + carry {
            self.set_flag(Flags::Carry, false);
            src - op - carry
        } else {
            self.set_flag(Flags::Carry, true);
            0xFFFF - op - carry + src + 1
        };

        self.set_flag(Flags::Sign, result >= 0x8000);
        self.set_flag(Flags::Zero, result == 0);
        // set the half flag if we are subtracting a larger number than is in the lower 3 nibbles
        // of the operand + carry
        self.set_flag(Flags::HalfCarry, src & 0xFFF < (op & 0xFFF) + carry);
        self.set_flag(
            Flags::OverflowParity,
            src & 0x8000 != (op + carry) & 0x8000 && src & 0x8000 != result & 0x8000,
        );
        self.set_flag(Flags::Subtract, true);

        self.set_reg_value_16(to, result);
        self.tick_clock(15);
    }

    /// Subtract the value in the operand from the Accumulator
    ///
    /// This function sets all of the necessary flags the subtraction and returns the result
    /// to the caller.
    ///
    /// # Parameters
    /// *acc* - the value being subtracted __from__
    /// *operand* - the value that is being subtracted
    /// *carry* - `true` if the carry flag should also be used in the subtraction
    fn sub_val_val(&mut self, acc: u16, operand: u16, carry: bool) -> u16 {
        let carry = if carry && self.flag(Flags::Carry) {
            1
        } else {
            0
        };

        let result = if acc >= (operand + carry) % 0x100 {
            flag!(self; unset Flags::Carry);
            acc - ((operand + carry) % 0x100)
        } else {
            flag!(self; set Flags::Carry);
            let remainder = (carry + operand) - acc;
            0xFF - remainder + 1
        };

        //println!("Subtracting... {} - {} = {}", acc, operand, result);

        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::Subtract, true);
        // set half carry if the lower 4 bits need to borrow from the upper 4 bits
        // this happends if subtracting a value that is larger
        self.set_flag(Flags::HalfCarry, acc & 0x0F < (operand + carry) & 0x0F);
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
        self.tick_clock(4);
    }

    fn sub_a_addr(&mut self, addr: u16) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let operand = self.fetch(addr) as u16;

        let result = self.sub_val_val(acc, operand, false);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(7);
    }

    fn sub_a_lit(&mut self, lit: u8) {
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.sub_val_val(acc, lit as u16, false);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(7);
    }

    fn sub_a_reg_carry(&mut self, reg: RegisterCode) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let val = self.reg_value(reg) as u16;

        let result = self.sub_val_val(acc, val, true);

        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(4);
    }

    fn sub_a_addr_carry(&mut self, addr: u16) {
        let acc = self.reg_value(RegisterCode::A) as u16;
        let operand = self.fetch(addr) as u16;

        let result = self.sub_val_val(acc, operand, true);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(7);
    }

    fn sub_a_lit_carry(&mut self, lit: u8) {
        let acc = self.reg_value(RegisterCode::A) as u16;

        let result = self.sub_val_val(acc, lit as u16, true);
        self.set_reg_value(RegisterCode::A, result);
        self.tick_clock(7);
    }

    fn and_val_val(&mut self, acc: u8, operand: u8) -> u8 {
        let result = acc & operand;
        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::HalfCarry, true);
        self.set_flag(Flags::Subtract, false);
        self.set_flag(Flags::Carry, false);

        self.set_flag(Flags::OverflowParity, Cpu::parity_even(result as u32));

        result
    }

    fn and_a_reg(&mut self, reg: RegisterCode) {
        let val = self.reg_value(reg);
        let acc = self.reg_value(RegisterCode::A);

        let result = self.and_val_val(acc, val);
        self.set_reg_value(RegisterCode::A, result as u16);

        self.tick_clock(4);
    }

    fn and_a_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let acc = self.reg_value(RegisterCode::A);

        let result = self.and_val_val(acc, val);
        self.set_reg_value(RegisterCode::A, result as u16);

        self.tick_clock(7);
    }

    fn and_a_lit(&mut self, val: u8) {
        let acc = self.reg_value(RegisterCode::A);

        let result = self.and_val_val(acc, val);
        self.set_reg_value(RegisterCode::A, result as u16);

        self.tick_clock(7);
    }

    fn or_val_val(&mut self, acc: u8, operand: u8) -> u8 {
        let result = acc | operand;
        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::Subtract, false);
        self.set_flag(Flags::Carry, false);

        // let mut parity = 0;
        // let mut val = result;
        // while val > 0 {
        //     parity ^= val & 1;
        //     val >>= 1;
        // }
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(result as u32));

        result
    }

    fn or_a_reg(&mut self, reg: RegisterCode) {
        let val = self.reg_value(reg);
        let acc = self.reg_value(RegisterCode::A);

        let result = self.or_val_val(acc, val);
        self.set_reg_value(RegisterCode::A, result as u16);
        self.tick_clock(4);
    }

    fn or_a_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let acc = self.reg_value(RegisterCode::A);

        let result = self.or_val_val(acc, val);
        self.set_reg_value(RegisterCode::A, result as u16);
        self.tick_clock(7);
    }

    fn or_a_lit(&mut self, val: u8) {
        let acc = self.reg_value(RegisterCode::A);

        let result = self.or_val_val(acc, val);

        self.set_reg_value(RegisterCode::A, result as u16);
        self.tick_clock(7);
    }

    fn xor_val_val(&mut self, acc: u8, operand: u8) -> u8 {
        let result = acc ^ operand;
        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::Subtract, false);
        self.set_flag(Flags::Carry, false);

        self.set_flag(Flags::OverflowParity, Cpu::parity_even(result as u32));

        result
    }

    fn xor_a_reg(&mut self, reg: RegisterCode) {
        let val = self.reg_value(reg);
        let acc = self.reg_value(RegisterCode::A);

        let result = self.xor_val_val(acc, val);
        self.set_reg_value(RegisterCode::A, result as u16);

        self.tick_clock(4);
    }

    fn xor_a_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let acc = self.reg_value(RegisterCode::A);

        let result = self.xor_val_val(acc, val);
        self.set_reg_value(RegisterCode::A, result as u16);

        self.tick_clock(7);
    }

    fn xor_a_lit(&mut self, val: u8) {
        let acc = self.reg_value(RegisterCode::A);

        let result = self.xor_val_val(acc, val);
        self.set_reg_value(RegisterCode::A, result as u16);

        self.tick_clock(7);
    }

    fn cp_a_val(&mut self, val: u8) -> bool {
        let a = self.reg_value(RegisterCode::A);
        let result = if val > a { 0xFF - val + a + 1 } else { a - val };

        //println!("Comparing {}, {}", a, val);

        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::OverflowParity, val > a);
        self.set_flag(Flags::Subtract, true);
        self.set_flag(Flags::HalfCarry, (val & 0x0F) > (a & 0x0F));
        self.set_flag(Flags::Carry, val > a);
        self.set_flag(Flags::Sign, result >= 0x80);

        result == 0
    }

    fn cp_a_reg(&mut self, reg: RegisterCode) {
        let val = self.reg_value(reg);

        self.cp_a_val(val);
        self.tick_clock(4);
    }

    fn cp_a_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);

        self.cp_a_val(val);
        self.tick_clock(7);
    }

    fn cp_a_lit(&mut self, val: u8) {
        self.cp_a_val(val);
        self.tick_clock(7);
    }

    fn rot_l_carry(&self, val: u8) -> (u8, bool) {
        let carry = val >> 7;
        let output = (val << 1) | carry;

        (output, carry > 0)
    }

    fn rlc_reg(&mut self, src: RegisterCode) {
        let val = self.reg_value(src);

        let (output, carry) = self.rot_l_carry(val);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);
        self.set_reg_value(src, output as u16);

        self.tick_clock(8);
    }

    fn rlc_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);

        let (output, carry) = self.rot_l_carry(val);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);
        self.store(addr, output);

        self.tick_clock(15);
    }

    fn rlca(&mut self) {
        let a = self.reg_value(RegisterCode::A);

        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::Subtract, false);

        let (val, carry) = self.rot_l_carry(a);

        self.set_flag(Flags::Carry, carry);
        self.set_reg_value(RegisterCode::A, val as u16);
        self.tick_clock(4);
    }

    fn rot_l(&self, val: u8) -> (u8, bool) {
        let carry = if self.flag(Flags::Carry) { 1 } else { 0 };

        let output = (val << 1) | carry;
        let carry = val >> 7;

        (output, carry > 0)
    }

    fn rl_reg(&mut self, src: RegisterCode) {
        let reg = self.reg_value(src);
        let (output, carry) = self.rot_l(reg);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.set_reg_value(src, output as u16);
        self.tick_clock(8);
    }

    fn rl_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);

        let (output, carry) = self.rot_l(val);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);
        self.store(addr, output);

        self.tick_clock(15);
    }

    fn rla(&mut self) {
        let a = self.reg_value(RegisterCode::A);

        let (val, carry) = self.rot_l(a);

        self.set_flag(Flags::Carry, carry);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::Subtract, false);

        self.set_reg_value(RegisterCode::A, val as u16);
        self.tick_clock(4);
    }

    fn sl(&mut self, val: u8, bit1: u8) -> (u8, bool) {
        let carry = val >> 7;
        let output = (val << 1) | (bit1 & 1);

        (output, carry > 0)
    }

    fn sla_reg(&mut self, src: RegisterCode) {
        let reg = self.reg_value(src);
        let (output, carry) = self.sl(reg, 0);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.set_reg_value(src, output as u16);
        self.tick_clock(8);
    }

    fn sla_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let (output, carry) = self.sl(val, 0);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.store(addr, output);
        self.tick_clock(15);
    }

    fn sll_reg(&mut self, src: RegisterCode) {
        let reg = self.reg_value(src);
        let (output, carry) = self.sl(reg, 1);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.set_reg_value(src, output as u16);
        self.tick_clock(8);
    }

    fn sll_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let (output, carry) = self.sl(val, 1);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.store(addr, output);
        self.tick_clock(15);
    }

    fn rot_r_carry(&self, val: u8) -> (u8, bool) {
        let carry = val & 1;
        let output = (carry << 7) | (val >> 1);

        (output, carry > 0)
    }

    fn rrc_reg(&mut self, src: RegisterCode) {
        let reg = self.reg_value(src);
        let (output, carry) = self.rot_r_carry(reg);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.set_reg_value(src, output as u16);
        self.tick_clock(8);
    }

    fn rrc_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);

        let (output, carry) = self.rot_r_carry(val);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);
        self.store(addr, output);

        self.tick_clock(15);
    }

    fn rrca(&mut self) {
        let a = self.reg_value(RegisterCode::A);
        let carry = a & 1;

        self.set_flag(Flags::Carry, carry > 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::Subtract, false);

        self.set_reg_value(RegisterCode::A, ((a >> 1) | (carry << 7)) as u16);
        self.tick_clock(4);
    }

    fn rot_r(&self, val: u8) -> (u8, bool) {
        let carry = if self.flag(Flags::Carry) { 1 } else { 0 };
        let output = (carry << 7) | (val >> 1);
        let carry = val & 1;

        (output, carry > 0)
    }

    fn rr_reg(&mut self, src: RegisterCode) {
        let reg = self.reg_value(src);
        let (output, carry) = self.rot_r(reg);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.set_reg_value(src, output as u16);
        self.tick_clock(8);
    }

    fn rr_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);

        let (output, carry) = self.rot_r(val);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);
        self.store(addr, output);

        self.tick_clock(15);
    }

    fn rra(&mut self) {
        let a = self.reg_value(RegisterCode::A);
        let carry_out = a & 1;
        let carry_in = if self.flag(Flags::Carry) { 1 } else { 0 };

        self.set_flag(Flags::Carry, carry_in > 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::Subtract, false);

        self.set_reg_value(RegisterCode::A, ((a >> 1) | (carry_out << 7)) as u16);
        self.tick_clock(4);
    }

    fn sr(&self, val: u8, retain_bit7: bool) -> (u8, bool) {
        let bit7 = if retain_bit7 { (val >> 7) & 1 } else { 0 };
        let output = (bit7 << 7) | (val >> 1);
        let carry = val & 1;

        (output, carry > 0)
    }

    fn sra_reg(&mut self, src: RegisterCode) {
        let reg = self.reg_value(src);
        let (output, carry) = self.sr(reg, true);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.set_reg_value(src, output as u16);
        self.tick_clock(8);
    }

    fn sra_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let (output, carry) = self.sr(val, true);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.store(addr, output);
        self.tick_clock(15);
    }

    fn srl_reg(&mut self, src: RegisterCode) {
        let reg = self.reg_value(src);
        let (output, carry) = self.sr(reg, false);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.set_reg_value(src, output as u16);
        self.tick_clock(8);
    }

    fn srl_addr(&mut self, addr: u16) {
        let val = self.fetch(addr);
        let (output, carry) = self.sr(val, false);

        self.set_flag(Flags::Sign, output >= 0x80);
        self.set_flag(Flags::Zero, output == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(output as u32));
        self.set_flag(Flags::Subtract, false);

        self.set_flag(Flags::Carry, carry);

        self.store(addr, output);
        self.tick_clock(15);
    }

    /* ============================ JUMP INSTRUCTIONS ============================= */
    /// Jump to the specified address
    fn jmp(&mut self, addr: u16) {
        self.set_reg_value_16(RegisterCode16::PC, addr);
        //println!("Jumping to 0x{:x}", addr);
        self.tick_clock(10);
    }

    fn jmp_addr(&mut self, src: RegisterCode16) {
        let addr = self.reg_value_16(src);
        self.set_reg_value_16(RegisterCode16::PC, addr);
        //println!("Jumping to 0x{:x}", addr);
        self.tick_clock(4);
    }

    /// Jump to the offset specified by the next byte
    fn jmp_rel(&mut self) {
        let addr = self.rel_addr();
        //println!(
        //    "Jumping from 0x{:x} to 0x{:x}",
        //    self.reg_value_16(RegisterCode16::PC),
        //    addr
        //);
        self.set_reg_value_16(RegisterCode16::PC, addr);
        self.tick_clock(12);
    }

    /// Execute a jump to the specified address if the flag matches the condition passed in
    fn jmp_cond(&mut self, addr: u16, flag: Flags, is_set: bool) {
        if self.flag(flag) == is_set {
            self.jmp(addr);
            self.tick_clock(12);
        } else {
            self.tick_clock(7);
        }
    }

    fn djnz(&mut self) {
        let addr = self.rel_addr();
        let mut b = self.reg_value(RegisterCode::B);
        b -= 1;
        self.set_reg_value(RegisterCode::B, b as u16);

        if b > 0 {
            self.set_reg_value_16(RegisterCode16::PC, addr);
            self.tick_clock(13);
        } else {
            self.tick_clock(8);
        }
    }

    fn ex_de_hl(&mut self) {
        self.reg
            .swap(RegisterCode::D as usize, RegisterCode::H as usize);
        self.reg
            .swap(RegisterCode::E as usize, RegisterCode::L as usize);

        self.tick_clock(4);
    }

    fn ex_af_altaf(&mut self) {
        mem::swap(
            &mut self.reg[RegisterCode::A as usize],
            &mut self.alt_reg[RegisterCode::A as usize],
        );

        mem::swap(
            &mut self.reg[RegisterCode::Flags as usize],
            &mut self.alt_reg[RegisterCode::Flags as usize],
        );

        self.tick_clock(4);
    }

    fn exx(&mut self) {
        self.reg[RegisterCode::B as usize..]
            .iter_mut()
            .zip(self.alt_reg[RegisterCode::B as usize..].iter_mut())
            .for_each(|(r, ar)| {
                mem::swap(r, ar);
            });

        self.tick_clock(4);
    }

    fn ex_spptr_reg(&mut self, reg_code: RegisterCode16) {
        let sp = self.reg_value_16(RegisterCode16::SP);
        let mut fetched_low = self.fetch(sp) as u16;
        let mut fetched_high = self.fetch(sp + 1) as u16;

        match reg_code {
            RegisterCode16::HL => {
                mem::swap(&mut self.reg[RegisterCode::H as usize], &mut fetched_high);
                mem::swap(&mut self.reg[RegisterCode::L as usize], &mut fetched_low);
            }

            RegisterCode16::IX | RegisterCode16::IY => {
                let mut val = ((fetched_high as u32) << 8) | fetched_low as u32;
                mem::swap(&mut self.spec_reg[reg_code as usize], &mut val);

                fetched_low = (val & 0xFF) as u16;
                fetched_high = ((val >> 8) & 0xFF) as u16;
            }

            _ => panic!(
                "Attempting to call Ex (SP), {:?}.  Only valid for IX, IY, or HL",
                reg_code
            ),
        }
        self.store(sp, fetched_low as u8);
        self.store(sp + 1, fetched_high as u8);

        self.tick_clock(19);
    }

    /// This instruction conditionally adjusts the Accumulator for BCD addition and
    /// subtraction operations
    ///
    /// See: https://stackoverflow.com/questions/8119577/z80-daa-instruction
    fn daa(&mut self) {
        let mut acc = self.reg_value(RegisterCode::A) as u16;
        let carry = self.flag(Flags::Carry)
            || (acc & 0xF0) > 0x90
            || (acc & 0xF0) >= 0x90 && (acc & 0x0F) > 0x90;

        let hcarry = (acc & 0x0F) > 0x90
            || self.flag(Flags::Subtract) && self.flag(Flags::HalfCarry) && (acc & 0xF0) <= 0x5;

        let func = |dest: &mut u16, val| {
            if self.flag(Flags::Subtract) {
                *dest += 0x100 - val;
            } else {
                *dest += val;
            }
        };

        // we have to add 6 to in order to "wrap" the lower 4 bits to 0
        if self.flag(Flags::HalfCarry) || (acc & 0xF) > 9 {
            func(&mut acc, 0x6);
        }

        if self.flag(Flags::Carry) || (acc & 0xF0) > 0x90 {
            func(&mut acc, 0x60);
        }

        self.set_flag(Flags::Carry, carry);
        self.set_flag(Flags::HalfCarry, hcarry);
        self.set_flag(Flags::Zero, acc == 0);
        self.set_flag(Flags::Sign, acc >= 080);

        let mut parity = 0;
        let mut val = acc;
        while val > 0 {
            parity ^= val & 1;
            val >>= 1;
        }

        self.set_flag(Flags::OverflowParity, parity > 0);

        self.tick_clock(4);
    }

    fn cpl(&mut self) {
        let mut acc = self.reg_value(RegisterCode::A);
        acc = !acc;
        self.set_reg_value(RegisterCode::A, acc as u16);

        self.set_flag(Flags::HalfCarry, true);
        self.set_flag(Flags::Subtract, true);

        self.tick_clock(4);
    }

    fn neg(&mut self) {
        let acc = self.reg_value(RegisterCode::A);
        let result = 0xFF - acc + 1;

        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::Zero, result & 0x0F > 0);
        self.set_flag(Flags::OverflowParity, acc == 0x80);
        self.set_flag(Flags::Subtract, true);
        self.set_flag(Flags::Carry, acc > 0);

        self.set_reg_value(RegisterCode::A, result as u16);
        self.tick_clock(8);
    }

    fn ccf(&mut self) {
        self.set_flag(Flags::HalfCarry, self.flag(Flags::Carry));
        self.set_flag(Flags::Subtract, false);
        self.set_flag(Flags::Carry, !self.flag(Flags::Carry));

        self.tick_clock(4);
    }

    fn scf(&mut self) {
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::Subtract, false);
        self.set_flag(Flags::Sign, true);

        self.tick_clock(4);
    }

    pub fn halt(&mut self) {
        self.halted = true;

        self.tick_clock(4);
    }

    pub fn reset(&mut self) {
        self.set_reg_value_16(RegisterCode16::PC, 0);
        self.set_reg_value(RegisterCode::I, 0);
        self.set_reg_value(RegisterCode::R, 0);
        self.iff1 = RESET;
        self.iff2 = RESET;
    }

    fn disable_intrpt(&mut self) {
        self.iff2 = false;
        self.iff1 = false;

        self.tick_clock(4);
    }

    fn enable_intrpt(&mut self) {
        self.iff2 = true;
        self.iff1 = true;
        self.interrupt_count = 2;

        self.tick_clock(4);
    }

    /* ====================== CALL and RET ============================= */

    fn call_addr(&mut self, addr: u16) {
        self.push_pc();
        self.set_reg_value_16(RegisterCode16::PC, addr);

        //println!("Calling 0x{:x}", addr);

        self.tick_clock(17);
    }

    fn call_cond_addr(&mut self, addr: u16, flag: Flags, is_set: bool) {
        if self.flag(flag) != is_set {
            self.tick_clock(10);
            return;
        }

        self.push_pc();
        self.set_reg_value_16(RegisterCode16::PC, addr);
        self.tick_clock(17);
    }

    fn ret(&mut self) {
        self.pop_pc();

        self.tick_clock(10);
    }

    fn ret_cond(&mut self, flag: Flags, is_set: bool) {
        if self.flag(flag) != is_set {
            self.tick_clock(5);
            return;
        }

        let low = self.pop() as u16;
        let high = self.pop() as u16;

        self.set_reg_value_16(RegisterCode16::PC, (high << 8) | low);
        self.tick_clock(11);
    }

    fn rst_lit(&mut self, offset: u8) {
        let offset = offset as u16;

        self.push_pc();

        self.set_reg_value_16(RegisterCode16::PC, offset);
        self.tick_clock(11);
    }

    fn retn(&mut self) {
        self.iff1 = self.iff2;

        self.pop_pc();
        self.tick_clock(14);
    }

    fn reti(&mut self) {
        self.pop_pc();
        self.tick_clock(14);
    }

    /// This is not supported in the sg-1000 and is a no op
    fn interrupt_0(&mut self) {
        self.tick_clock(8);
    }

    fn interrupt_1(&mut self) {
        if self.iff1 && self.interrupt_count == 0 {
            self.halted = false;
            self.push_pc();
            self.set_reg_value_16(RegisterCode16::PC, 0x0038);
            self.tick_clock(8);
        }
    }

    fn interrupt_nomask(&mut self) {
        self.push_pc();
        self.iff2 = self.iff1;
        self.iff1 = false;
        self.set_reg_value_16(RegisterCode16::PC, 0x0066);
        self.tick_clock(11);
    }

    fn ld_id(&mut self, is_inc: bool) {
        let mut src = self.reg_value_16(RegisterCode16::HL);
        let mut dst = self.reg_value_16(RegisterCode16::DE);

        let val = self.fetch(src);
        self.store(dst, val);

        let inc = |val: u16| (((val as u32) + 1) % 0xFFFF) as u16;
        let dec = |val: u16| if val == 0 { 0xFFFF } else { val - 1 };

        if is_inc {
            src = inc(src);
            dst = inc(src);
        } else {
            src = dec(src);
            dst = dec(dst);
        };

        let mut bc = self.reg_value_16(RegisterCode16::BC);
        bc = dec(bc);
        self.set_reg_value_16(RegisterCode16::BC, bc);

        self.set_reg_value_16(RegisterCode16::HL, src);
        self.set_reg_value_16(RegisterCode16::DE, dst);

        self.set_flag(Flags::HalfCarry, RESET);
        self.set_flag(Flags::OverflowParity, bc != 0);
        self.set_flag(Flags::Subtract, RESET);

        self.tick_clock(16);
    }

    fn ld_id_r(&mut self, is_inc: bool) {
        self.ld_id(is_inc);

        // if we have to repeat the opcode then we do not mess with the flags.  The next time the
        // cpu uses an opcode it will repeat this command.  No looping allows the cpu to register
        // non-maskable interrupts from perripherals
        if self.reg_value_16(RegisterCode16::BC) > 0 {
            let addr = self.reg_value_16(RegisterCode16::PC);
            self.set_reg_value_16(RegisterCode16::PC, addr - 2);
            // we only have extra clock ticks if we need to change the PC to repeat the command
            self.tick_clock(5);
        } else {
            // we are done repeating, so we can now set the flags
            self.set_flag(Flags::HalfCarry, RESET);
            self.set_flag(Flags::OverflowParity, RESET);
            self.set_flag(Flags::Subtract, RESET);
        }
    }

    fn cp_id(&mut self, is_inc: bool) {
        let inc = |val: u16| (((val as u32) + 1) % 0xFFFF) as u16;
        let dec = |val: u16| if val == 0 { 0xFFFF } else { val - 1 };

        let mut hl = self.reg_value_16(RegisterCode16::HL);
        let val = self.fetch(hl);
        let a = self.reg_value(RegisterCode::A);

        let result = if val > a { 0xFF - val + a + 1 } else { a - val };
        let mut bc = self.reg_value_16(RegisterCode16::BC);
        bc = dec(bc);
        self.set_reg_value_16(RegisterCode16::BC, bc);

        self.set_flag(Flags::Sign, result >= 0x80);
        self.set_flag(Flags::Zero, result == 0);
        self.set_flag(Flags::HalfCarry, (val & 0x0F) > (a & 0x0F));
        self.set_flag(Flags::OverflowParity, bc != 0);
        self.set_flag(Flags::Subtract, SET);

        hl = if is_inc { inc(hl) } else { dec(hl) };
        self.set_reg_value_16(RegisterCode16::HL, hl);
    }

    fn cp_id_r(&mut self, is_inc: bool) {
        self.cp_id(is_inc);

        // if we have to repeat the opcode then we do not mess with the flags.  We repeat the
        // command if the BC register pair (BC for byte-count) is greater than 0 and A != (HL).
        // The next time the cpu uses an opcode it will repeat this command.  No looping allows
        // the cpu to register non-maskable interrupts from perripherals
        if self.reg_value_16(RegisterCode16::BC) > 0 && !self.flag(Flags::Zero) {
            let addr = self.reg_value_16(RegisterCode16::PC);
            self.set_reg_value_16(RegisterCode16::PC, addr - 2);
            // we only have extra clock ticks if we need to change the PC to repeat the command
            self.tick_clock(5);
        }
    }

    fn rrd(&mut self) {
        let mut acc = self.reg_value(RegisterCode::A);
        let addr = self.reg_value_16(RegisterCode16::HL);
        let mut val = self.fetch(addr);

        let temp = val >> 4;
        val = ((val & 0b1111) << 4) | acc & 0b1111; // set the high bits to the low bits and the low bits to the low bits of the accumulator
        acc = (acc & 0b1111_0000) | temp;

        self.set_flag(Flags::Sign, acc >= 0x80);
        self.set_flag(Flags::Zero, acc == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(acc as u32));
        self.set_flag(Flags::Subtract, false);

        self.store(addr, val);
        self.set_reg_value(RegisterCode::A, acc as u16);

        self.tick_clock(18);
    }

    fn rld(&mut self) {
        let mut acc = self.reg_value(RegisterCode::A);
        let addr = self.reg_value_16(RegisterCode16::HL);
        let mut val = self.fetch(addr);

        let temp = val & 0b1111;
        val = ((acc & 0b1111) << 4) | val >> 4; // set the high bits to the low bits and the low bits to the low bits of the accumulator
        acc = (acc & 0b1111_0000) | temp;

        self.set_flag(Flags::Sign, acc >= 0x80);
        self.set_flag(Flags::Zero, acc == 0);
        self.set_flag(Flags::HalfCarry, false);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(acc as u32));
        self.set_flag(Flags::Subtract, false);

        self.store(addr, val);
        self.set_reg_value(RegisterCode::A, acc as u16);

        self.tick_clock(18);
    }

    fn test_bit_val(val: u8, bit: u8) -> bool {
        (val >> bit) & 1 > 0
    }

    fn test_bit_reg(&mut self, src: RegisterCode, bit: u8) {
        let reg = self.reg_value(src);
        let is_set = Cpu::test_bit_val(reg, bit);

        self.set_flag(Flags::Zero, !is_set);
        self.set_flag(Flags::HalfCarry, true);
        self.set_flag(Flags::Subtract, false);
        self.tick_clock(8);
    }

    fn test_bit_addr(&mut self, addr: u16, bit: u8) {
        let val = self.fetch(addr);
        let is_set = Cpu::test_bit_val(val, bit);

        self.set_flag(Flags::Zero, !is_set);
        self.set_flag(Flags::HalfCarry, true);
        self.set_flag(Flags::Subtract, false);
        self.tick_clock(12);
    }

    fn res_bit(val: u8, bit: u8) -> u8 {
        val & !(1 << bit)
    }

    fn set_bit(val: u8, bit: u8) -> u8 {
        val | (1 << bit)
    }

    fn change_bit_reg(&mut self, src: RegisterCode, bit: u8, set: bool) {
        let val = self.reg_value(src);
        let output = if set {
            Cpu::set_bit(val, bit)
        } else {
            Cpu::res_bit(val, bit)
        };

        self.set_reg_value(src, output as u16);
        self.tick_clock(8);
    }

    fn change_bit_addr(&mut self, addr: u16, bit: u8, set: bool) {
        let val = self.fetch(addr);
        let output = if set {
            Cpu::set_bit(val, bit)
        } else {
            Cpu::res_bit(val, bit)
        };

        self.store(addr, output);
        self.tick_clock(15);
    }

    fn out_addr_val(&mut self, addr: u16, val: u8) {
        self.io_bus.borrow_mut().cpu_write(addr, val);
    }

    fn out_a_lit(&mut self) {
        let val = self.reg_value(RegisterCode::A);
        let addr_high = self.reg_value(RegisterCode::A) as u16;
        let addr_low = self.imm_addr() as u16;

        let addr = (addr_high << 8) | addr_low;
        self.out_addr_val(addr, val);
        self.tick_clock(11);
    }

    fn out_c_reg(&mut self, src: Option<RegisterCode>) {
        let val = src.map(|reg| self.reg_value(reg)).unwrap_or(0);
        let dst = self.reg_value_16(RegisterCode16::BC);

        self.out_addr_val(dst, val);
        self.tick_clock(12);
    }

    fn out_id(&mut self, inc: bool) {
        let mut hl = self.reg_value_16(RegisterCode16::HL);
        let val = self.fetch(hl);

        let mut b = self.reg_value(RegisterCode::B);
        b = if b == 0 { 0xFF } else { b - 1 };
        self.set_reg_value(RegisterCode::B, b as u16);

        let addr = self.reg_value_16(RegisterCode16::BC);

        self.out_addr_val(addr, val);

        if inc {
            hl = (((hl as u32) + 1) % 0xFFFF) as u16;
        } else {
            hl = if hl == 0 { 0xFFFF } else { hl - 1 };
        }
        self.set_reg_value_16(RegisterCode16::HL, hl);

        // set flags
        self.set_flag(Flags::Zero, b == 0);
        self.set_flag(Flags::Subtract, SET);
        self.tick_clock(16);
    }

    fn out_id_rep(&mut self, inc: bool) {
        self.out_id(inc);
        //println!("B after OutIR: {}", self.reg_value(RegisterCode::B));

        if self.reg_value(RegisterCode::B) != 0 {
            self.tick_clock(5);
            let pc = self.get_pc();
            self.set_pc(if pc < 2 { 0xFFFF - pc + 1 } else { pc - 2 });
        }
    }

    fn in_addr(&self, addr: u16) -> u8 {
        self.io_bus.borrow_mut().cpu_read(addr).expect(&format!(
            "Attempting to read from IO bus at 0x{:04x} but there was no mapping for that address",
            addr
        ))
    }

    fn in_a_lit(&mut self) {
        let addr_low = self.imm_addr() as u16;
        let addr_high = self.reg_value(RegisterCode::A) as u16;
        let addr = (addr_high << 8) | addr_low;
        let val = self.in_addr(addr);

        self.set_reg_value(RegisterCode::A, val as u16);
        self.tick_clock(11);
    }

    /// dst is set to `Some` if we store the value to a register or `None` if it
    /// gets ignored
    fn in_reg_c(&mut self, dst: Option<RegisterCode>) {
        let addr = self.reg_value_16(RegisterCode16::BC);

        let val = self.in_addr(addr);
        dst.map(|reg| self.set_reg_value(reg, val as u16));

        self.set_flag(Flags::Sign, val >= 0x80);
        self.set_flag(Flags::Zero, val == 0);
        self.set_flag(Flags::HalfCarry, RESET);
        self.set_flag(Flags::OverflowParity, Cpu::parity_even(val.into()));
        self.set_flag(Flags::Subtract, RESET);

        self.tick_clock(12);
    }

    fn in_id(&mut self, inc: bool) {
        let mut hl = self.reg_value_16(RegisterCode16::HL);

        let mut b = self.reg_value(RegisterCode::B);
        b = if b == 0 { 0xFF } else { b - 1 };
        self.set_reg_value(RegisterCode::B, b as u16);

        let addr = self.reg_value_16(RegisterCode16::BC);

        let val = self.in_addr(addr);
        self.store(hl, val);

        if inc {
            hl = (((hl as u32) + 1) % 0xFFFF) as u16;
        } else {
            hl = if hl == 0 { 0xFFFF } else { hl - 1 };
        }
        self.set_reg_value_16(RegisterCode16::HL, hl);

        // set flags
        self.set_flag(Flags::Zero, b == 0);
        self.set_flag(Flags::Subtract, SET);
        self.tick_clock(16);
    }

    fn in_id_rep(&mut self, inc: bool) {
        self.in_id(inc);

        if self.reg_value(RegisterCode::B) != 0 {
            self.tick_clock(5);
            let pc = self.get_pc();
            self.set_pc(if pc < 2 { 0xFFFF - pc + 1 } else { pc - 2 });
        }
    }
}

/* --------------------------------- TESTING --------------------------------- */

pub trait BitsOperator {
    fn pre_operate(&mut self, _cpu: &mut Cpu, _src: RegisterCode) {}
    fn post_operate(&mut self, _cpu: &mut Cpu, _src: RegisterCode) {}
    fn pointer(&mut self, _cpu: &mut Cpu) -> u16;
    fn prepare(&mut self, _cpu: &mut Cpu) {}
}

struct BitsOperatorDefault {}

impl BitsOperator for BitsOperatorDefault {
    fn pointer(&mut self, cpu: &mut Cpu) -> u16 {
        cpu.indirect_reg_addr(RegisterCode16::HL)
    }
}

struct IndexedBitsOperator {
    reg: RegisterCode16,
    addr: Option<u16>,
}

impl IndexedBitsOperator {
    pub fn new(reg: RegisterCode16) -> IndexedBitsOperator {
        IndexedBitsOperator { reg, addr: None }
    }
}

impl BitsOperator for IndexedBitsOperator {
    fn pre_operate(&mut self, cpu: &mut Cpu, src: RegisterCode) {
        if self.addr.is_none() {
            self.addr = Some(cpu.index_addr(self.reg));
        }

        let val = cpu.fetch(self.addr.unwrap());
        cpu.set_reg_value(src, val as u16);
        cpu.queue_clock_tick(4);
    }

    fn post_operate(&mut self, cpu: &mut Cpu, src: RegisterCode) {
        if self.addr.is_none() {
            self.addr = Some(cpu.index_addr(self.reg));
        }

        let val = cpu.reg_value(src);
        cpu.store(self.addr.unwrap(), val);
        cpu.queue_clock_tick(4);
    }

    fn pointer(&mut self, cpu: &mut Cpu) -> u16 {
        cpu.index_addr(self.reg)
    }

    fn prepare(&mut self, cpu: &mut Cpu) {
        self.addr = Some(cpu.index_addr(self.reg));
    }
}
