mod assamble;
mod dissasamble;


type Line = (Option<String>, String, Option<String>, Option<String>);
type SymbolTable = Dict<String, Option<usize>>;
use crate::{cpu::REGISTER_NAMES};
pub const INSTRUCTION_NAMES: [&str; 21] = [
    "nop", "add", "sub", "and", "or", "xor", "not", "mov", "str", "pop", "push", "cmp", "jmp",
    "je", "jz", "jne", "jmr", "jls", "int", "call", "ret",
];
#[derive(Clone)]
pub struct CodeTable(pub Vec<Line>);
impl CodeTable {
    pub fn new() -> Self {
        Self(vec![(None, String::new(), None, None)])
    }
    pub fn add_line(&mut self) {
        self.0.push((None, String::new(), None, None));
    }
    pub fn last_mut(&mut self) -> &mut Line {
        self.0.last_mut().unwrap()
    }
}

use std::fmt::{Debug, Display};

use crate::cpu::instructions::OPCODES;
impl Debug for CodeTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lines = &self.0;
        let mut dbg_lines: Vec<String> = vec![];
        for line in lines {
            dbg_lines.push(match &line {
                (None, instruction, None, None) => {
                    format!("{:20}{:20}", "", format!("({})", instruction))
                }
                (None, instruction, Some(arg1), None) => format!(
                    "{:20}{:20}{:12}",
                    "",
                    format!("({})", instruction),
                    format!("({})", arg1)
                ),
                (None, instruction, Some(arg1), Some(arg2)) => format!(
                    "{:20}{:20}{:12}{:10}",
                    "",
                    format!("({})", instruction),
                    format!("({})", arg1),
                    format!("({})", arg2)
                ),
                (Some(label), instruction, None, None) => format!(
                    "{:20}{:20}",
                    format!("({})", label),
                    format!("({})", instruction)
                ),
                (Some(label), instruction, Some(arg1), None) => format!(
                    "{:20}{:20}{:12}",
                    format!("({})", label),
                    format!("({})", instruction),
                    format!("({})", arg1)
                ),
                (Some(label), instruction, Some(arg1), Some(arg2)) => format!(
                    "{:20}{:20}{:12}{:10}",
                    format!("({})", label),
                    format!("({})", instruction),
                    format!("({})", arg1),
                    format!("({})", arg2)
                ),
                err => format!("{:?}", err),
            });
        }
        let mut debug_tuple = f.debug_tuple("SymbolTable");
        for line in dbg_lines {
            debug_tuple.field(&format!("{}", line));
        }
        debug_tuple.finish()
    }
}
pub struct Dict<K: PartialEq + PartialOrd + Debug + Clone, V: Clone + Debug> {
    keys: Vec<K>,
    vals: Vec<V>,
}
impl<K, V> Dict<K, V>
where
    K: std::cmp::PartialOrd + Debug + std::clone::Clone,
    V: Clone + Debug,
{
    pub fn new() -> Self {
        Self {
            keys: vec![],
            vals: vec![],
        }
    }
    pub fn set(&mut self, key: K, val: V) {
        for i in 0..self.keys.len() {
            if key == self.keys[i] {
                self.vals[i] = val;
                return;
            }
        }
        self.keys.push(key);
        self.vals.push(val)
    }
    pub fn get(&self, key: K) -> Option<V> {
        for i in 0..self.keys.len() {
            if key == self.keys[i] {
                return Some(self.vals[i].clone());
            }
        }
        return None;
    }
    pub fn get_vals(&self) -> Vec<V> {
        self.vals.clone()
    }
    pub fn get_keys(&self) -> Vec<K> {
        self.keys.clone()
    }
}
impl<K, V> Debug for Dict<K, V>

where
    K: std::cmp::PartialOrd + std::fmt::Debug + std::clone::Clone,
    V: std::clone::Clone + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut dbg = f.debug_struct("Dict");
        for i in 0..self.keys.len() {
            dbg.field(format!("{:?}", self.keys[i]).as_str(), &self.vals[i]);
        }
        return dbg.finish();
    }
}
trait GetIndexOf<T> {
    fn get_index_of(&self, val: T) -> Option<usize>;
}
impl<T> GetIndexOf<T> for Vec<T>
where
    T: PartialEq<T>,
{
    fn get_index_of(&self, val: T) -> Option<usize> {
        for i in 0..self.len() {
            if self[i] == val {
                return Some(i);
            }
        }
        return None;
    }
}
impl<T> GetIndexOf<T> for [T]
where
    T: PartialEq<T>,
{
    fn get_index_of(&self, val: T) -> Option<usize> {
        for i in 0..self.len() {
            if self[i] == val {
                return Some(i);
            }
        }
        return None;
    }
}

pub fn string_is_alphanumeric(string: &String) -> bool {
    string
        .chars()
        .filter(|x| x.is_alphanumeric())
        .collect::<Vec<char>>()
        .len()
        == string.len()
}
pub fn string_is_numeric(string: &String) -> bool {
    string
        .chars()
        .filter(|x| x.is_numeric())
        .collect::<Vec<char>>()
        .len()
        == string.len()
}
pub fn string_has_white_space(string: &String) -> bool {
    string
        .chars()
        .filter(|x| x.is_whitespace())
        .collect::<Vec<char>>()
        .len()
        == string.len()
}
pub fn string_is_ident(string: &String) -> bool {
    return !string.contains(":") && !string_has_white_space(string) && !string.contains(",");
}
pub fn string_is_number(string: &String) -> bool {
    if string.starts_with("0x") {
        return usize::from_str_radix(string.as_str(), 16).is_ok();
    } else if string.starts_with("0b") {
        return usize::from_str_radix(string.as_str(), 2).is_ok();
    } else {
        return string_is_numeric(string);
    }
}
pub fn string_to_usize(string: String) -> Option<usize> {
    if string.starts_with("0x") {
        let res = string.strip_prefix("0x");
        if res.is_none() {
            return None;
        } else {
            let res = res.unwrap();
            let res = usize::from_str_radix(res, 16);
            if res.is_ok() {
                return Some(res.unwrap());
            } else {
                return None;
            }
        }
    } else if string.starts_with("0b") {
        let res = string.strip_prefix("0b");
        if res.is_none() {
            return None;
        } else {
            let res = res.unwrap();
            let res = usize::from_str_radix(res, 2);
            if res.is_ok() {
                return Some(res.unwrap());
            } else {
                return None;
            }
        }
    } else {
        let res = usize::from_str_radix(&string, 10);
        if res.is_ok() {
            return Some(res.unwrap());
        } else {
            return None;
        }
    }
}
pub fn get_words(mut code: String) -> Vec<String> {
    code = code.to_lowercase();
    let mut words: Vec<String> = vec![];
    let mut word: String = String::new();
    let mut instring = false;
    for ch in code.chars() {
        if ch == '"' {
            if instring {
                word.push(ch);
                instring = false;
                continue;
            } else {
                instring = true;
            }
        }
        if instring {
            word.push(ch);
            continue;
        }
        if ch.is_whitespace() {
            if !word.is_empty() {
                words.push(word.clone());
            }
            if ch == '\n' {
                // println!("found new_line");
                words.push("\n".to_string());
            }
            word = String::new();
            continue;
        }
        if [',', ';', '+', '[', ']', ':', '/'].contains(&ch) {
            for special_char in [',', ';', '+', '[', ']', ':', '/'] {
                if ch == special_char {
                    if !word.is_empty() {
                        words.push(word.clone());
                    }
                    word = String::new();
                    words.push(ch.to_string());
                    break;
                }
            }
        } else {
            word.push(ch);
        }
    }
    if !word.is_empty() {
        words.push(word.clone());
    }
    return words;
}
pub fn get_opcode_id(opcode: String) -> Option<usize> {
    for i in 0..OPCODES.len() {
        if OPCODES[i].1 == opcode.as_str() {
            return Some(i);
        }
    }
    return None;
}

#[derive(Debug, Clone)]
pub struct AssamblyError {
    msg: String,
    line: Option<usize>,
}
impl AssamblyError {
    pub fn new(line: Option<usize>, msg: String) -> Self {
        Self {
            msg,
            line,
        }
    }

}
impl Display for AssamblyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.line {
            Some(line) => write!(f, "[{}] {}", line, self.msg),
            None => write!(f, "{}", self.msg)
        }
        
    }
}



pub fn assamble(code: String) -> Result<Vec<u8>, AssamblyError> {
    assamble::assamble(code)
}
pub fn dissassamble(data: Vec<u8>) -> Option<CodeTable> {
    dissasamble::dissassamble(data)
}