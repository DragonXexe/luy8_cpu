use crate::{assambly::CodeTable, cpu::{instructions::{OPCODES, ArgumentType}, REGISTER_NAMES}, utils::SetGetBytes};

use super::INSTRUCTION_NAMES;


pub fn dissassamble(data: Vec<u8>) -> Option<CodeTable> {
    let mut res_table = CodeTable::new();

    let mut i: usize = 0;
    loop {
        if i >= data.len() {
            break;
        }
        let opcode_id = data[i];
        // println!("[{}]",i);
        let opcode = OPCODES.get(opcode_id as usize);
        if opcode.is_none() {
            println!("[{}] invalid opcode: {:X}",i,opcode_id);
            return None;
        }
        let opcode = opcode.unwrap(); 
        for instruction_name in INSTRUCTION_NAMES {
            if opcode.1.to_lowercase().starts_with(instruction_name) {
                let arg1 = opcode.0.arguments.0;
                let arg2 = opcode.0.arguments.1;
                
                i += 1;
                if i >= data.len() {
                    break;
                }
                let arg1_text: Option<String> = match arg1 {
                    ArgumentType::None => {i -= 1; None},
                    ArgumentType::Value => {
                        Some(format!("{}",data[i]))
                    },
                    ArgumentType::Register => {
                        Some(REGISTER_NAMES[data[i] as usize].to_string())
                    },
                    ArgumentType::Addres => {
                        let addr_high = data[i];
                        i += 1;
                        if i >= data.len() {
                            break;
                        }
                        let addr_low =data[i];
                        let mut addr: usize = 0;
                        addr.set_byte(0, addr_low);
                        addr.set_byte(1, addr_high);
                        Some(format!("[{}]",addr))
                    },
                    ArgumentType::RegAddr => {
                        let reg = data[i];
                        i += 1;
                        if i >= data.len() {
                            break;
                        }
                        let addr = data[i];
                        Some(format!("[{} + {}]",REGISTER_NAMES[reg as usize], addr))
                    },
                };
                i += 1;
                if i >= data.len() {
                    break;
                }
                let arg2_text: Option<String> = match arg2 {
                    ArgumentType::None => {i -= 1; None},
                    ArgumentType::Value => {
                        Some(format!("{}",data[i]))
                    },
                    ArgumentType::Register => {
                        Some(REGISTER_NAMES[data[i] as usize].to_string())
                    },
                    ArgumentType::Addres => {
                        let addr_high = data[i];
                        i += 1;
                        if i >= data.len() {
                            break;
                        }
                        let addr_low =data[i];
                        let mut addr: usize = 0;
                        addr.set_byte(0, addr_low);
                        addr.set_byte(1, addr_high);
                        Some(format!("[{}]",addr))
                    },
                    ArgumentType::RegAddr => {
                        let reg = data[i];
                        i += 1;
                        if i >= data.len() {
                            break;
                        }
                        let addr = data[i];
                        Some(format!("[{} + {}]",REGISTER_NAMES[reg as usize], addr))
                    },
                };
                res_table.add_line();
                let current_line = res_table.last_mut();
                println!("[{}] {}", i, instruction_name);
                current_line.1 = instruction_name.to_string();
                current_line.2 = arg1_text;
                current_line.3 = arg2_text;
                break;
            }
        }
        i += 1;
    }
    return Some(res_table);
}