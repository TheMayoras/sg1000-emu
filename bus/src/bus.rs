use std::vec::Vec;

/// Represents an object connected to a bus
///
/// An object connected to a bus
pub trait BusConnectable {
    fn accept(&self, addr: u16) -> bool;
    fn cpu_write(&mut self, addr: u16, data: u8) -> bool;
    fn cpu_read(&self, addr: u16) -> u8;
}

/// A simple implementation for a vector to be connected to a bus
///
/// The vector accepts all addresses will resize to be able to always return a value
impl BusConnectable for Vec<u8> {
    #[allow(unused_variables)]
    fn accept(&self, addr: u16) -> bool {
        true
    }

    fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        if self.len() < addr as usize {
            self.resize(addr as usize, 0);
        }
        self[addr as usize] = data;
        true
    }

    fn cpu_read(&self, addr: u16) -> u8 {
        **self.get(addr as usize).get_or_insert(&0)
    }
}

/// Represent a data bus
///
/// One one piece of data may be on the bus at one time
pub struct Bus {
    connections: Vec<Box<dyn BusConnectable>>,
}

#[allow(dead_code)]
impl Bus {
    pub fn new(connections: Vec<Box<dyn BusConnectable>>) -> Bus {
        Bus { connections }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        self.connections
            .iter_mut()
            .find(|conn| conn.accept(addr))
            .map(|conn| conn.cpu_write(addr, data))
            .is_some()
    }
    pub fn cpu_read(&self, addr: u16) -> Option<u8> {
        self.connections
            .iter()
            .find(|&conn| conn.accept(addr))
            .map(|conn| conn.cpu_read(addr))
    }
}

impl Default for Bus {
    fn default() -> Bus {
        Bus::new(vec![Box::new(vec![])])
    }
}
