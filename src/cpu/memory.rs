use std::fmt::Debug;

#[derive(Debug)]
pub struct DataBus {
    pub drivers: Vec<Box<dyn DataDriver>>,
}
impl DataBus {
    pub fn new() -> Self {
        DataBus { drivers: vec![] }
    }
    pub fn read_byte(&self, addr: usize) -> u8 {
        let mut current_max_size = 0;
        for driver in &self.drivers {
            current_max_size += driver.get_size();
            if (addr as usize) * 8 < current_max_size {
                // println!("reading: addr: {}, data: {}", addr, driver.read_byte(addr));
                return driver.read_byte(addr);
            }
        }
        println!("out of bounds!!! addr {}", addr);
        return 0;
    }
    pub fn write_byte(&mut self, addr: usize, data: u8) {
        let mut current_max_size = 0;
        for driver in &mut self.drivers {
            current_max_size += driver.get_size();
            // println!("addr: {addr}");
            if (addr as usize) * 8 < current_max_size {
                driver.write_byte(addr, data);
                return;
            }
        }
        println!("out of bounds!!! addr {}", addr);
        panic!();
    }
    pub fn add_driver(&mut self, driver: Box<dyn DataDriver>) {
        self.drivers.push(driver);
    }
}
#[derive(Debug)]
pub struct BitMap {
    data: Vec<bool>,
    size: usize,
}
impl BitMap {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![false; size],
            size,
        }
    }
    pub fn set_bit(&mut self, addr: usize, data: bool) {
        self.data[addr] = data;
    }
    pub fn get_bit(&self, addr: usize) -> bool {
        return self.data[addr];
    }
    pub fn get_size(&self) -> usize {
        return self.size;
    }
    pub fn write_byte(&mut self, addr: usize, data: u8) {
        for i in 0..8 {
            self.set_bit((addr * 8) + i, data.get_bit(7 - i));
        }
    }
    pub fn read_byte(&self, addr: usize) -> u8 {
        let mut res: u8 = 0;
        for i in 0..8 {
            res.set_bit(7 - i, self.get_bit((addr * 8) + i));
        }
        return res;
    }
}

pub trait DataDriver {
    fn get_size(&self) -> usize;
    fn write_byte(&mut self, addr: usize, data: u8);
    fn read_byte(&self, addr: usize) -> u8;
}
impl Debug for dyn DataDriver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut data: Vec<u8> = vec![0; self.get_size() / 8];
        let mut data_string = "[".to_string();

        for i in 0..self.get_size() / 8 {
            data[i] = self.read_byte(i);
            data_string += format!("{:#04X}, ", data[i]).as_str();
        }
        data_string.pop(); // remover the ", " from the last part
        data_string.pop();
        data_string += "]";
        f.debug_struct("driver")
            .field("data", &data_string)
            .finish()
    }
}

pub struct RAM {
    bit_map: BitMap,
}
impl RAM {
    pub fn new(size: usize) -> Self {
        Self {
            bit_map: BitMap::new(size),
        }
    }
}
impl DataDriver for RAM {
    fn get_size(&self) -> usize {
        self.bit_map.get_size()
    }

    fn write_byte(&mut self, addr: usize, data: u8) {
        self.bit_map.write_byte(addr, data)
    }

    fn read_byte(&self, addr: usize) -> u8 {
        self.bit_map.read_byte(addr)
    }
}
pub trait ManipulateBits {
    fn set_bit(&mut self, bit: usize, data: bool);
    fn get_bit(&self, bit: usize) -> bool;
}
impl ManipulateBits for u8 {
    fn set_bit(&mut self, bit: usize, data: bool) {
        *self &= !(1 << bit);
        *self |= (data as u8) << bit;
    }

    fn get_bit(&self, bit: usize) -> bool {
        return ((*self >> bit) % 2) != 0;
    }
}
