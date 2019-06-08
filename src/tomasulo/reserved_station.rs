use super::instruction::InstructionType;
use super::display::ReservedStationDisplay;
use std::sync::Arc;
use std::cell::RefCell;
use std::boxed::Box;

#[derive(Eq, PartialEq)]
pub enum RsState {
    BUSY,
    FREE,
    CALCULATING,
    CALCULATED
}

pub struct ReservedStation {
    pub state : RsState,
    op_type : InstructionType, // 计算类型
    source1 : Option<u32>,
    source2 : Option<u32>,
    waiters : Vec<(Arc<RefCell<ReservedStation>>, u8)>,    // 正在等待该保留站的各个保留站
    pub target : usize, // 要写入的结果寄存器
    pc_result : Option<u32>, // 当指令为jump指令时，可能需要写入pc寄存器的值
    pub ui : Box<dyn ReservedStationDisplay>,   // ui相关成员，用于显示保留站内容
    pub name : &'static str,    // 保留站名称，用于在ui中显示
    res : u32,
}

impl ReservedStation {
    pub fn new(_type : InstructionType, ui : Box<dyn ReservedStationDisplay>, name : &'static str) -> Self{ ReservedStation{
            state : RsState::FREE,
            op_type : _type, 
            source1 : None,  
            source2 : None, 
            waiters : Vec::new(), 
            target : 0, 
            pc_result : None,
            ui,
            name,
            res : 0,
        }
    }

    pub fn set_source(&mut self, idx : u8, source : u32) {  // 将操作数拷贝到保留站对应位置
        if idx == 1 {
            self.ui.show_vj(Some(&source));
            self.ui.show_qj("");
            self.source1 = Some(source);
        } else {
            self.ui.show_vk(Some(&source));
            self.ui.show_qk("");
            self.source2 = Some(source);
        };
    }

    pub fn refresh(&mut self) {     // 将上一周期写回的值更新到其他正在等待的保留站中
        while let Some((rs, idx)) = self.waiters.pop() {
            rs.borrow_mut().set_source(idx, self.res);
        }

        self.waiters.clear();
        self.state = RsState::FREE;
        self.source1 = None;
        self.source2 = None;
        self.state = RsState::FREE;
        self.ui.clear();
    }

    pub fn get_type(&self) -> InstructionType {
        self.op_type.clone()
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
        // 切换保留站状态
        self.state = RsState::CALCULATED;
        // 设置计算结果到保留站
        self.res = res;

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
