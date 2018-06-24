pub mod memory;

#[cfg(test)]
mod tests {
    use memory;
    use memory::Memory;

    #[test]
    fn simple_get_byte() {
        let mem = memory::SimpleMemory::new(16);
        for address in 0..15 {
            assert_eq!(mem.get_byte(address), 0);
        }
    }

    #[test]
    fn simple_get_bytes() {
        let mem = memory::SimpleMemory::new(16);
        for address in 0..15 {
            let count = 16 - address;
            assert_eq!(mem.get_bytes(address, count),
                vec![0; count].as_slice());
        }
    }
}
