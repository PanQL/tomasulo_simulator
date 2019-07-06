use std::sync::Arc;
use std::fmt;
use std::cell::RefCell;
use super::reserved_station::ReservedStation;
use crate::gtk::TextViewExt;
use crate::gtk::TextBufferExt;

pub struct Register {
    value : u32,
    writer : Option<Arc<RefCell<ReservedStation>>>,
    text : gtk::TextView,
    pub writer_name : Option<&'static str>,
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "value : {} ", self.value)
    }
}

impl Register {
    pub fn new(text : gtk::TextView) -> Self{
        Register{
            value : 0, 
            writer : None,
            text,
            writer_name : None,
        }
    }

    pub fn set_value(&mut self, value : u32) {
        self.value = value;
        if self.writer.is_some() {
            self.writer = None;
            self.writer_name = None;
        }
    }

    pub fn is_waiting(&self) -> bool{
        self.writer.is_some()
    }

    pub fn get_value(&self) -> u32 {
        self.value.clone()
    }

    pub fn set_value_purlly(&mut self, value : u32 ) {
        self.value = value;
    }

    pub fn register_rs(&self, idx : u8, ptr : Arc<RefCell<ReservedStation>>) {
        if let Some(writer) = &self.writer {
            writer.borrow_mut().register_rs(idx, ptr);
        }
    }

    pub fn set_writer(&mut self, writer : Arc<RefCell<ReservedStation>>, writer_name : &'static str ) {
        self.writer = Some(writer);
        self.writer_name = Some(writer_name);
    }

    pub fn clear_writer(&mut self) {
        self.writer = None;
        self.writer_name = None;
    }

    pub fn show(&self) {
        //if self.writer_name.is_some() {
            //self.text.get_buffer().expect("failed to get buffer").set_text(self.writer_name.unwrap());
        //} else {
            //self.text.get_buffer().expect("failed to get buffer").set_text(&*(self.value as i32).to_string());
        //}
    }
}
