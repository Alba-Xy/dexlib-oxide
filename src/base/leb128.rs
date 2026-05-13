pub fn decode_uleb128(data: &[u8], offset: usize) -> (u32, usize) {
    let mut result: u32 = 0;
    let mut shift: u32 = 0;
    let mut pos = offset;
    loop {
        if pos >= data.len() {
            break;
        }
        let byte = data[pos];
        result |= ((byte & 0x7F) as u32) << shift;
        pos += 1;
        if byte & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 35 {
            break;
        }
    }
    (result, pos)
}

pub fn encode_uleb128(value: u32) -> Vec<u8> {
    let mut result = Vec::new();
    let mut remaining = value;
    loop {
        let mut byte = (remaining & 0x7F) as u8;
        remaining >>= 7;
        if remaining != 0 {
            byte |= 0x80;
        }
        result.push(byte);
        if remaining == 0 {
            break;
        }
    }
    result
}

pub fn decode_sleb128(data: &[u8], offset: usize) -> (i32, usize) {
    let mut result: i32 = 0;
    let mut shift: u32 = 0;
    let mut pos = offset;
    let mut byte: u8 = 0;
    loop {
        if pos >= data.len() {
            break;
        }
        byte = data[pos];
        result |= ((byte & 0x7F) as i32) << shift;
        shift += 7;
        pos += 1;
        if byte & 0x80 == 0 {
            break;
        }
    }
    if shift < 32 && (byte & 0x40) != 0 {
        result |= !0i32 << shift;
    }
    (result, pos)
}

pub fn encode_sleb128(value: i32) -> Vec<u8> {
    let mut result = Vec::new();
    let mut remaining = value;
    let mut more = true;
    while more {
        let mut byte = (remaining & 0x7F) as u8;
        remaining >>= 7;
        if (remaining == 0 && (byte & 0x40) == 0) || (remaining == -1 && (byte & 0x40) != 0) {
            more = false;
        } else {
            byte |= 0x80;
        }
        result.push(byte);
    }
    result
}

pub fn decode_uleb128p1(data: &[u8], offset: usize) -> (u32, usize) {
    let (value, new_offset) = decode_uleb128(data, offset);
    (value.wrapping_sub(1), new_offset)
}

pub fn encode_uleb128p1(value: i32) -> Vec<u8> {
    encode_uleb128((value as u32).wrapping_add(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uleb128_roundtrip() {
        let values = [0u32, 1, 127, 128, 255, 16383, 16384, 2097151, 268435455];
        for &v in &values {
            let encoded = encode_uleb128(v);
            let (decoded, _) = decode_uleb128(&encoded, 0);
            assert_eq!(decoded, v, "uleb128 roundtrip failed for {}", v);
        }
    }

    #[test]
    fn test_sleb128_roundtrip() {
        let values = [0i32, 1, -1, 63, -64, 64, -65, 8191, -8192, 268435455, -268435456];
        for &v in &values {
            let encoded = encode_sleb128(v);
            let (decoded, _) = decode_sleb128(&encoded, 0);
            assert_eq!(decoded, v, "sleb128 roundtrip failed for {}", v);
        }
    }

    #[test]
    fn test_uleb128p1_roundtrip() {
        let values = [0i32, 1, -1, 127, 16383];
        for &v in &values {
            let encoded = encode_uleb128p1(v);
            let (decoded, _) = decode_uleb128p1(&encoded, 0);
            assert_eq!(decoded, v as u32, "uleb128p1 roundtrip failed for {}", v);
        }
    }

    #[test]
    fn test_uleb128_known_encoding() {
        assert_eq!(encode_uleb128(0), vec![0x00]);
        assert_eq!(encode_uleb128(1), vec![0x01]);
        assert_eq!(encode_uleb128(127), vec![0x7F]);
        assert_eq!(encode_uleb128(128), vec![0x80, 0x01]);
        assert_eq!(encode_uleb128(624485), vec![0xE5, 0x8E, 0x26]);
    }

    #[test]
    fn test_sleb128_known_encoding() {
        assert_eq!(encode_sleb128(0), vec![0x00]);
        assert_eq!(encode_sleb128(1), vec![0x01]);
        assert_eq!(encode_sleb128(-1), vec![0x7F]);
        assert_eq!(encode_sleb128(-128), vec![0x80, 0x7F]);
        assert_eq!(encode_sleb128(624485), vec![0xE5, 0x8E, 0x26]);
    }

    #[test]
    fn test_uleb128p1_negative_one() {
        assert_eq!(encode_uleb128p1(-1), vec![0x00]);
        let (decoded, _) = decode_uleb128p1(&[0x00], 0);
        assert_eq!(decoded, -1i32 as u32);
    }

    #[test]
    fn test_decode_with_offset() {
        let buf = [0xFF, 0x00, 0xE5, 0x8E, 0x26];
        let (val, end) = decode_uleb128(&buf, 2);
        assert_eq!(val, 624485);
        assert_eq!(end, 5);
    }
}
