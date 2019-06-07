use super::*;
use gtk::prelude::*;


pub trait ReservedStationDisplay {
    fn show_busy(&self, busy : bool);
    fn set_op(&self, op_type : InstructionType);
    fn show_vj(&self, vj : Option<&u32>);
    fn show_vk(&self, vk : Option<&u32>);
    fn show_qj(&self, qj : &str);
    fn show_qk(&self, qk : &str);
    fn clear(&self);
}

pub trait CalculatorDisplay {
    fn show_times(&self, times : &u32);
    fn set_op(&self, op_type : InstructionType);
    fn show_src1(&self, src1 : &u32);
    fn show_src2(&self, src2 : &u32);
    fn show_target(&self, target : &usize);
    fn clear(&self);
}

pub struct LoaderBufferUi {
    pub busy : gtk::TextView,
    pub value : gtk::TextView,
}

pub struct CalculatorRsUi {
    pub op_type : InstructionType,
    pub busy : gtk::TextView,
    pub op  : gtk::TextView,
    pub vj : gtk::TextView,
    pub vk : gtk::TextView,
    pub qj : gtk::TextView,
    pub qk : gtk::TextView,
}

impl ReservedStationDisplay for LoaderBufferUi {
    fn show_busy(&self, busy: bool) {
        if busy {
            self.busy.get_buffer().expect("failed to get load buffer").set_text("Yes");
        } else {
            self.busy.get_buffer().expect("failed to get load buffer").set_text("No");
        }
    }

    fn set_op(&self, _ : InstructionType) {
    }

    fn show_vj(&self, vj: Option<&u32>) {
        if vj.is_some() {
            self.value.get_buffer().expect("failed to get load value").set_text(&*vj.unwrap().to_string());
        } else {
            self.value.get_buffer().expect("failed to get load value").set_text("");
        }
    }

    fn show_vk(&self, _ : Option<&u32>) {
    }

    fn show_qj(&self, _ : &str) {
    }

    fn show_qk(&self, _ : &str) {
    }

    fn clear(&self) {
        self.busy.get_buffer().expect("failed to get load buffer").set_text("No");
        self.value.get_buffer().expect("failed to get load value").set_text("");
    }
}

impl ReservedStationDisplay for CalculatorRsUi {
    fn show_busy(&self, busy: bool) {
        if busy {
            self.busy.get_buffer().expect("failed to get load buffer").set_text("Yes");
        } else {
            self.busy.get_buffer().expect("failed to get load buffer").set_text("No");
        }
    }

    fn set_op(&self, op_type: InstructionType) {
        let s = match op_type {
            InstructionType::ADD => "ADD",
            InstructionType::SUB => "SUB",
            InstructionType::MUL => "MUL",
            InstructionType::DIV => "DIV",
            InstructionType::JUMP => "JUMP",
            _ => ""
        };
        self.op.get_buffer().expect("failed to get calculator op ui").set_text(s);
    }

    fn show_vj(&self, vj: Option<&u32>) {
        if vj.is_some() {
            self.vj.get_buffer().expect("failed to get load value").set_text(&*vj.unwrap().to_string());
        } else {
            self.vj.get_buffer().expect("failed to get load value").set_text("");
        }
    }

    fn show_vk(&self, vk: Option<&u32>) {
        if vk.is_some() {
            self.vk.get_buffer().expect("failed to get load value").set_text(&*vk.unwrap().to_string());
        } else {
            self.vk.get_buffer().expect("failed to get load value").set_text("");
        }
    }

    fn show_qj(&self, qj: &str) {
        self.qj.get_buffer().expect("failed to get load value").set_text(qj);
    }

    fn show_qk(&self, qk: &str) {
        self.qk.get_buffer().expect("failed to get load value").set_text(qk);
    }

    fn clear(&self) {
        self.busy.get_buffer().expect("failed to get load buffer").set_text("No");
        self.op.get_buffer().expect("failed to get calculator op ui").set_text("");
        self.vj.get_buffer().expect("failed to get load value").set_text("");
        self.vk.get_buffer().expect("failed to get load value").set_text("");
        self.qj.get_buffer().expect("failed to get load value").set_text("");
        self.qk.get_buffer().expect("failed to get load value").set_text("");
    }
}

pub struct LoaderUi {
    pub target : gtk::TextView,
    pub times : gtk::TextView,
    pub value : gtk::TextView,
}

pub struct CalculatorUi {
    pub op_type : InstructionType,
    pub times : gtk::TextView,
    pub target : gtk::TextView,
    pub op  : gtk::TextView,
    pub value1 : gtk::TextView,
    pub value2 : gtk::TextView,
}

impl CalculatorDisplay for LoaderUi {
    fn show_times(&self, times: &u32) {
        self.times.get_buffer().expect("failed to get load buffer").set_text(&*times.to_string());
    }

    fn set_op(&self, _ : InstructionType) {
    }

    fn show_src1(&self, src1: &u32) {
        self.value.get_buffer().expect("failed to get load buffer").set_text(&*src1.to_string());
    }

    fn show_src2(&self, _ : &u32) {
    }

    fn show_target(&self, target: &usize) {
        self.target.get_buffer().expect("failed to get load buffer").set_text(&*target.to_string());
    }

    fn clear(&self) {
        self.target.get_buffer().expect("failed to get load buffer").set_text("");
        self.times.get_buffer().expect("failed to get load buffer").set_text("");
        self.value.get_buffer().expect("failed to get load buffer").set_text("");
    }
}

impl CalculatorDisplay for CalculatorUi {
    fn show_times(&self, times: &u32) {
        self.times.get_buffer().expect("failed to get load buffer").set_text(&*times.to_string());
    }

    fn set_op(&self, op_type: InstructionType) {
        let s = match op_type {
            InstructionType::ADD => "ADD",
            InstructionType::SUB => "SUB",
            InstructionType::MUL => "MUL",
            InstructionType::DIV => "DIV",
            InstructionType::JUMP => "JUMP",
            _ => ""
        };
        self.op.get_buffer().expect("failed to get calculator op ui").set_text(s);
    }

    fn show_src1(&self, src1: &u32) {
        self.value1.get_buffer().expect("failed to get load buffer").set_text(&*src1.to_string());
    }

    fn show_src2(&self, src2: &u32) {
        self.value2.get_buffer().expect("failed to get load buffer").set_text(&*src2.to_string());
    }

    fn show_target(&self, target: &usize) {
        self.target.get_buffer().expect("failed to get load buffer").set_text(&*target.to_string());
    }

    fn clear(&self) {
        self.target.get_buffer().expect("failed to get load buffer").set_text("");
        self.times.get_buffer().expect("failed to get load buffer").set_text("");
        self.op.get_buffer().expect("failed to get calculator op ui").set_text("");
        self.value1.get_buffer().expect("failed to get load buffer").set_text("");
        self.value2.get_buffer().expect("failed to get load buffer").set_text("");
    }
}
