use crate::{BusConnectable, MemoryMap, MutRef};
use std::{cell::RefCell, rc::Rc};

const MAX_SIZE: usize = 0x1_00_00;

#[allow(dead_code)]
pub struct Ram {
    data: MutRef<Vec<u8>>,
    size: usize,
    memory_map: MemoryMap,
    mirrors: Vec<MemoryMap>,
    read_only: bool,
}

impl Ram {
    pub fn new(size: usize, memory_map: MemoryMap) -> Ram {
        Ram {
            size,
            data: Rc::new(RefCell::new(Vec::with_capacity(size))),
            memory_map,
            mirrors: vec![],
            read_only: false,
        }
    }

    /// A builder for building a ram module
    ///
    /// # Default values:
    ///     size:       0x01 00 00 (0 through 0xFFFF)
    ///     memory_map: min = 0 max = 0xFFFF
    ///     mirrors:    empty vec
    ///     data:       Vec of size with all values 0
    ///     read_only:  false
    pub fn builder() -> RamBuilder {
        RamBuilder::new()
    }

    /// Get the an Rc::clone of the data in the VRAM
    pub fn vram(&self) -> MutRef<Vec<u8>> {
        Rc::clone(&self.data)
    }
}

impl BusConnectable for Ram {
    fn accept(&self, addr: u16) -> bool {
        if self.memory_map.contains(addr) {
            return true;
        }
        self.mirrors.iter().find(|map| map.contains(addr)).is_some()
    }

    fn cpu_read(&mut self, addr: u16) -> u8 {
        if self.memory_map.contains(addr) {
            let index = addr - self.memory_map.min;
            return self.data.borrow()[index as usize];
        }

        self.mirrors
            .iter()
            .find(|map| map.contains(addr))
            .map(|map| self.data.borrow()[(addr - map.min) as usize])
            .unwrap()
    }

    fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        if self.read_only {
            panic!(
                "Attempting to write to ROM at address 0x{:x} with data 0x{:x}",
                addr, data
            );
        }

        if self.memory_map.contains(addr) {
            let index = addr - self.memory_map.min;
            self.data.borrow_mut()[index as usize] = data;
        }

        let vec = &mut self.data.borrow_mut();
        self.mirrors
            .iter_mut()
            .find(|map| map.contains(addr))
            .map(move |map| vec[(addr - map.min) as usize] = data)
            .is_some()
    }
}

pub struct RamBuilder {
    data: Option<MutRef<Vec<u8>>>,
    size: Option<usize>,
    memory_map: Option<MemoryMap>,
    mirrors: Option<Vec<MemoryMap>>,
    read_only: Option<bool>,
}

impl RamBuilder {
    pub fn new() -> RamBuilder {
        RamBuilder {
            data: None,
            size: None,
            memory_map: None,
            mirrors: None,
            read_only: None,
        }
    }

    pub fn size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = Some(Rc::new(RefCell::new(data)));
        self
    }

    pub fn mirrors<T: Into<MemoryMap>>(mut self, memory_maps: Vec<T>) -> Self {
        self.mirrors = Some(memory_maps.into_iter().map(|m| m.into()).collect());
        self
    }

    pub fn mirror<T: Into<MemoryMap>>(mut self, map: T) -> Self {
        if let Some(maps) = &mut self.mirrors {
            maps.push(map.into());
        } else {
            self.mirrors = Some(vec![map.into()]);
        }

        self
    }

    pub fn map<T: Into<MemoryMap>>(mut self, map: T) -> Self {
        self.memory_map = Some(map.into());
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = Some(read_only);
        self
    }

    pub fn build(self) -> Ram {
        let size = self.size.unwrap_or(MAX_SIZE);
        let data = self
            .data
            .unwrap_or(Rc::new(RefCell::new(Vec::with_capacity(size))));

        data.borrow_mut().resize(size, 0);
        Ram {
            data,
            size,
            memory_map: self.memory_map.unwrap_or(MemoryMap::from(0..0xFFFF)),
            mirrors: self.mirrors.unwrap_or(vec![]),
            read_only: self.read_only.unwrap_or(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ram_builder() {
        let ram = Ram::builder()
            .size(100)
            .data((0..100).collect())
            .map(0..5)
            .mirror(MemoryMap::from(500..505))
            .build();

        assert_eq!(100, ram.size);
        ram.data
            .borrow()
            .iter()
            .zip(0..100)
            .for_each(|(&fst, snd)| assert_eq!(fst, snd));
        assert_eq!(0, ram.memory_map.min);
        assert_eq!(4, ram.memory_map.max);
        assert_eq!(MemoryMap::from(500..505), ram.mirrors[0]);

        let ram = Ram::builder()
            .size(100)
            .data((0..100).collect())
            .map(0..=5)
            .mirror(MemoryMap::from(500..505))
            .build();

        assert_eq!(5, ram.memory_map.max);
    }

    #[test]
    fn test_accept() {
        let ram = Ram::builder()
            .map(0xFF..=0xFFFE)
            .mirror(0x01..=0xA0)
            .build();
        assert_eq!(MemoryMap::from(0xFF..=0xFFFE), ram.memory_map);
        assert!(ram.accept(0xFF));
        assert!(ram.accept(0xFFFE));
        assert!(ram.accept(0xFFF0));

        assert!(!ram.accept(0xFF - 1));
        assert!(!ram.accept(0xFFFF));

        assert!(ram.accept(0x01));
        assert!(ram.accept(0xA0));
        assert!(ram.accept(0x0F));

        assert!(!ram.accept(0x01 - 1));
        assert!(!ram.accept(0xA1));
    }
}
