use crate::{BusConnectable, MutRef};
use std::{cell::RefCell, rc::Rc, vec::Vec};

impl Into<Bus> for Vec<u8> {
    fn into(self) -> Bus {
        Bus::new(vec![Rc::new(RefCell::new(self))])
    }
}

/// Represent a data bus
///
/// One one piece of data may be on the bus at one time
pub struct Bus {
    connections: Vec<MutRef<dyn BusConnectable>>,
}

#[allow(dead_code)]
impl Bus {
    pub fn builder() -> BusBuilder {
        BusBuilder::new()
    }

    pub fn new(connections: Vec<MutRef<dyn BusConnectable>>) -> Bus {
        Bus { connections }
    }

    pub fn cpu_write(&mut self, addr: u16, data: u8) -> bool {
        self.connections
            .iter_mut()
            .find(|conn| conn.borrow().accept(addr))
            .map(|conn| conn.borrow_mut().cpu_write(addr, data))
            .is_some()
    }
    pub fn cpu_read(&self, addr: u16) -> Option<u8> {
        self.connections
            .iter()
            .find(|&conn| conn.borrow().accept(addr))
            .map(|conn| conn.borrow_mut().cpu_read(addr))
    }
}

impl Default for Bus {
    fn default() -> Bus {
        Bus::new(vec![Rc::new(RefCell::new(vec![]))])
    }
}

pub struct BusBuilder {
    connections: Vec<MutRef<dyn BusConnectable>>,
}

impl BusBuilder {
    pub fn new() -> BusBuilder {
        BusBuilder {
            connections: vec![],
        }
    }

    pub fn add<T>(mut self, connection: T) -> Self
    where
        T: 'static + BusConnectable,
    {
        self.connections.push(Rc::new(RefCell::new(connection)));
        self
    }

    pub fn add_box<T>(mut self, connection: Box<T>) -> Self
    where
        T: 'static + BusConnectable,
    {
        self.connections.push(Rc::new(RefCell::new(*connection)));
        self
    }

    pub fn add_ref(mut self, connection: &MutRef<dyn BusConnectable>) -> Self {
        self.connections.push(Rc::clone(connection));
        self
    }

    pub fn build(self) -> Bus {
        Bus {
            connections: self.connections,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bus_builder() {
        let bus = Bus::builder()
            .add(vec![0x01, 0x02])
            .add(vec![0x03, 0x04])
            .build();

        assert_eq!(bus.connections.len(), 2);
    }
}
