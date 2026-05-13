use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Mutf8Error {
    UnexpectedEnd,
    InvalidByte(u8),
    InvalidSurrogate(u16),
}

impl fmt::Display for Mutf8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mutf8Error::UnexpectedEnd => write!(f, "unexpected end of MUTF-8 data"),
            Mutf8Error::InvalidByte(b) => write!(f, "invalid MUTF-8 byte: 0x{:02X}", b),
            Mutf8Error::InvalidSurrogate(s) => write!(f, "invalid surrogate: 0x{:04X}", s),
        }
    }

}

impl std::error::Error for Mutf8Error {}

pub fn decode_mutf8(data: &[u8]) -> Result<String, Mutf8Error> {
    let mut chars = Vec::new();
    let mut pos = 0;
    while pos < data.len() {
        if data[pos] == 0x00 {
            break;
        }
        let (ch, new_pos) = decode_mutf8_char(data, pos)?;
        chars.push(ch);
        pos = new_pos;
    }
    Ok(chars.into_iter().collect())
}

pub fn encode_mutf8(s: &str) -> Vec<u8> {
    let mut result = Vec::new();
    for ch in s.chars() {
        encode_mutf8_char(ch, &mut result);
    }
    result.push(0x00);
    result
}

pub fn decode_mutf8_with_length(data: &[u8], utf16_len: usize) -> Result<String, Mutf8Error> {
    let mut chars = Vec::new();
    let mut pos = 0;
    let mut utf16_count = 0;
    while utf16_count < utf16_len && pos < data.len() {
        if data[pos] == 0x00 {
            break;
        }
        let start_pos = pos;
        let (ch, new_pos) = decode_mutf8_char(data, pos)?;
        utf16_count += char_utf16_len(ch);
        if utf16_count > utf16_len {
            break;
        }
        chars.push(ch);
        pos = new_pos;
        let _ = start_pos;
    }
    Ok(chars.into_iter().collect())
}

fn char_utf16_len(ch: char) -> usize {
    if (ch as u32) < 0x10000 { 1 } else { 2 }
}

fn decode_mutf8_char(data: &[u8], pos: usize) -> Result<(char, usize), Mutf8Error> {
    if pos >= data.len() {
        return Err(Mutf8Error::UnexpectedEnd);
    }
    let b1 = data[pos];

    if b1 == 0x00 {
        return Err(Mutf8Error::InvalidByte(0x00));
    }

    if (b1 & 0x80) == 0x00 {
        return Ok((b1 as char, pos + 1));
    }

    if (b1 & 0xE0) == 0xC0 {
        if pos + 1 >= data.len() {
            return Err(Mutf8Error::UnexpectedEnd);
        }
        let b2 = data[pos + 1];
        if (b2 & 0xC0) != 0x80 {
            return Err(Mutf8Error::InvalidByte(b2));
        }
        let code_point = (((b1 & 0x1F) as u32) << 6) | ((b2 & 0x3F) as u32);
        if code_point == 0 {
            return Ok(('\0', pos + 2));
        }
        return Ok((char::from_u32(code_point).unwrap_or('\u{FFFD}'), pos + 2));
    }

    if (b1 & 0xF0) == 0xE0 {
        if pos + 2 >= data.len() {
            return Err(Mutf8Error::UnexpectedEnd);
        }
        let b2 = data[pos + 1];
        let b3 = data[pos + 2];
        if (b2 & 0xC0) != 0x80 || (b3 & 0xC0) != 0x80 {
            return Err(Mutf8Error::InvalidByte(b3));
        }
        let code_point = (((b1 & 0x0F) as u32) << 12)
            | (((b2 & 0x3F) as u32) << 6)
            | ((b3 & 0x3F) as u32);

        if (0xD800..=0xDFFF).contains(&code_point) {
            let high = code_point - 0xD800;
            if pos + 5 < data.len()
                && (data[pos + 3] & 0xF0) == 0xE0
                && (data[pos + 4] & 0xC0) == 0x80
                && (data[pos + 5] & 0xC0) == 0x80
            {
                let low_byte1 = data[pos + 3];
                let low_byte2 = data[pos + 4];
                let low_byte3 = data[pos + 5];
                let low_surrogate = (((low_byte1 & 0x0F) as u32) << 12)
                    | (((low_byte2 & 0x3F) as u32) << 6)
                    | ((low_byte3 & 0x3F) as u32);
                if (0xDC00..=0xDFFF).contains(&low_surrogate) {
                    let low = low_surrogate - 0xDC00;
                    let full_char = 0x10000 + (high << 10) + low;
                    if let Some(ch) = char::from_u32(full_char) {
                        return Ok((ch, pos + 6));
                    }
                }
            }
            return Err(Mutf8Error::InvalidSurrogate(code_point as u16));
        }

        if let Some(ch) = char::from_u32(code_point) {
            return Ok((ch, pos + 3));
        }
        return Err(Mutf8Error::InvalidByte(b1));
    }

    Err(Mutf8Error::InvalidByte(b1))
}

fn encode_mutf8_char(ch: char, out: &mut Vec<u8>) {
    let code = ch as u32;

    if code == 0 {
        out.push(0xC0);
        out.push(0x80);
        return;
    }

    if code < 0x80 {
        out.push(code as u8);
        return;
    }

    if code < 0x800 {
        out.push(0xC0 | ((code >> 6) as u8));
        out.push(0x80 | ((code & 0x3F) as u8));
        return;
    }

    if code >= 0x10000 {
        let v = code - 0x10000;
        let high = 0xD800 + ((v >> 10) as u16);
        let low = 0xDC00 + ((v & 0x3FF) as u16);
        encode_mutf8_surrogate(high, out);
        encode_mutf8_surrogate(low, out);
        return;
    }

    out.push(0xE0 | ((code >> 12) as u8));
    out.push(0x80 | (((code >> 6) & 0x3F) as u8));
    out.push(0x80 | ((code & 0x3F) as u8));
}

fn encode_mutf8_surrogate(surrogate: u16, out: &mut Vec<u8>) {
    out.push(0xE0 | ((surrogate >> 12) as u8));
    out.push(0x80 | (((surrogate >> 6) & 0x3F) as u8));
    out.push(0x80 | ((surrogate & 0x3F) as u8));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_roundtrip() {
        let s = "Hello, World!";
        let encoded = encode_mutf8(s);
        let decoded = decode_mutf8(&encoded).unwrap();
        assert_eq!(decoded, s);
    }

    #[test]
    fn test_null_char() {
        let s = "\0";
        let encoded = encode_mutf8(s);
        assert_eq!(encoded, vec![0xC0, 0x80, 0x00]);
        let decoded = decode_mutf8(&encoded).unwrap();
        assert_eq!(decoded, "\0");
    }

    #[test]
    fn test_chinese_roundtrip() {
        let s = "你好世界";
        let encoded = encode_mutf8(s);
        let decoded = decode_mutf8(&encoded).unwrap();
        assert_eq!(decoded, s);
    }

    #[test]
    fn test_emoji_roundtrip() {
        let s = "😀";
        let encoded = encode_mutf8(s);
        let decoded = decode_mutf8(&encoded).unwrap();
        assert_eq!(decoded, s);
    }

    #[test]
    fn test_decode_with_length() {
        let s = "Hello";
        let encoded = encode_mutf8(s);
        let data = &encoded[..encoded.len() - 1];
        let decoded = decode_mutf8_with_length(data, 5).unwrap();
        assert_eq!(decoded, "Hello");
    }

    #[test]
    fn test_mixed_content() {
        let s = "abc你好123😀xyz";
        let encoded = encode_mutf8(s);
        let decoded = decode_mutf8(&encoded).unwrap();
        assert_eq!(decoded, s);
    }

    #[test]
    fn test_null_terminated() {
        let data = [0x48, 0x69, 0x00, 0xFF];
        let decoded = decode_mutf8(&data).unwrap();
        assert_eq!(decoded, "Hi");
    }
}
