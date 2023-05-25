pub mod assambly;
pub mod cpu;
pub mod utils;

pub const MAX_BYTES: usize = 2usize.pow(16);
pub const RAM_BYTES: usize = MAX_BYTES / 128;

#[cfg(test)]
mod test {
    use crate::cpu::memory::{DataBus, RAM};
    use crate::{
        cpu::{self, CPU},
        load_bin_file, RAM_BYTES,
    };

    #[test]
    fn mutlitply_test() {
        let mut cpu = CPU::new();
        let mut data_bus = DataBus::new();
        data_bus.add_driver(Box::new(RAM::new(RAM_BYTES * 8)));
        cpu.mount_data_bus(data_bus);
        let data = load_bin_file("./data/multiply_test.bin".to_string());
        if data.is_err() {
            println!("something went wrong");
            panic!()
        }
        let data = data.unwrap();
        for i in 0..data.len() {
            let byte = data[i];
            cpu.data_bus.write_byte(i, byte);
        }
        for _ in 0..21 {
            cpu.clock();
        }
        use cpu::Register;
        assert_eq!(cpu.registers.read_reg(Register::PC), 100);
        assert_eq!(cpu.registers.read_reg(Register::AX), 6);
        assert_eq!(cpu.registers.read_reg(Register::BX), 2);
        assert_eq!(cpu.registers.read_reg(Register::CX), 5);
        assert_eq!(cpu.registers.read_reg(Register::STP), 511);
    }
    #[test]
    fn stack_test() {
        let mut cpu = CPU::new();
        let mut data_bus = DataBus::new();
        data_bus.add_driver(Box::new(RAM::new(RAM_BYTES * 8)));
        cpu.mount_data_bus(data_bus);
        let data = vec![
            32, 4, 31, 0x11, 31, 0x33, 31, 0x55, 31, 0x77, 31, 0x99, 31, 0xBB, 31, 0xDD, 30, 0, 35,
            0, 0xDD, 49, 0, 0, 30, 0, 35, 0, 0xBB, 49, 0, 0, 30, 0, 35, 0, 0x99, 49, 0, 0, 30, 0,
            35, 0, 0x77, 49, 0, 0,
        ];
        for i in 0..data.len() {
            let byte = data[i];
            cpu.data_bus.write_byte(i, byte);
        }

        for _ in 0..20 {
            cpu.clock();
        }
        use cpu::Register;
        assert_eq!(cpu.registers.read_reg(Register::PC), 48);
    }
}

use std::{fs, io};
pub fn load_bin_file(file_path: String) -> io::Result<Vec<u8>> {
    let content = fs::read(file_path);
    if content.is_ok() {
        println!("{:?}", content)
    }
    return content;
}
pub fn store_bin_file(file_path: String, bin: Vec<u8>) -> Result<(), std::io::Error> {
    fs::write(file_path, bin)
}
