use std::fmt;


#[derive(Debug, Clone, Copy)]
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
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "_type : {:?} register1 : {} register2 : {} register3 : {}  number1 : {:#x} number2 : {:#x} ", 
               self._type, self.reg1, self.reg2, self.reg3, self.num1, self.num2)
    }
}

impl Instruction {
    pub fn get_type(&self) -> InstructionType {
        self._type.clone()
    }

    pub fn get_reg1(&self) -> Option<u8> {
        return Some(self.reg1);
    }

    pub fn get_reg2(&self) -> Option<u8> {
        match self._type {
            InstructionType::ADD => { return Some(self.reg2); }
            InstructionType::SUB => { return Some(self.reg2); }
            InstructionType::MUL => { return Some(self.reg2); }
            InstructionType::DIV => { return Some(self.reg2); }
            InstructionType::LD => { return None; }
            InstructionType::JUMP => { return None; }
        };
    }

    pub fn get_reg3(&self) -> Option<u8> {
        match self._type {
            InstructionType::ADD => { return Some(self.reg3); }
            InstructionType::SUB => { return Some(self.reg3); }
            InstructionType::MUL => { return Some(self.reg3); }
            InstructionType::DIV => { return Some(self.reg3); }
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
            InstructionType::LD => { return Some(self.num1); }
            InstructionType::JUMP => { return Some(self.num1); }
        };
    }

    pub fn get_num2(&self) -> Option<u32> {
        match self._type {
            InstructionType::ADD => { return None; }
            InstructionType::SUB => { return None; }
            InstructionType::MUL => { return None; }
            InstructionType::DIV => { return None; }
            InstructionType::LD => { return None; }
            InstructionType::JUMP => { return Some(self.num2); }
        };
    }
}
