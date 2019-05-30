use super::instruction::InstructionType;
use std::sync::Arc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct ReservedStation {
    busy : bool, // 是否正在使用
    calculating : bool, // 是否正在计算
    op_type : InstructionType, // 计算类型
    source1 : Option<u32>,
    source2 : Option<u32>,
    waiters : Vec<(Arc<RefCell<ReservedStation>>, u8)>,    // 正在等待该保留站的各个保留站
    target : usize, // 要写入的结果寄存器
    pc_result : Option<u32>, // 当指令为jump指令时，可能需要写入pc寄存器的值
}

impl ReservedStation {
    pub fn new(_type : InstructionType) -> Self{ ReservedStation{
            busy : false, 
            calculating : false, 
            op_type : _type, 
            source1 : None,  
            source2 : None, 
            waiters : Vec::new(), 
            target : 0, 
            pc_result : None,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.busy
    }

    pub fn set_busy(&mut self) {
        self.busy = true;
    }

    pub fn is_calculating(&self) -> bool {
        self.calculating
    }

    pub fn set_calculating(&mut self) {
        self.calculating = true;
    }

    pub fn set_source(&mut self, idx : u8, source : u32) {  // 将操作数拷贝到保留站对应位置
        if idx == 1 { self.source1 = Some(source) } else { self.source2 = Some(source) };
    }

    pub fn get_type(&self) -> InstructionType {
        self.op_type
    }

    pub fn set_type(&mut self , _type : InstructionType) {
        self.op_type = _type;
    }

    pub fn set_target(&mut self, target : usize) {
        self.target = target;
    }

    pub fn get_all_source(&mut self) -> Option<(u32, u32)> {
        match self.op_type {
            InstructionType::ADD | InstructionType::SUB | InstructionType::DIV | InstructionType::MUL | InstructionType::JUMP => {
                if self.source1.is_some() && self.source2.is_some() {
                    return Some((self.source1.unwrap(), self.source2.unwrap()));
                }
            }
            InstructionType::LD => {
                if self.source1.is_some() {
                    return Some((self.source1.unwrap(), 0));
                }
            }
        }
        None
    }
    
    pub fn write_back(&mut self, res : u32) -> usize{
        while let Some((rs, idx)) = self.waiters.pop() {
            rs.borrow_mut().set_source(idx, res);
        }

        self.waiters.clear();
        self.busy = false;
        self.calculating = false;
        self.source1 = None;
        self.source2 = None;

        self.target.clone()
    }

    pub fn register_rs(&mut self, idx : u8, ptr : Arc<RefCell<ReservedStation>>) {
        self.waiters.push((ptr, idx));
    }

    pub fn set_pc_result(&mut self, new_pc : u32) {
        self.pc_result = Some(new_pc);
    }

    pub fn get_pc_result(&self) -> Option<u32> {
        self.pc_result
    }
}
