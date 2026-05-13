pub trait DataStore {
    fn write_at(&mut self, offset: u32, data: &[u8]);
    fn size(&self) -> u32;
    fn get_buffer(&self) -> &[u8];
}

pub struct MemoryDataStore {
    buffer: Vec<u8>,
}

impl MemoryDataStore {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self { buffer: Vec::with_capacity(capacity) }
    }

    pub fn ensure_capacity(&mut self, needed: usize) {
        if needed > self.buffer.len() {
            self.buffer.resize(needed, 0);
        }
    }
}

impl DataStore for MemoryDataStore {
    fn write_at(&mut self, offset: u32, data: &[u8]) {
        let end = offset as usize + data.len();
        self.ensure_capacity(end);
        self.buffer[offset as usize..end].copy_from_slice(data);
    }

    fn size(&self) -> u32 {
        self.buffer.len() as u32
    }

    fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl Default for MemoryDataStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_read() {
        let mut store = MemoryDataStore::new();
        store.write_at(0, &[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(store.size(), 4);
        assert_eq!(&store.get_buffer()[0..4], &[0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    fn test_write_at_offset() {
        let mut store = MemoryDataStore::new();
        store.write_at(0, &[0x00; 10]);
        store.write_at(5, &[0xAA, 0xBB]);
        assert_eq!(store.get_buffer()[5], 0xAA);
        assert_eq!(store.get_buffer()[6], 0xBB);
    }

    #[test]
    fn test_auto_extend() {
        let mut store = MemoryDataStore::new();
        store.write_at(100, &[0xFF]);
        assert_eq!(store.size(), 101);
    }
}
