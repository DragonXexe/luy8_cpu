#[macro_use]
pub mod instructions;
pub mod memory;

use std::fmt::Debug;

use crate::{cpu::instructions::ArgumentType, utils::SetGetBytes, RAM_BYTES};

use self::{
    instructions::{Instruction, OPCODES},
    memory::{BitMap, DataBus},
};

pub const FLAGS_OFFSET: usize = 48;
pub const REGISTER_NAMES: [&str; 10] = [
    "ax", "bx", "cx", "dx", "pc", "pcl", "pch", "flags", "stk", "stp",
];

pub struct CPU {
    pub data_bus: DataBus,
    pub registers: Registers,
    pub instructions: Vec<(Instruction, &'static str)>,
}

impl Debug for CPU {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CPU")
            .field("data_bus", &self.data_bus)
            .field("registers", &self.registers)
            .field("instructions", &self.instructions)
            .finish()
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            data_bus: DataBus::new(),
            registers: Registers::new(),
            instructions: OPCODES.to_vec(),
        }
    }
    pub fn mount_data_bus(&mut self, data_bus: DataBus) {
        self.data_bus = data_bus;
    }
    pub fn clock(&mut self) {
        let pc = self.inc_pc();
        let opcode = self.data_bus.read_byte(pc);
        // println!("opcode: {} pc: {}", opcode, pc);
        self.handle_instruction(opcode);
    }
    pub fn get_pc(&self) -> usize {
        self.registers.read_reg(Register::PC)
    }
    pub fn inc_pc(&mut self) -> usize {
        let pc = self.get_pc();
        self.registers.write_reg(Register::PC, pc + 1);
        return pc;
    }

    fn read_value(&mut self, pc: usize) -> usize {
        return self.data_bus.read_byte(pc) as usize;
    }
    fn read_addres(&mut self, pc: usize) -> usize {
        let addr_high = self.data_bus.read_byte(pc);
        let pc = self.inc_pc();
        let addr_low = self.data_bus.read_byte(pc);
        let mut addr: usize = 0;
        addr.set_byte(0, addr_low);
        addr.set_byte(1, addr_high);
        return addr as usize;
    }
    fn read_register(&mut self, pc: usize) -> usize {
        return self.data_bus.read_byte(pc) as usize;
    }
    fn read_register_addres(&mut self, pc: usize) -> usize {
        let reg: Register = (self.data_bus.read_byte(pc) as usize).into();
        let reg_val = self.registers.read_reg(reg);
        let pc = self.inc_pc();
        let val = self.data_bus.read_byte(pc) as usize;
        let addr = reg_val + val;
        return addr as usize;
    }

    pub fn handle_instruction(&mut self, opcode: u8) {
        if opcode as usize > self.instructions.len() {
            return;
        }
        let start_pc = self.registers.read_reg(Register::PC);
        let instruction = self.instructions[opcode as usize].clone();
        // println!("opcode: {} instruction: {}",opcode, instruction.1);
        let arg1 = match instruction.0.arguments.0 {
            ArgumentType::None => 0,
            _ => {
                // println!("first argument");
                let pc = self.inc_pc();
                match instruction.0.arguments.0 {
                    ArgumentType::None => 0,                                // 0-bit
                    ArgumentType::Value => self.read_value(pc),             // 8-bit
                    ArgumentType::Addres => self.read_addres(pc),           // 16-bit
                    ArgumentType::Register => self.read_register(pc),       // 8-bit
                    ArgumentType::RegAddr => self.read_register_addres(pc), // 16-bit
                }
            }
        };
        let arg2 = match instruction.0.arguments.1 {
            ArgumentType::None => 0,
            _ => {
                // println!("second argument");
                let pc = self.inc_pc();
                match instruction.0.arguments.1 {
                    ArgumentType::None => 0,                                // 0-bit
                    ArgumentType::Value => self.read_value(pc),             // 8-bit
                    ArgumentType::Addres => self.read_addres(pc),           // 16-bit
                    ArgumentType::Register => self.read_register(pc),       // 8-bit
                    ArgumentType::RegAddr => self.read_register_addres(pc), // 16-bit
                }
            }
        };
        println!("{}: {} {}, {}", start_pc, instruction.1, arg1, arg2);
        (instruction.0.handler)(self, arg1, arg2)
    }
}

pub struct Registers {
    bit_map: BitMap,
}
impl Registers {
    pub fn new() -> Self {
        let mut new = Self {
            bit_map: BitMap::new(88),
        };
        new.write_reg(Register::STK, RAM_BYTES - 1);
        new.write_reg(Register::STP, RAM_BYTES - 1);
        return new;
    }
    pub fn read_reg(&self, register: Register) -> usize {
        match register {
            Register::AX => self.bit_map.read_byte(0) as usize,
            Register::BX => self.bit_map.read_byte(1) as usize,
            Register::CX => self.bit_map.read_byte(2) as usize,
            Register::DX => self.bit_map.read_byte(3) as usize,
            Register::PC => {
                (self.bit_map.read_byte(4) as usize) | ((self.bit_map.read_byte(5) as usize) << 8)
            }
            Register::PCL => self.bit_map.read_byte(4) as usize,
            Register::PCH => self.bit_map.read_byte(5) as usize,
            Register::FLAGS => self.bit_map.read_byte(6) as usize,
            Register::STK => {
                (self.bit_map.read_byte(7) as usize) | ((self.bit_map.read_byte(8) as usize) << 8)
            }
            Register::STP => {
                (self.bit_map.read_byte(9) as usize) | ((self.bit_map.read_byte(10) as usize) << 8)
            }
        }
    }
    pub fn write_reg(&mut self, register: Register, data: usize) {
        // println!("register: {:?}, data: {}",register, data);
        // if register == Register::STP {
        //     println!("register: {:?}, data: {}",register, data);
        // }
        match register {
            Register::AX => self.bit_map.write_byte(0, data as u8),
            Register::BX => self.bit_map.write_byte(1, data as u8),
            Register::CX => self.bit_map.write_byte(2, data as u8),
            Register::DX => self.bit_map.write_byte(3, data as u8),
            Register::PC => {
                self.bit_map.write_byte(4, data.get_byte(0));
                self.bit_map.write_byte(5, data.get_byte(1))
            }
            Register::PCL => self.bit_map.write_byte(4, data as u8),
            Register::PCH => self.bit_map.write_byte(5, data as u8),
            Register::FLAGS => self.bit_map.write_byte(6, data as u8),
            Register::STK => {
                self.bit_map.write_byte(7, data.get_byte(0));
                self.bit_map.write_byte(8, data.get_byte(1))
            }
            Register::STP => {
                self.bit_map.write_byte(9, data.get_byte(0));
                self.bit_map.write_byte(10, data.get_byte(1))
            }
        };
    }
    pub fn set_eq(&mut self, val: bool) {
        self.bit_map.set_bit(FLAGS_OFFSET, val);
    }
    pub fn get_eq(&self) -> bool {
        return self.bit_map.get_bit(FLAGS_OFFSET);
    }
    pub fn set_mr(&mut self, val: bool) {
        self.bit_map.set_bit(FLAGS_OFFSET + 1, val);
    }
    pub fn get_mr(&self) -> bool {
        return self.bit_map.get_bit(FLAGS_OFFSET + 1);
    }
    pub fn set_ls(&mut self, val: bool) {
        self.bit_map.set_bit(FLAGS_OFFSET + 2, val);
    }
    pub fn get_ls(&self) -> bool {
        return self.bit_map.get_bit(FLAGS_OFFSET + 2);
    }
    pub fn set_zero(&mut self, val: bool) {
        self.bit_map.set_bit(FLAGS_OFFSET + 3, val);
    }
    pub fn get_zero(&self) -> bool {
        return self.bit_map.get_bit(FLAGS_OFFSET + 3);
    }
    pub fn set_carry(&mut self, val: bool) {
        self.bit_map.set_bit(FLAGS_OFFSET + 4, val);
    }
    pub fn get_carry(&self) -> bool {
        return self.bit_map.get_bit(FLAGS_OFFSET + 4);
    }
}
impl Debug for Registers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registers")
            .field("ax", &self.read_reg(Register::AX))
            .field("bx", &self.read_reg(Register::BX))
            .field("cx", &self.read_reg(Register::CX))
            .field("dx", &self.read_reg(Register::DX))
            .field("pc", &self.read_reg(Register::PC))
            .field("pcl", &self.read_reg(Register::PCL))
            .field("pch", &self.read_reg(Register::PCH))
            .field("flags", &self.read_reg(Register::FLAGS))
            .field("stk", &self.read_reg(Register::STK))
            .field("stp", &self.read_reg(Register::STP))
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Register {
    AX = 0,
    BX = 1,
    CX = 2,
    DX = 3,
    PC = 4,
    PCL = 5,
    PCH = 6,
    FLAGS = 7,
    STP = 8,
    STK = 9,
}
impl Into<Register> for usize {
    fn into(self) -> Register {
        match self {
            0 => Register::AX,
            1 => Register::BX,
            2 => Register::CX,
            3 => Register::DX,
            4 => Register::PC,
            5 => Register::PCL,
            6 => Register::PCH,
            7 => Register::FLAGS,
            8 => Register::STK,
            9 => Register::STP,
            _ => Register::AX,
        }
    }
}
impl From<Register> for usize {
    fn from(value: Register) -> Self {
        return value as Self;
    }
}
