use std::ops::Range;

pub mod bus;
pub mod ram;

pub type MutRef<T> = std::rc::Rc<std::cell::RefCell<T>>;

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
        if self.len() <= addr as usize {
            self.resize(addr as usize + 1, 0);
        }
        self[addr as usize] = data;
        true
    }

    fn cpu_read(&self, addr: u16) -> u8 {
        **self.get(addr as usize).get_or_insert(&0)
    }
}

/// Acts as a memory map.  Min is an inclusive minimum value, Max is an inclusive maximum.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MemoryMap {
    pub min: u16, // Min address (inclusive)
    pub max: u16, // Max address (inclusive)
}

impl MemoryMap {
    pub fn new(min: u16, max: u16) -> MemoryMap {
        MemoryMap { min, max }
    }

    pub fn contains(&self, val: u16) -> bool {
        val >= self.min && val <= self.max
    }
}

impl From<(u16, u16)> for MemoryMap {
    fn from(range: (u16, u16)) -> Self {
        MemoryMap {
            min: range.0,
            max: range.1,
        }
    }
}

impl From<(u8, u8)> for MemoryMap {
    fn from(range: (u8, u8)) -> Self {
        MemoryMap {
            min: range.0 as u16,
            max: range.1 as u16,
        }
    }
}

impl From<Range<u16>> for MemoryMap {
    fn from(range: Range<u16>) -> Self {
        MemoryMap {
            min: range.start,
            max: range.end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_memory_map() {
        let mem_map = MemoryMap::from((10u16, 20u16));
        assert!(mem_map.contains(10));
        assert!(mem_map.contains(20));
        assert!(mem_map.contains(15));
        assert!(!mem_map.contains(9));
        assert!(!mem_map.contains(21));
    }
}
