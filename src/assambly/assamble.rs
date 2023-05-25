use crate::{assambly::*, utils::{Enumerate, SetGetBytes}};


pub fn pass0(words: Vec<String>) -> CodeTable {
    enum State {
        Argument1,
        Argument2,
        Opcode,
    }
    let mut code_table: CodeTable = CodeTable::new();
    let mut i = 0;
    let mut state = State::Opcode;
    let mut word: &String;
    loop {
        word = &words[i];
        i += 1;
        match state {
            State::Argument1 => {
                // println!("{i}: argument1: {} next word {:?}", word, words.get(i));
                if i >= words.len() {
                    break;
                }
                if words.get(i) == Some(&",".to_string()) {
                    state = State::Argument2;
                    // println!("changed to argument2 with word: {} next_word: {}",word, words[i]);
                    i += 1;
                }
                if word == &"\n".to_string() {
                    code_table.add_line();
                    state = State::Opcode;
                } else if word == &"[".to_string() {
                    // println!("found address!!!");
                    word = &words[i];
                    i += 1;
                    let mut argument = format!("[{}", word);
                    if words[i] == "+" {
                        i += 1;
                        // println!("found register addres");
                        argument.push('+');
                        word = &words[i];
                        i += 1;
                        argument += word;
                    }
                    word = &words[i];
                    i += 1;
                    if word == &"]".to_string() {
                        argument.push(']');
                    }

                    code_table.last_mut().2 = Some(argument);
                    state = State::Argument2;
                } else {
                    if string_is_ident(word) {
                        code_table.last_mut().2 = Some(word.clone());
                    }
                }
            }
            State::Argument2 => {
                // println!("{i}: argument2: {}", word);
                if i >= words.len() {
                    break;
                }
                if word == &"\n".to_string() {
                    code_table.add_line();
                    state = State::Opcode;
                } else if word == &"[".to_string() {
                    // println!("found address!!!");
                    word = &words[i];
                    i += 1;
                    let mut argument = format!("[{}", word);
                    if words[i] == "+" {
                        i += 1;
                        // println!("found register addres");
                        argument.push('+');
                        word = &words[i];
                        i += 1;
                        argument += word;
                    }
                    word = &words[i];
                    i += 1;
                    if word == &"]".to_string() {
                        argument.push(']');
                    }

                    code_table.last_mut().3 = Some(argument);
                } else {
                    if string_is_ident(word) {
                        code_table.last_mut().3 = Some(word.clone());
                    }
                }
            }
            State::Opcode => {
                if i >= words.len() {
                    break;
                }
                let next_word = &words[i];

                if next_word == &":".to_string() {
                    i += 1;
                    code_table.last_mut().0 = Some(word.clone());
                    continue;
                } else {
                    if !string_is_ident(word) {
                        continue;
                    }
                    code_table.last_mut().1 = word.clone();
                    state = State::Argument1;
                    continue;
                }
            }
        };
    }
    return code_table;
}
pub fn pass1(code_table: CodeTable) -> SymbolTable {
    let mut current_byte_location: usize = 0;
    let mut symbol_table: SymbolTable = Dict::new();
    for line in code_table.0 {
        if let Some(label) = line.0 {
            symbol_table.set(label, Some(current_byte_location));
        }
        current_byte_location += 1; // add 1 for the instruction
        if let Some(arg) = line.2 {
            if arg.starts_with("[") {
                current_byte_location += 2;
            } else {
                current_byte_location += 1;
            }
            if !REGISTER_NAMES.contains(&arg.as_str()) && !arg.starts_with("[") {
                if symbol_table.get(arg.clone()).is_none() && !string_is_number(&arg) {
                    symbol_table.set(arg, None)
                }
            }
        }
        if let Some(arg) = line.3 {
            if arg.starts_with("[") {
                current_byte_location += 2;
            } else {
                current_byte_location += 1;
            }
            if !REGISTER_NAMES.contains(&arg.as_str()) && !arg.starts_with("[") {
                if symbol_table.get(arg.clone()).is_none() && !string_is_number(&arg) {
                    symbol_table.set(arg, None)
                }
            }
        }
    }
    println!("total size should be {}", current_byte_location);
    return symbol_table;
}
pub fn pass2(code_table: CodeTable, symbol_table: SymbolTable) -> Result<Vec<u8>, AssamblyError> {
    let mut byte_code: Vec<Option<u8>> = vec![];
    for (i,line) in code_table.0.enumerate() {
        let mut opcode = line.1.to_uppercase();
        if INSTRUCTION_NAMES.contains(&opcode.to_lowercase().as_str()) {
            byte_code.push(None);
            let opcode_index = byte_code.len() - 1;
            // println!("line: {:?}",line);
            for arg in [line.2.clone(),line.3.clone()] {
                if let Some(mut arg) = arg {
                    if arg.starts_with("[") && arg.contains("+") {
                        opcode += "RA";
                        arg.remove(0);
                        arg.remove(arg.len() - 1);
                        let parts = arg.split("+").collect::<Vec<&str>>();
                        if parts.len() != 2 {
                            return Err(AssamblyError::new(Some(i), format!("invalid register address: {}", arg)));
                        }
                        if let Some(reg_id) = REGISTER_NAMES.get_index_of(parts[0]) {
                            byte_code.push(Some(reg_id as u8))
                        } else {
                            return Err(AssamblyError::new(Some(i), format!("invalid register: {}", parts[0])));
                        }
                        let addres = string_to_usize(parts[1].to_string());
                        if addres.is_none() {
                            println!("invalid addres: {}", parts[1]);
                            return Err(AssamblyError::new(Some(i), format!("invalid addres: {}", parts[1])));
                        }
                        let addres = addres.unwrap();
                        byte_code.push(Some(addres as u8));
                    } else if arg.starts_with("[") {
                        opcode += "A";
                        let mut addr_string = arg.clone();
                        addr_string.remove(0);
                        addr_string.remove(addr_string.len()-1);
                        let addr = string_to_usize(addr_string.clone());
                        if addr.is_none() {
                            return Err(AssamblyError::new(Some(i), format!("invalid addr: {}", addr_string)))
                        } else if addr.unwrap() > u16::MAX as usize {
                            return Err(AssamblyError::new(Some(i), format!("addres to larger must be valid u16: {}", addr_string)))
                        }
                        let addr = addr.unwrap();
                        byte_code.push(Some(addr.get_byte(1) as u8));
                        byte_code.push(Some(addr.get_byte(0) as u8));
                    } else {
                        if let Some(reg_id) = REGISTER_NAMES.get_index_of(&arg) {
                            opcode += "R";
                            byte_code.push(Some(reg_id as u8));
                        } else {
                            opcode += "V";
                            if let Some(number) = string_to_usize(arg.clone()) {
                                byte_code.push(Some(number as u8));
                            } else {
                                let label_value = symbol_table.get(arg.clone());
                                if label_value.is_none() {
                                    return Err(AssamblyError::new(Some(i), format!("label is undefind: {}", arg)));
                                    
                                } else {
                                    let label_value = label_value.unwrap().unwrap();
                                    byte_code.push(Some(label_value as u8))
                                }
                            }
                        }
                    }
                }
            }
            
            if let Some(opcode_id) = get_opcode_id(opcode.clone()) {
                // println!("found opcode: {}, opcode_id: {}", opcode, opcode_id);
                byte_code[opcode_index] = Some(opcode_id as u8);
            } else {
                return Err(AssamblyError::new(Some(i), format!("opcode: {} is invalid", opcode)));
            }
        } else {
            return Err(AssamblyError::new(Some(i), format!("invalid instruction: {}", opcode)));
        }
    }
    if byte_code.len()
        != byte_code
            .iter()
            .filter_map(|x| *x)
            .collect::<Vec<u8>>()
            .len()
    {
        println!("something is wrong!!!");
    }
    return Ok(byte_code.iter().filter_map(|x| *x).collect::<Vec<u8>>());
}


pub fn assamble(mut code: String) -> Result<Vec<u8>, AssamblyError> {
    code.make_ascii_lowercase();
    let words: Vec<String> = get_words(code);
    println!("words: {:?}", words);
    let code_table: CodeTable = pass0(words);
    dbg!(&code_table);
    let symbol_table: SymbolTable = pass1(code_table.clone());
    let mut unresolved_labels = vec![];
    
    for key in symbol_table.get_keys() {
        if symbol_table.get(key.clone()).unwrap().is_none() {
            println!("unresolved symbol: {}", key);
            unresolved_labels.push(key)
        }
    }
    if unresolved_labels.len() != 0 {
        return Err(AssamblyError::new(None, format!("unresolved symbols {:?}",unresolved_labels)));
    }
    println!("{:?}",symbol_table);
    let byte_code = pass2(code_table, symbol_table);
    
    // dbg!(&byte_code);
    return byte_code;
}
