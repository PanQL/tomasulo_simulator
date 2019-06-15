use super::reserved_station::ReservedStation;
use super::instruction::InstructionType;
use super::display::CalculatorDisplay;
use std::sync::Arc;
use std::cell::RefCell;

pub struct Calculator {
    busy : bool, // 是否正在使用
    times : u32, // 剩余计算周期
    op_type : InstructionType, // 计算类型
    s1 : u32, // 操作数1
    s2 : u32, // 操作数2
    station : Option<Arc<RefCell<ReservedStation>>>,    // 对应的保留站
    pub ui : Box<dyn CalculatorDisplay>,
}

impl Calculator {
    pub fn new(_type : InstructionType, ui : Box<dyn CalculatorDisplay>) -> Self{
        Calculator {
            busy : false,
            times : 0,
            op_type : _type,
            s1 : 0,
            s2 : 0,
            station : None,
            ui,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.busy
    }

    pub fn step(&mut self) -> Option<(usize, u32, &'static str)>{
        if !self.busy { return None; }
        self.times -= 1;
        if self.times == 0 {    // 写回结束
            self.busy = false;
            return self.write_back();
        } else {
            self.ui.show_times(&self.times);
            return None;
        }
    }

    fn write_back(&mut self) -> Option<(usize, u32, &'static str)> {
        let res : u32 = match self.op_type {
            InstructionType::ADD => { self.s1 + self.s2 }
            InstructionType::SUB => { self.s1 - self.s2 }
            InstructionType::MUL => { self.s1 * self.s2 }
            InstructionType::DIV => { self.s1 / self.s2 }
            InstructionType::LD => { self.s1 }
            InstructionType::JUMP => { if self.s1 == self.s2 { 1 } else { 0 }}
        };

        let ret;
        if let Some(station) = &self.station {
            let mut rs = station.borrow_mut();
            let position = rs.write_back(res.clone());
            match self.op_type {
                InstructionType::JUMP => {
                    ret = if self.s1 == self.s2 {
                        Some((position, rs.get_pc_result().unwrap(), rs.name))
                    } else {
                        Some((position, 0xFFFF_FFFF, rs.name))
                    };
                }
                _ => {
                    ret =  Some((position, res, rs.name));
                }
            }
        } else {
            ret =  None;
        }
        self.station = None;
        self.ui.clear();
        ret
    }

    pub fn set_instruction(&mut self, _type : InstructionType, source1 : u32, source2 : u32) {
        self.busy = true;
        self.op_type = _type;
        self.times = match self.op_type {
            InstructionType::ADD => 3,
            InstructionType::SUB => 3,
            InstructionType::MUL => 12,
            InstructionType::DIV => if source2 == 0 { 1 } else { 40 },
            InstructionType::LD => 3,
            InstructionType::JUMP => 1,
        };
        self.s1 = source1;
        self.s2 = match self.op_type {
            InstructionType::DIV => if source2 == 0 { 1 } else { source2 }
            _ => source2
        };
        self.ui.show_times(&self.times);
        self.ui.set_op(self.op_type);
        self.ui.show_src1(&self.s1);
        self.ui.show_src2(&self.s2);
    }

    pub fn set_station(&mut self, station : Arc<RefCell<ReservedStation>> ) {
        self.station = Some(station);
    }
}
