use std::fmt;


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InstructionType {
    ADD,
    SUB,
    MUL,
    LD,
    DIV,
    JUMP,
}

pub struct Instruction {
    pub _type : InstructionType,
    pub reg1 : u8,
    pub reg2 : u8,
    pub reg3 : u8,
    pub num1 : u32,
    pub num2 : u32,
    pub e_time : Option<u32>,   // 发射周期
    pub r_time : Option<u32>,   // 运行结束周期
    pub w_time : Option<u32>,   // 写回周期
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "_type : {:?} ,e_time : {:?} r_time : {:?} w_time : {:?}", 
               self._type, self.e_time, self.r_time, self.w_time)
    }
}

impl Instruction {
    pub fn get_type(&self) -> InstructionType {
        self._type.clone()
    }

    pub fn get_reg1(&self) -> Option<u8> {
        return Some(self.reg1.clone());
    }

    pub fn get_reg2(&self) -> Option<u8> {
        match self._type {
            InstructionType::ADD => { return Some(self.reg2.clone()); }
            InstructionType::SUB => { return Some(self.reg2.clone()); }
            InstructionType::MUL => { return Some(self.reg2.clone()); }
            InstructionType::DIV => { return Some(self.reg2.clone()); }
            InstructionType::LD => { return None; }
            InstructionType::JUMP => { return None; }
        };
    }

    pub fn get_reg3(&self) -> Option<u8> {
        match self._type {
            InstructionType::ADD => { return Some(self.reg3.clone()); }
            InstructionType::SUB => { return Some(self.reg3.clone()); }
            InstructionType::MUL => { return Some(self.reg3.clone()); }
            InstructionType::DIV => { return Some(self.reg3.clone()); }
            InstructionType::LD => { return None; }
            InstructionType::JUMP => { return None; }
        };
    }

    pub fn get_num1(&self) -> Option<u32> {
        match self._type {
            InstructionType::ADD => { return None; }
            InstructionType::SUB => { return None; }
            InstructionType::MUL => { return None; }
            InstructionType::DIV => { return None; }
            InstructionType::LD => { return Some(self.num1.clone()); }
            InstructionType::JUMP => { return Some(self.num1.clone()); }
        };
    }

    pub fn get_num2(&self) -> Option<u32> {
        match self._type {
            InstructionType::ADD => { return None; }
            InstructionType::SUB => { return None; }
            InstructionType::MUL => { return None; }
            InstructionType::DIV => { return None; }
            InstructionType::LD => { return None; }
            InstructionType::JUMP => { return Some(self.num2.clone()); }
        };
    }
}
