use std::sync::Arc;
use std::fmt;
use std::cell::RefCell;
use super::reserved_station::ReservedStation;

pub struct Register {
    value : u32,
    writer : Option<Arc<RefCell<ReservedStation>>>,
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "value : {} ", self.value)
    }
}

impl Register {
    pub fn new() -> Self{
        Register{
            value : 0, 
            writer : None,
        }
    }

    pub fn set_value(&mut self, value : u32) {
        self.value = value;
        if self.writer.is_some() {
            self.writer = None;
        }
    }

    pub fn is_waiting(&self) -> bool{
        self.writer.is_some()
    }

    pub fn get_value(&self) -> u32 {
        self.value
    }

    pub fn set_value_purlly(&mut self, value : u32 ) {
        self.value = value;
    }

    pub fn register_rs(&self, idx : u8, ptr : Arc<RefCell<ReservedStation>>) {
        if let Some(writer) = &self.writer {
            writer.borrow_mut().register_rs(idx, ptr);
        }
    }

    pub fn get_writer(&self) -> Option<Arc<RefCell<ReservedStation>>> {
        self.writer.clone()
    }

    pub fn set_writer(&mut self, writer : Arc<RefCell<ReservedStation>> ) {
        self.writer = Some(writer);
    }

    pub fn clear_writer(&mut self) {
        self.writer = None;
    }
}
