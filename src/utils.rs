pub trait SetGetBytes {
    fn set_byte(&mut self, byte: usize, data: u8);
    fn get_byte(&self, byte: usize) -> u8;
}
impl SetGetBytes for usize {
    fn set_byte(&mut self, byte: usize, data: u8) {
        let mask: usize = 0xff << byte * 8;
        let temp: usize = *self & !mask;
        *self = temp | ((data as usize) << byte * 8);
    }

    fn get_byte(&self, byte: usize) -> u8 {
        return ((*self >> 8 * byte) & 0xff) as u8;
    }
}
pub trait Enumerate {
    type Item;
    fn enumerate(&self) -> Vec<(usize, &Self::Item)>;
}
impl<T> Enumerate for Vec<T> {
    type Item = T;

    fn enumerate(&self) -> Vec<(usize, &Self::Item)> {
        let mut res = vec![];
        for i in 0..self.len() {
            res.push((i, &self[i]))
        }
        return res;
    }
}