mod instruction;
mod register;
mod calculator;
mod reserved_station;
mod display;

use std::fs::File;
use std::io::{ BufRead, BufReader};
use std::cell::RefCell;
use std::sync::Arc;
use std::boxed::Box;

pub use instruction::*;
use register::Register;
use calculator::Calculator;
use reserved_station::{ ReservedStation, RsState };
use display::*;

extern crate gio;
extern crate gtk;

use gtk::Builder;
use crate::gtk::TextViewExt;
use crate::gtk::TextBufferExt;


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
    write_back_list : Vec<(usize, u32, &'static str)>,
    cycle : gtk::TextView,
}

impl TomasuloSimulator {
    pub fn new(builder : Builder) -> Self{
        let pc : gtk::TextView = builder.get_object("pc").expect("Could not get register1");
        let reg1 : gtk::TextView = builder.get_object("reg1").expect("Could not get register1");
        let reg2 : gtk::TextView = builder.get_object("reg2").expect("Could not get register1");
        let reg3 : gtk::TextView = builder.get_object("reg3").expect("Could not get register1");
        let reg4 : gtk::TextView = builder.get_object("reg4").expect("Could not get register1");
        let reg5 : gtk::TextView = builder.get_object("reg5").expect("Could not get register1");
        let reg6 : gtk::TextView = builder.get_object("reg6").expect("Could not get register1");
        let reg7 : gtk::TextView = builder.get_object("reg7").expect("Could not get register1");
        let reg8 : gtk::TextView = builder.get_object("reg8").expect("Could not get register1");
        let reg9 : gtk::TextView = builder.get_object("reg9").expect("Could not get register1");
        let reg10 : gtk::TextView = builder.get_object("reg10").expect("Could not get register1");
        let reg11 : gtk::TextView = builder.get_object("reg11").expect("Could not get register1");
        let reg12 : gtk::TextView = builder.get_object("reg12").expect("Could not get register1");
        let reg13 : gtk::TextView = builder.get_object("reg13").expect("Could not get register1");
        let reg14 : gtk::TextView = builder.get_object("reg14").expect("Could not get register1");
        let reg15 : gtk::TextView = builder.get_object("reg15").expect("Could not get register1");
        let reg16 : gtk::TextView = builder.get_object("reg16").expect("Could not get register1");
        let reg17 : gtk::TextView = builder.get_object("reg17").expect("Could not get register1");
        let reg18 : gtk::TextView = builder.get_object("reg18").expect("Could not get register1");
        let reg19 : gtk::TextView = builder.get_object("reg19").expect("Could not get register1");
        let reg20 : gtk::TextView = builder.get_object("reg20").expect("Could not get register1");
        let reg21 : gtk::TextView = builder.get_object("reg21").expect("Could not get register1");
        let reg22 : gtk::TextView = builder.get_object("reg22").expect("Could not get register1");
        let reg23 : gtk::TextView = builder.get_object("reg23").expect("Could not get register1");
        let reg24 : gtk::TextView = builder.get_object("reg24").expect("Could not get register1");
        let reg25 : gtk::TextView = builder.get_object("reg25").expect("Could not get register1");
        let reg26 : gtk::TextView = builder.get_object("reg26").expect("Could not get register1");
        let reg27 : gtk::TextView = builder.get_object("reg27").expect("Could not get register1");
        let reg28 : gtk::TextView = builder.get_object("reg28").expect("Could not get register1");
        let reg29 : gtk::TextView = builder.get_object("reg29").expect("Could not get register1");
        let reg30 : gtk::TextView = builder.get_object("reg30").expect("Could not get register1");
        let reg31 : gtk::TextView = builder.get_object("reg31").expect("Could not get register1");
        let reg32 : gtk::TextView = builder.get_object("reg32").expect("Could not get register1");
        let loader_rs1 : Box<dyn ReservedStationDisplay> = Box::new(LoaderBufferUi {
            busy: builder.get_object("LoaderRs1busy").expect("Could not get Loader_rs1busy"),
            value: builder.get_object("LoaderRs1value").expect("Could not get Loader_rs1value"),
        });
        let loader_rs2 : Box<dyn ReservedStationDisplay> = Box::new(LoaderBufferUi {
            busy: builder.get_object("LoaderRs2busy").expect("Could not get Loader_rs2busy"),
            value: builder.get_object("LoaderRs2value").expect("Could not get Loader_rs2value"),
        });
        let loader_rs3 : Box<dyn ReservedStationDisplay> = Box::new(LoaderBufferUi {
            busy: builder.get_object("LoaderRs3busy").expect("Could not get Loader_rs3busy"),
            value: builder.get_object("LoaderRs3value").expect("Could not get Loader_rs3value"),
        });
        let adder_rs1 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::ADD,
            busy: builder.get_object("AdderRs1busy").expect("Could not get Adder_rs1busy"),
            op: builder.get_object("AdderRs1op").expect("Could not get Adder_rs1op"),
            vj: builder.get_object("AdderRs1vj").expect("Could not get Adder_rs1vj"),
            vk: builder.get_object("AdderRs1vk").expect("Could not get Adder_rs1vk"),
            qj: builder.get_object("AdderRs1qj").expect("Could not get Adder_rs1qj"),
            qk: builder.get_object("AdderRs1qk").expect("Could not get Adder_rs1qk")
        });
        let adder_rs2 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::ADD,
            busy: builder.get_object("AdderRs2busy").expect("Could not get Adder_rs2busy"),
            op: builder.get_object("AdderRs2op").expect("Could not get Adder_rs2op"),
            vj: builder.get_object("AdderRs2vj").expect("Could not get Adder_rs2vj"),
            vk: builder.get_object("AdderRs2vk").expect("Could not get Adder_rs2vk"),
            qj: builder.get_object("AdderRs2qj").expect("Could not get Adder_rs2qj"),
            qk: builder.get_object("AdderRs2qk").expect("Could not get Adder_rs2qk")
        });
        let adder_rs3 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::ADD,
            busy: builder.get_object("AdderRs3busy").expect("Could not get Adder_rs3busy"),
            op: builder.get_object("AdderRs3op").expect("Could not get Adder_rs3op"),
            vj: builder.get_object("AdderRs3vj").expect("Could not get Adder_rs3vj"),
            vk: builder.get_object("AdderRs3vk").expect("Could not get Adder_rs3vk"),
            qj: builder.get_object("AdderRs3qj").expect("Could not get Adder_rs3qj"),
            qk: builder.get_object("AdderRs3qk").expect("Could not get Adder_rs3qk")
        });
        let adder_rs4 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::ADD,
            busy: builder.get_object("AdderRs4busy").expect("Could not get Adder_rs4busy"),
            op: builder.get_object("AdderRs4op").expect("Could not get Adder_rs4op"),
            vj: builder.get_object("AdderRs4vj").expect("Could not get Adder_rs4vj"),
            vk: builder.get_object("AdderRs4vk").expect("Could not get Adder_rs4vk"),
            qj: builder.get_object("AdderRs4qj").expect("Could not get Adder_rs4qj"),
            qk: builder.get_object("AdderRs4qk").expect("Could not get Adder_rs4qk")
        });
        let adder_rs5 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::ADD,
            busy: builder.get_object("AdderRs5busy").expect("Could not get Adder_rs5busy"),
            op: builder.get_object("AdderRs5op").expect("Could not get Adder_rs5op"),
            vj: builder.get_object("AdderRs5vj").expect("Could not get Adder_rs5vj"),
            vk: builder.get_object("AdderRs5vk").expect("Could not get Adder_rs5vk"),
            qj: builder.get_object("AdderRs5qj").expect("Could not get Adder_rs5qj"),
            qk: builder.get_object("AdderRs5qk").expect("Could not get Adder_rs5qk")
        });
        let adder_rs6 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::ADD,
            busy: builder.get_object("AdderRs6busy").expect("Could not get Adder_rs6busy"),
            op: builder.get_object("AdderRs6op").expect("Could not get Adder_rs6op"),
            vj: builder.get_object("AdderRs6vj").expect("Could not get Adder_rs6vj"),
            vk: builder.get_object("AdderRs6vk").expect("Could not get Adder_rs6vk"),
            qj: builder.get_object("AdderRs6qj").expect("Could not get Adder_rs6qj"),
            qk: builder.get_object("AdderRs6qk").expect("Could not get Adder_rs6qk")
        });
        let multer_rs1 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::MUL,
            busy: builder.get_object("MulterRs1busy").expect("Could not get Multer_rs1busy"),
            op: builder.get_object("MulterRs1op").expect("Could not get Multer_rs1op"),
            vj: builder.get_object("MulterRs1vj").expect("Could not get Multer_rs1vj"),
            vk: builder.get_object("MulterRs1vk").expect("Could not get Multer_rs1vk"),
            qj: builder.get_object("MulterRs1qj").expect("Could not get Multer_rs1qj"),
            qk: builder.get_object("MulterRs1qk").expect("Could not get Multer_rs1qk")
        });
        let multer_rs2 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::MUL,
            busy: builder.get_object("MulterRs2busy").expect("Could not get Multer_rs2busy"),
            op: builder.get_object("MulterRs2op").expect("Could not get Multer_rs2op"),
            vj: builder.get_object("MulterRs2vj").expect("Could not get Multer_rs2vj"),
            vk: builder.get_object("MulterRs2vk").expect("Could not get Multer_rs2vk"),
            qj: builder.get_object("MulterRs2qj").expect("Could not get Multer_rs2qj"),
            qk: builder.get_object("MulterRs2qk").expect("Could not get Multer_rs2qk")
        });
        let multer_rs3 : Box<dyn ReservedStationDisplay> = Box::new(CalculatorRsUi {
            op_type : InstructionType::MUL,
            busy: builder.get_object("MulterRs3busy").expect("Could not get Multer_rs3busy"),
            op: builder.get_object("MulterRs3op").expect("Could not get Multer_rs3op"),
            vj: builder.get_object("MulterRs3vj").expect("Could not get Multer_rs3vj"),
            vk: builder.get_object("MulterRs3vk").expect("Could not get Multer_rs3vk"),
            qj: builder.get_object("MulterRs3qj").expect("Could not get Multer_rs3qj"),
            qk: builder.get_object("MulterRs3qk").expect("Could not get Multer_rs3qk")
        });
        let loader1 : Box<dyn CalculatorDisplay> = Box::new(LoaderUi {
            target : builder.get_object("Loader1target").expect("Could not get Loader_rs1target"),
            times: builder.get_object("Loader1times").expect("Could not get Loader_rs1times"),
            value: builder.get_object("Loader1value").expect("Could not get Loader_rs1value"),
        });
        let loader2 : Box<dyn CalculatorDisplay> = Box::new(LoaderUi {
            target : builder.get_object("Loader2target").expect("Could not get Loader_rs1target"),
            times: builder.get_object("Loader2times").expect("Could not get Loader_rs1times"),
            value: builder.get_object("Loader2value").expect("Could not get Loader_rs1value"),
        });
        let adder1 : Box<dyn CalculatorDisplay> = Box::new(CalculatorUi {
            op_type : InstructionType::ADD,
            times: builder.get_object("Adder1times").expect("Could not get Adder1times"),
            target: builder.get_object("Adder1target").expect("Could not get Adder1target"),
            op: builder.get_object("Adder1op").expect("Could not get Adder1op"),
            value1: builder.get_object("Adder1value1").expect("Could not get Adder1value1"),
            value2: builder.get_object("Adder1value2").expect("Could not get Adder1value2"),
        });
        let adder2 : Box<dyn CalculatorDisplay> = Box::new(CalculatorUi {
            op_type : InstructionType::ADD,
            times: builder.get_object("Adder2times").expect("Could not get Adder2times"),
            target: builder.get_object("Adder2target").expect("Could not get Adder2target"),
            op: builder.get_object("Adder2op").expect("Could not get Adder2op"),
            value1: builder.get_object("Adder2value1").expect("Could not get Adder2value1"),
            value2: builder.get_object("Adder2value2").expect("Could not get Adder2value2"),
        });
        let adder3 : Box<dyn CalculatorDisplay> = Box::new(CalculatorUi {
            op_type : InstructionType::ADD,
            times: builder.get_object("Adder3times").expect("Could not get Adder3times"),
            target: builder.get_object("Adder3target").expect("Could not get Adder3target"),
            op: builder.get_object("Adder3op").expect("Could not get Adder3op"),
            value1: builder.get_object("Adder3value1").expect("Could not get Adder3value1"),
            value2: builder.get_object("Adder3value2").expect("Could not get Adder3value2"),
        });
        let multer1 : Box<dyn CalculatorDisplay> = Box::new(CalculatorUi {
            op_type : InstructionType::MUL,
            times: builder.get_object("Multer1times").expect("Could not get Multer1times"),
            target: builder.get_object("Multer1target").expect("Could not get Multer1target"),
            op: builder.get_object("Multer1op").expect("Could not get Multer1op"),
            value1: builder.get_object("Multer1value1").expect("Could not get Multer1value1"),
            value2: builder.get_object("Multer1value2").expect("Could not get Multer1value2"),
        });
        let multer2 : Box<dyn CalculatorDisplay> = Box::new(CalculatorUi {
            op_type : InstructionType::MUL,
            times: builder.get_object("Multer2times").expect("Could not get Multer2times"),
            target: builder.get_object("Multer2target").expect("Could not get Multer2target"),
            op: builder.get_object("Multer2op").expect("Could not get Multer2op"),
            value1: builder.get_object("Multer2value1").expect("Could not get Multer2value1"),
            value2: builder.get_object("Multer2value2").expect("Could not get Multer2value2"),
        });
        TomasuloSimulator {
            times : 0,
            inst_vec : Vec::new(),
            registers : [
                Register::new(pc),Register::new(reg1),
                Register::new(reg2),Register::new(reg3),
                Register::new(reg4),Register::new(reg5),
                Register::new(reg6),Register::new(reg7),
                Register::new(reg8),Register::new(reg9),
                Register::new(reg10),Register::new(reg11),
                Register::new(reg12),Register::new(reg13),
                Register::new(reg14),Register::new(reg15),
                Register::new(reg16),Register::new(reg17),
                Register::new(reg18),Register::new(reg19),
                Register::new(reg20),Register::new(reg21),
                Register::new(reg22),Register::new(reg23),
                Register::new(reg24),Register::new(reg25),
                Register::new(reg26),Register::new(reg27),
                Register::new(reg28),Register::new(reg29),
                Register::new(reg30),Register::new(reg31),
                Register::new(reg32),
            ],
            adders : [
                Calculator::new(InstructionType::ADD, adder1),
                Calculator::new(InstructionType::ADD, adder2),
                Calculator::new(InstructionType::ADD, adder3),
            ],
            rs_adders : [
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD, adder_rs1, "AdderRs1"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD, adder_rs2, "AdderRs2"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD, adder_rs3, "AdderRs3"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD, adder_rs4, "AdderRs4"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD, adder_rs5, "AdderRs5"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::ADD, adder_rs6, "AdderRs6"))),
            ],
            multers : [
                Calculator::new(InstructionType::MUL, multer1),
                Calculator::new(InstructionType::MUL, multer2),
            ],
            rs_multers : [
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::MUL, multer_rs1, "MulterRs1"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::MUL, multer_rs2, "MulterRs2"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::MUL, multer_rs3, "MulterRs3"))),
            ],
            loaders : [
                Calculator::new(InstructionType::LD, loader1),
                Calculator::new(InstructionType::LD, loader2),
            ],
            rs_loaders : [
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::LD, loader_rs1, "LoderBuffer1"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::LD, loader_rs2, "LoderBuffer2"))),
                Arc::new(RefCell::new(ReservedStation::new(InstructionType::LD, loader_rs3, "LoderBuffer3"))),
            ],
            write_back_list : Vec::new(),
            cycle : builder.get_object("cycle").expect("Could not get cycle"),
        }
    }

    pub fn show_ui(&self) {
        for i in 0..33 {
            self.registers[i].show();
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
    }

    fn handle_adder_rs(&mut self, rs_id : usize, _type : InstructionType, pc_id : usize) -> bool{
        let (mut rs, rs_ref) = match _type {
            InstructionType::ADD | InstructionType::SUB => (self.rs_adders[rs_id].borrow_mut(),self.rs_adders.get(rs_id).unwrap()),
            InstructionType::MUL | InstructionType::DIV => (self.rs_multers[rs_id].borrow_mut(),self.rs_multers.get(rs_id).unwrap()),
            _ => {
                unimplemented!()
            }
        };
        if rs.state == RsState::FREE {  // 有可用的保留站
            rs.ui.set_op(_type);
            // 第一个操作数
            let source1 = self.inst_vec[pc_id].get_reg2().unwrap() as usize;
            if self.registers[source1].is_waiting() {  // 当前源寄存器被锁定
                self.registers[source1].register_rs(1, rs_ref.clone());
                rs.ui.show_vj(None);
                let name = self.registers[source1].writer_name.as_ref().unwrap();
                rs.ui.show_qj(name);
            } else {
                rs.set_source(1, self.registers[source1].get_value());
            }

            // 第二个操作数
            let source2 = self.inst_vec[pc_id].get_reg3().unwrap() as usize;
            if self.registers[source2].is_waiting() {  // 当前源寄存器被锁定
                self.registers[source2].register_rs(2, rs_ref.clone());
                rs.ui.show_vk(None);
                let name = self.registers[source2].writer_name.as_ref().unwrap();
                rs.ui.show_qk(name);
            } else {
                rs.set_source(2, self.registers[source2].get_value());
            }

            let target = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
            rs.set_target(target);

            rs.set_type(self.inst_vec[pc_id].get_type());
            rs.state = RsState::BUSY;
            rs.ui.show_busy(true);
            self.registers[0].set_value_purlly(pc_id as u32 + 1);
            self.registers[target as usize].set_writer(rs_ref.clone());
            self.registers[target as usize].writer_name = Some(rs.name);
            return true;
        }
        false
    }

    pub fn step(&mut self) {
        self.times += 1;

        for (pos, res, name) in self.write_back_list.iter() {
            if !(self.registers[*pos].writer_name.unwrap() == *name) { continue; }
            if ( *pos == 0 ) && ( *res == 0xFFFF_FFFF ) {
                self.registers[0].clear_writer();
                continue;
            }
            self.registers[*pos].set_value(*res);
        }
        self.write_back_list.clear();
        for i in 0..6 {
            let mut rs = self.rs_adders[i].borrow_mut();
            if rs.state == RsState::CALCULATED {
                rs.refresh();
            }
        }
        for i in 0..3 {
            let mut rs = self.rs_multers[i].borrow_mut();
            if rs.state == RsState::CALCULATED {
                rs.refresh();
            }
        }
        for i in 0..3 {
            let mut rs = self.rs_loaders[i].borrow_mut();
            if rs.state == RsState::CALCULATED {
                rs.refresh();
            }
        }

        // 步进运算部件
        for i in 0..3 {
            if let Some(info) = self.adders[i].step() {
                self.write_back_list.push(info);
            }
        }
        for i in 0..2 {
            if let Some(info) = self.multers[i].step() {
                self.write_back_list.push(info);
            }
        }
        for i in 0..2 {
            if let Some(info) = self.loaders[i].step() {
                self.write_back_list.push(info);
            }
        }

        // 如果有空闲保留站与当前需要发射的指令相符，则发射一条指令
        println!("current cycle : {}", self.times);
        self.cycle.get_buffer().expect("").set_text(&*self.times.to_string());
        if !self.registers[0].is_waiting() {
            let pc_id = self.registers[0].get_value() as usize;
            println!("pc is {}", pc_id);
            if pc_id < self.inst_vec.len() {
                let _type = self.inst_vec[pc_id].get_type();
                match _type {
                    InstructionType::ADD | InstructionType::SUB => {
                        for i in 0..6 {
                            if self.handle_adder_rs(i, _type, pc_id) { break; }
                        }
                    }
                    InstructionType::MUL | InstructionType::DIV => {
                        for i in 0..3 {
                            if self.handle_adder_rs(i, _type, pc_id) { break; }
                        }
                    }
                    InstructionType::LD => {
                        for i in 0..3 {
                            let mut rs = self.rs_loaders[i].borrow_mut();
                            //if !rs.is_busy() {
                            if rs.state == RsState::FREE {
                                let number1 = self.inst_vec[pc_id].get_num1().unwrap();
                                rs.set_source(1, number1);

                                let target = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
                                rs.set_target(target);

                                //rs.set_busy();
                                rs.state = RsState::BUSY;
                                rs.ui.show_busy(true);
                                self.registers[0]
                                    .set_value_purlly(pc_id as u32 + 1);
                                self.registers[target]
                                    .set_writer(self.rs_loaders[i].clone());
                                self.registers[target as usize].writer_name = Some(rs.name);
                                break;
                            }
                        }
                    }
                    InstructionType::JUMP => {
                        for i in 0..6 {
                            let mut rs = self.rs_adders[i].borrow_mut();
                            //if !rs.is_busy() {  // 有可用的保留站
                            if rs.state == RsState::FREE {
                                rs.ui.set_op(self.inst_vec[pc_id].get_type());
                                // 第一个操作数
                                let source1 = self.inst_vec[pc_id].get_num1().unwrap();
                                rs.set_source(1, source1);

                                // 第二个操作数
                                let source2 = self.inst_vec[pc_id].get_reg1().unwrap() as usize;
                                if self.registers[source2].is_waiting() {  // 当前源寄存器被锁定
                                    self.registers[source2].register_rs(2, self.rs_adders[i].clone());
                                    rs.ui.show_vk(None);
                                    rs.ui.show_qk(self.registers[source2].writer_name.unwrap());
                                } else {
                                    rs.set_source(2, self.registers[source2].get_value());
                                }

                                // 跳转的偏移
                                let number = self.inst_vec[pc_id].get_num2().unwrap();
                                let new_pc = ( pc_id as i32 + number as i32) as u32;
                                println!("new pc : {}", new_pc);
                                rs.set_pc_result(new_pc);

                                rs.set_target(0);
                                self.registers[0]
                                    .set_writer(self.rs_adders[i].clone());
                                self.registers[0]
                                    .set_value_purlly(pc_id as u32 + 1);
                                self.registers[0].writer_name = Some(rs.name);

                                rs.set_type(self.inst_vec[pc_id].get_type());
                                //rs.set_busy();
                                rs.state = RsState::BUSY;
                                rs.ui.show_busy(true);
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
            if rs.state == RsState::BUSY {
                if let Some((s1, s2)) = rs.get_all_source() {
                    for j in 0..3 {
                        if !self.adders[j].is_busy() {  // 发现空闲运算加法器
                            self.adders[j].set_instruction(
                                rs.get_type(), s1, s2
                            );
                            self.adders[j].set_station(self.rs_adders[i].clone());
                            self.adders[j].ui.show_target(&rs.target);
                            rs.state = RsState::CALCULATING;
                            break;
                        }
                    }
                }
            }
        }
        for i in 0..3 {
            let mut rs = self.rs_multers[i].borrow_mut();
            if rs.state == RsState::BUSY {
                if let Some((s1, s2)) = rs.get_all_source() {
                    for j in 0..2 {
                        if !self.multers[j].is_busy() {
                            self.multers[j].set_instruction(
                                rs.get_type(), s1, s2
                            );
                            self.multers[j].set_station(self.rs_multers[i].clone());
                            self.multers[j].ui.show_target(&rs.target);
                            rs.state = RsState::CALCULATING;
                            break;
                        }
                    }
                }
            }
        }
        for i in 0..3 {
            let mut rs = self.rs_loaders[i].borrow_mut();
            if rs.state == RsState::BUSY {
                if let Some((s1, _)) = rs.get_all_source() {
                    for j in 0..2 {
                        if !self.loaders[j].is_busy() {
                            self.loaders[j].set_instruction(
                                rs.get_type(), s1, 0
                            );
                            self.loaders[j].set_station(self.rs_loaders[i].clone());
                            self.loaders[j].ui.show_target(&rs.target);
                            rs.state = RsState::CALCULATING;
                            break;
                        }
                    }
                }
            }
        }

        self.show_ui();
    }

    pub fn show(&self) {
    }
}
