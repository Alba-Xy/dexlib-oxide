use crate::base::leb128;
use crate::base::mutf8;
use crate::writer::memory_data_store::DataStore;

pub struct DexWriter<'a> {
    data_store: &'a mut dyn DataStore,
    offset: u32,
}

impl<'a> DexWriter<'a> {
    pub fn new(data_store: &'a mut dyn DataStore) -> Self {
        Self { data_store, offset: 0 }
    }

    pub fn write_u8(&mut self, value: u8) {
        self.data_store.write_at(self.offset, &[value]);
        self.offset += 1;
    }

    pub fn write_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.data_store.write_at(self.offset, &bytes);
        self.offset += 2;
    }

    pub fn write_u32(&mut self, value: u32) {
        let bytes = value.to_le_bytes();
        self.data_store.write_at(self.offset, &bytes);
        self.offset += 4;
    }

    pub fn write_i8(&mut self, value: i8) {
        self.write_u8(value as u8);
    }

    pub fn write_i16(&mut self, value: i16) {
        self.write_u16(value as u16);
    }

    pub fn write_i32(&mut self, value: i32) {
        self.write_u32(value as u32);
    }

    pub fn write_i64(&mut self, value: i64) {
        let bytes = value.to_le_bytes();
        self.data_store.write_at(self.offset, &bytes);
        self.offset += 8;
    }

    pub fn write_uleb128(&mut self, value: u32) {
        let encoded = leb128::encode_uleb128(value);
        self.data_store.write_at(self.offset, &encoded);
        self.offset += encoded.len() as u32;
    }

    pub fn write_sleb128(&mut self, value: i32) {
        let encoded = leb128::encode_sleb128(value);
        self.data_store.write_at(self.offset, &encoded);
        self.offset += encoded.len() as u32;
    }

    pub fn write_bytes(&mut self, data: &[u8]) {
        self.data_store.write_at(self.offset, data);
        self.offset += data.len() as u32;
    }

    pub fn write_u16_at(&mut self, offset: u32, value: u16) {
        let bytes = value.to_le_bytes();
        self.data_store.write_at(offset, &bytes);
    }

    pub fn write_u32_at(&mut self, offset: u32, value: u32) {
        let bytes = value.to_le_bytes();
        self.data_store.write_at(offset, &bytes);
    }

    pub fn align_to(&mut self, alignment: u32) {
        let misalignment = self.offset % alignment;
        if misalignment != 0 {
            let padding = alignment - misalignment;
            for _ in 0..padding {
                self.write_u8(0);
            }
        }
    }

    pub fn current_offset(&self) -> u32 {
        self.offset
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    pub fn write_string_data(&mut self, string: &str) -> u32 {
        let start_offset = self.offset;
        let encoded = mutf8::encode_mutf8(string);
        let utf16_len = string.chars().map(|c| {
            if (c as u32) < 0x10000 { 1u32 } else { 2u32 }
        }).sum::<u32>();
        self.write_uleb128(utf16_len);
        self.write_bytes(&encoded[..encoded.len() - 1]);
        self.write_u8(0);
        start_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::writer::memory_data_store::MemoryDataStore;

    #[test]
    fn test_write_primitives() {
        let mut store = MemoryDataStore::new();
        let mut writer = DexWriter::new(&mut store);
        writer.write_u8(0x01);
        writer.write_u16(0x0203);
        writer.write_u32(0x04050607);
        assert_eq!(writer.current_offset(), 7);
        let buf = store.get_buffer();
        assert_eq!(buf[0], 0x01);
        assert_eq!(buf[1], 0x03);
        assert_eq!(buf[2], 0x02);
    }

    #[test]
    fn test_write_uleb128() {
        let mut store = MemoryDataStore::new();
        let mut writer = DexWriter::new(&mut store);
        writer.write_uleb128(0);
        writer.write_uleb128(127);
        writer.write_uleb128(128);
        assert_eq!(store.get_buffer()[0], 0x00);
        assert_eq!(store.get_buffer()[1], 0x7F);
        assert_eq!(store.get_buffer()[2], 0x80);
        assert_eq!(store.get_buffer()[3], 0x01);
    }

    #[test]
    fn test_align_to() {
        let mut store = MemoryDataStore::new();
        let mut writer = DexWriter::new(&mut store);
        writer.write_u8(0x01);
        assert_eq!(writer.current_offset(), 1);
        writer.align_to(4);
        assert_eq!(writer.current_offset(), 4);
    }

    #[test]
    fn test_write_at_offset() {
        let mut store = MemoryDataStore::new();
        let mut writer = DexWriter::new(&mut store);
        writer.write_u32(0);
        writer.write_u32(0);
        writer.write_u32_at(4, 0xDEADBEEF);
        assert_eq!(store.get_buffer()[4..8], [0xEF, 0xBE, 0xAD, 0xDE]);
    }

    #[test]
    fn test_write_string_data() {
        let mut store = MemoryDataStore::new();
        let mut writer = DexWriter::new(&mut store);
        let offset = writer.write_string_data("hello");
        assert_eq!(offset, 0);
        assert!(store.get_buffer().len() > 0);
    }
}
