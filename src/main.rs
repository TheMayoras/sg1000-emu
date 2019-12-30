extern crate bus;
extern crate z80_cpu_emu;

use bus::bus::Bus;
use z80_cpu_emu::cpu::*;

fn main() {
    let cpu = Cpu::new(Bus::new(vec![Box::new(vec![])]));
}
