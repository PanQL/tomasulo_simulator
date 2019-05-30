use super::reserved_station::ReservedStation;
use super::instruction::InstructionType;
use std::sync::Arc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Calculator {
    id : u8,
    busy : bool, // 是否正在使用
    times : u32, // 剩余计算周期
    op_type : InstructionType, // 计算类型
    s1 : u32, // 操作数1
    s2 : u32, // 操作数2
    station : Option<Arc<RefCell<ReservedStation>>>,    // 对应的保留站
}

impl Calculator {
    pub fn new(id : u8, _type : InstructionType) -> Self{
        Calculator {
            id,
            busy : false,
            times : 0,
            op_type : _type,
            s1 : 0,
            s2 : 0,
            station : None,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.busy
    }

    pub fn step(&mut self) -> Option<(usize, u32)>{
        if !self.busy { return None; }
        if self.times == 0 {    // 写回结束
            self.busy = false;
            return self.write_back();
        } else {
            self.times -= 1;
            return None;
        }
    }

    fn write_back(&mut self) -> Option<(usize, u32)> {
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
            let position = rs.write_back(res);
            println!("s1 : {:?} s2 : {:?} pos : {}", self.s1, self.s2, position);
            match self.op_type {
                InstructionType::JUMP => {
                    ret = if self.s1 == self.s2 {
                        Some((position, rs.get_pc_result().unwrap()))
                    } else {
                        Some((position, 0xFFFF_FFFF))
                    };
                }
                _ => {
                    ret =  Some((position, res));
                }
            }
        } else {
            ret =  None;
        }
        self.station = None;
        ret
    }

    pub fn set_instruction(&mut self, _type : InstructionType, source1 : u32, source2 : u32) {
        self.busy = true;
        self.op_type = _type;
        self.times = match self.op_type {
            InstructionType::ADD => 3,
            InstructionType::SUB => 3,
            InstructionType::MUL => 12,
            InstructionType::DIV => 40,
            InstructionType::LD => 3,
            InstructionType::JUMP => 1,
        };
        self.s1 = source1;
        self.s2 = source2;
    }

    pub fn set_station(&mut self, station : Arc<RefCell<ReservedStation>> ) {
        self.station = Some(station);
    }
}
