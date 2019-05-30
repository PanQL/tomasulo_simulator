mod instruction;
mod register;
mod calculator;
mod reserved_station;

use std::fs::File;
use std::io::{ BufRead, BufReader};
use std::cell::RefCell;
use std::sync::Arc;

use instruction::*;
use register::Register;
use calculator::Calculator;
use reserved_station::ReservedStation;

pub struct TomasuloSimulator {
    times : u32, //代表当前正处于第几个周期
    // 0号寄存器代表当前指令所在位置
    inst_vec : Vec<Instruction>,
    registers : [Register; 32 + 1],
    adders : [Calculator; 3],
    rs_adders : [Arc<RefCell<ReservedStation>>; 6],
    multers : [Calculator; 2],
    rs_multers : [Arc<RefCell<ReservedStation>>; 3],
    loaders : [Calculator; 2],
    rs_loaders : [Arc<RefCell<ReservedStation>>; 3],
}

impl TomasuloSimulator {
    pub fn new() -> Self{
        TomasuloSimulator {
            times : 0,
            inst_vec : Vec::new(),
            registers : [
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),Register::new(),
                Register::new(),
            ],
            adders : [
                Calculator::new(0, InstructionType::ADD),
                Calculator::new(1, InstructionType::ADD),
                Calculator::new(2, InstructionType::ADD),
            ],
            rs_adders : [
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD))),
            ],
            multers : [
                Calculator::new(3, InstructionType::MUL),
                Calculator::new(4, InstructionType::MUL),
            ],
            rs_multers : [
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::MUL))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::MUL))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::MUL))),
            ],
            loaders : [
                Calculator::new(5, InstructionType::LD),
                Calculator::new(6, InstructionType::LD),
            ],
            rs_loaders : [
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::LD))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::LD))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::LD))),
            ],
        }
    }

    pub fn load_nel(&mut self, filename : &str) {
        let file = File::open(filename).expect("failed to open file");
        let reader = BufReader::new(file);

        for (_, line) in reader.lines().enumerate() {
            let line = line.expect("failed to read a line"); // Ignore errors.
            let line_vec = line.split(",").collect::<Vec<&str>>();
            println!("line {}", line);
            match line_vec[0] {
                "ADD" => { 
                    let r1 : u8 = (&line_vec[1][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let r2 : u8 = (&line_vec[2][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let r3 : u8 = (&line_vec[3][1..]).parse()
                        .expect("failed to parse a string to int ");
                    self.inst_vec.push(Instruction{
                        _type : InstructionType::ADD,
                        reg1 : r1, 
                        reg2 : r2, 
                        reg3 : r3, 
                        num1 : 0,
                        num2 : 0,
                    });
                }
                "SUB" => { 
                    let r1 : u8 = (&line_vec[1][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let r2 : u8 = (&line_vec[2][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let r3 : u8 = (&line_vec[3][1..]).parse()
                        .expect("failed to parse a string to int ");
                    self.inst_vec.push(Instruction{
                        _type : InstructionType::SUB,
                        reg1 : r1, 
                        reg2 : r2, 
                        reg3 : r3, 
                        num1 : 0,
                        num2 : 0,
                    });
                }
                "MUL" => { 
                    let r1 : u8 = (&line_vec[1][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let r2 : u8 = (&line_vec[2][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let r3 : u8 = (&line_vec[3][1..]).parse()
                        .expect("failed to parse a string to int ");
                    self.inst_vec.push(Instruction{
                        _type : InstructionType::MUL,
                        reg1 : r1, 
                        reg2 : r2, 
                        reg3 : r3, 
                        num1 : 0,
                        num2 : 0,
                    });
                }
                "DIV" => { 
                    let r1 : u8 = (&line_vec[1][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let r2 : u8 = (&line_vec[2][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let r3 : u8 = (&line_vec[3][1..]).parse()
                        .expect("failed to parse a string to int ");
                    self.inst_vec.push(Instruction{
                        _type : InstructionType::DIV,
                        reg1 : r1, 
                        reg2 : r2, 
                        reg3 : r3, 
                        num1 : 0,
                        num2 : 0,
                    });
                }
                "LD" => { 
                    let r1 : u8 = (&line_vec[1][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let n1 : u32 = u32::from_str_radix(&line_vec[2][2..], 16).expect("failed to parse u32");
                    self.inst_vec.push(Instruction{
                        _type : InstructionType::LD,
                        reg1 : r1, 
                        reg2 : 0, 
                        reg3 : 0, 
                        num1 : n1,
                        num2 : 0,
                    });
                }
                "JUMP" => { 
                    let r1 : u8 = (&line_vec[2][1..]).parse()
                        .expect("failed to parse a string to int ");
                    let n1 : u32 = u32::from_str_radix(&line_vec[1][2..], 16).expect("failed to parse u32");
                    let n2 : u32 = u32::from_str_radix(&line_vec[3][2..], 16).expect("failed to parse u32");
                    self.inst_vec.push(Instruction{
                        _type : InstructionType::JUMP,
                        reg1 : r1, 
                        reg2 : 0, 
                        reg3 : 0, 
                        num1 : n1,
                        num2 : n2,
                    });
                }
                _ => { println!("unknown type instruction : {}", line); }
            }
        }
        print!("{:?}", self.inst_vec);
    }

    pub fn step(&mut self) {
        self.times += 1;

        // 步进运算部件
        for i in 0..3 {
            if let Some((pos, res)) = self.adders[i].step() {
                if ( pos == 0 ) && ( res == 0xFFFF_FFFF ) {
                    self.registers[0].clear_writer();
                    continue;
                }
                self.registers[pos].set_value(res);
            }
        }
        for i in 0..2 {
            //self.multers[i].step();
            if let Some((pos, res)) = self.multers[i].step() {
                self.registers[pos].set_value(res);
            }
        }
        for i in 0..2 {
            //self.loaders[i].step();
            if let Some((pos, res)) = self.loaders[i].step() {
                self.registers[pos].set_value(res);
            }
        }

        // 如果有空闲保留站与当前需要发射的指令相符，则发射一条指令
        println!("current cycle : {}", self.times);
        if !self.registers[0].is_waiting() {
            let pc_id = self.registers[0].get_value() as usize;
            println!("pc is {} {:?}", pc_id, self.inst_vec[pc_id]);
            if ! (pc_id == (self.inst_vec.len() - 1)) {
                match self.inst_vec[pc_id].get_type() {
                    InstructionType::ADD => {
                        for i in 0..6 {
                            let mut rs = self.rs_adders[i].borrow_mut();
                            if !rs.is_busy() {  // 有可用的保留站
                                // 第一个操作数
                                let source1 = self.inst_vec[pc_id].get_reg2().unwrap() as usize;
                                if self.registers[source1].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source1].register_rs(1, self.rs_adders[i].clone());
                                } else {
                                    rs.set_source(1, self.registers[source1].get_value());
                                }

                                // 第二个操作数
                                let source2 = self.inst_vec[pc_id].get_reg3().unwrap() as usize;
                                if self.registers[source2].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source2].register_rs(2, self.rs_adders[i].clone());
                                } else {
                                    rs.set_source(2, self.registers[source2].get_value());
                                }

                                let target = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
                                rs.set_target(target);

                                rs.set_type(self.inst_vec[pc_id].get_type());
                                rs.set_busy();
                                self.registers[0]
                                    .set_value_purlly(pc_id as u32 + 1);
                                self.registers[target as usize]
                                    .set_writer(self.rs_adders[i].clone());
                                break;
                            }
                        }
                    }
                    InstructionType::SUB => {
                        for i in 0..6 {
                            let mut rs = self.rs_adders[i].borrow_mut();
                            if !rs.is_busy() {  // 有可用的保留站
                                // 第一个操作数
                                let source1 = self.inst_vec[pc_id].get_reg2().unwrap() as usize;
                                if self.registers[source1].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source1].register_rs(1, self.rs_adders[i].clone());
                                } else {
                                    rs.set_source(1, self.registers[source1].get_value());
                                }

                                // 第二个操作数
                                let source2 = self.inst_vec[pc_id].get_reg3().unwrap() as usize;
                                if self.registers[source2].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source2].register_rs(2, self.rs_adders[i].clone());
                                } else {
                                    rs.set_source(2, self.registers[source2].get_value());
                                }

                                println!("{}", 1);
                                let target = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
                                rs.set_target(target);

                                rs.set_type(self.inst_vec[pc_id].get_type());
                                rs.set_busy();
                                self.registers[0]
                                    .set_value_purlly(pc_id as u32 + 1);
                                self.registers[target as usize]
                                    .set_writer(self.rs_adders[i].clone());
                                break;
                            }
                        }
                    }
                    InstructionType::MUL => {
                        for i in 0..3 {
                            let mut rs = self.rs_multers[i].borrow_mut();
                            if !rs.is_busy() {  // 有可用的保留站
                                // 第一个操作数
                                let source1 = self.inst_vec[pc_id].get_reg2().unwrap() as usize;
                                if self.registers[source1].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source1].register_rs(1, self.rs_multers[i].clone());
                                } else {
                                    rs.set_source(1, self.registers[source1].get_value());
                                }

                                // 第二个操作数
                                let source2 = self.inst_vec[pc_id].get_reg3().unwrap() as usize;
                                if self.registers[source2].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source2].register_rs(2, self.rs_multers[i].clone());
                                } else {
                                    rs.set_source(2, self.registers[source2].get_value());
                                }

                                let target = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
                                rs.set_target(target);

                                rs.set_type(self.inst_vec[pc_id].get_type());
                                rs.set_busy();
                                self.registers[0]
                                    .set_value_purlly(pc_id as u32 + 1);
                                self.registers[target as usize]
                                    .set_writer(self.rs_multers[i].clone());
                                break;
                            }
                        }
                    }
                    InstructionType::DIV => {
                        for i in 0..3 {
                            let mut rs = self.rs_multers[i].borrow_mut();
                            if !rs.is_busy() {  // 有可用的保留站
                                // 第一个操作数
                                let source1 = self.inst_vec[pc_id].get_reg2().unwrap() as usize;
                                if self.registers[source1].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source1].register_rs(1, self.rs_multers[i].clone());
                                } else {
                                    rs.set_source(1, self.registers[source1].get_value());
                                }

                                // 第二个操作数
                                let source2 = self.inst_vec[pc_id].get_reg3().unwrap() as usize;
                                if self.registers[source2].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source2].register_rs(2, self.rs_multers[i].clone());
                                } else {
                                    rs.set_source(2, self.registers[source2].get_value());
                                }

                                let target = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
                                rs.set_target(target);

                                rs.set_type(self.inst_vec[pc_id].get_type());
                                rs.set_busy();
                                self.registers[0]
                                    .set_value_purlly(pc_id as u32 + 1);
                                self.registers[target]
                                    .set_writer(self.rs_multers[i].clone());
                                break;
                            }
                        }
                    }
                    InstructionType::LD => {
                        for i in 0..3 {
                            let mut rs = self.rs_loaders[i].borrow_mut();
                            if !rs.is_busy() {
                                let number1 = self.inst_vec[pc_id].get_num1().unwrap();
                                rs.set_source(1, number1);

                                let target = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
                                rs.set_target(target);

                                rs.set_busy();
                                self.registers[0]
                                    .set_value_purlly(pc_id as u32 + 1);
                                self.registers[target]
                                    .set_writer(self.rs_loaders[i].clone());
                                break;
                            }
                        }
                    }
                    InstructionType::JUMP => {
                        for i in 0..6 {
                            let mut rs = self.rs_adders[i].borrow_mut();
                            if !rs.is_busy() {  // 有可用的保留站
                                // 第一个操作数
                                let source1 = self.inst_vec[pc_id].get_num1().unwrap();
                                rs.set_source(1, source1);

                                // 第二个操作数
                                let source2 = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
                                if self.registers[source2].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source2].register_rs(2, self.rs_adders[i].clone());
                                } else {
                                    rs.set_source(2, self.registers[source2].get_value());
                                }

                                // 跳转的偏移
                                let number = self.inst_vec[pc_id].get_num2().unwrap();
                                let new_pc = ( pc_id as i32 + number as i32) as u32;
                                rs.set_pc_result(new_pc);

                                rs.set_target(0);
                                self.registers[0]
                                    .set_writer(self.rs_adders[i].clone());
                                self.registers[0]
                                    .set_value_purlly(pc_id as u32 + 1);

                                rs.set_type(self.inst_vec[pc_id].get_type());
                                rs.set_busy();
                                break;
                            }
                        }
                    }
                }
            }
        }

        // 遍历保留站，执行可以执行的指令
        for i in 0..6 {
            let mut rs = self.rs_adders[i].borrow_mut();
            if rs.is_busy() && !rs.is_calculating() {
                if let Some((s1, s2)) = rs.get_all_source() {
                    for j in 0..3 {
                        if !self.adders[j].is_busy() {  // 发现空闲运算加法器
                            self.adders[j].set_instruction(
                                rs.get_type(), s1, s2
                            );
                            self.adders[j].set_station(self.rs_adders[i].clone());
                            rs.set_calculating();
                            break;
                        }
                    }
                }
            }
        }
        for i in 0..3 {
            let mut rs = self.rs_multers[i].borrow_mut();
            if rs.is_busy() && !rs.is_calculating() {
                if let Some((s1, s2)) = rs.get_all_source() {
                    for j in 0..2 {
                        if !self.multers[j].is_busy() {
                            self.multers[j].set_instruction(
                                rs.get_type(), s1, s2
                            );
                            self.multers[j].set_station(self.rs_multers[i].clone());
                            rs.set_calculating();
                            break;
                        }
                    }
                }
            }
        }
        for i in 0..3 {
            let mut rs = self.rs_loaders[i].borrow_mut();
            if rs.is_busy() && !rs.is_calculating() {
                if let Some((s1, _)) = rs.get_all_source() {
                    for j in 0..2 {
                        if !self.loaders[j].is_busy() {
                            self.loaders[j].set_instruction(
                                rs.get_type(), s1, 0
                            );
                            self.loaders[j].set_station(self.rs_loaders[i].clone());
                            rs.set_calculating();
                            break;
                        }
                    }
                }
            }
        }

        for i in 0..6 {
            print!("{:?} \n", self.rs_adders[i]); 
        }
        for i in 0..3 {
            print!("{:?} \n", self.rs_multers[i]); 
        }
        for i in 0..3 {
            print!("{:?} \n", self.rs_loaders[i]); 
        }
        for i in 0..3 {
            print!("{:?} \n", self.adders[i]); 
        }
        for i in 0..2 {
            print!("{:?} \n", self.multers[i]); 
        }
        for i in 0..2 {
            print!("{:?} \n", self.loaders[i]); 
        }
        for i in 0..8 {
            let reg1 = &self.registers[1 + 4 * i];
            let reg2 = &self.registers[1 + 4 * i + 1];
            let reg3 = &self.registers[1 + 4 * i + 2];
            let reg4 = &self.registers[1 + 4 * i + 3];
            print!("Register {}:{:?} {:?} {}:{:?} {:?} {}:{:?} {:?} {}:{:?} {:?} \n", 
                   1 + 4 * i, reg1.get_value(), reg1.get_writer(), 
                   1 + 4 * i + 1, reg2.get_value(), reg2.get_writer(), 
                   1 + 4 * i + 2, reg3.get_value(), reg3.get_writer(), 
                   1 + 4 * i + 3, reg4.get_value(), reg4.get_writer());
        }

    }

    pub fn show(&self) {
        print!("{:?}", self.adders);
        print!("{:?}", self.multers);
        print!("{:?}", self.loaders);
    }
}
