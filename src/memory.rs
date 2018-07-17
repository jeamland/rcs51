pub trait Memory {
    fn get_byte(&self, address: usize) -> u8;
    fn get_bytes(&self, address: usize, count: usize) -> &[u8];
}

pub struct SimpleMemory {
    bytes: Vec<u8>,
}

impl SimpleMemory {
    pub fn new(size: usize) -> SimpleMemory {
        SimpleMemory {
            bytes: vec![0; size],
        }
    }
}

impl Memory for SimpleMemory {
    fn get_byte(&self, address: usize) -> u8 {
        self.bytes[address]
    }

    fn get_bytes(&self, address: usize, count: usize) -> &[u8] {
        &self.bytes[address..(address + count)]
    }
}
