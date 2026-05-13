use crate::base::leb128;
use crate::base::mutf8;

#[derive(Debug, Clone)]
pub struct DexHeader {
    pub magic: [u8; 8],
    pub checksum: u32,
    pub signature: [u8; 20],
    pub file_size: u32,
    pub header_size: u32,
    pub endian_tag: u32,
    pub link_size: u32,
    pub link_off: u32,
    pub map_off: u32,
    pub string_ids_size: u32,
    pub string_ids_off: u32,
    pub type_ids_size: u32,
    pub type_ids_off: u32,
    pub proto_ids_size: u32,
    pub proto_ids_off: u32,
    pub field_ids_size: u32,
    pub field_ids_off: u32,
    pub method_ids_size: u32,
    pub method_ids_off: u32,
    pub class_defs_size: u32,
    pub class_defs_off: u32,
    pub data_size: u32,
    pub data_off: u32,
}

impl DexHeader {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        if data.len() < 0x70 {
            return Err("DEX file too small for header".to_string());
        }
        if &data[0..4] != b"dex\n" {
            return Err("Invalid DEX magic".to_string());
        }

        let mut magic = [0u8; 8];
        magic.copy_from_slice(&data[0..8]);

        let mut signature = [0u8; 20];
        signature.copy_from_slice(&data[12..32]);

        Ok(Self {
            magic,
            checksum: read_u32_le(data, 8),
            signature,
            file_size: read_u32_le(data, 32),
            header_size: read_u32_le(data, 36),
            endian_tag: read_u32_le(data, 40),
            link_size: read_u32_le(data, 44),
            link_off: read_u32_le(data, 48),
            map_off: read_u32_le(data, 52),
            string_ids_size: read_u32_le(data, 56),
            string_ids_off: read_u32_le(data, 60),
            type_ids_size: read_u32_le(data, 64),
            type_ids_off: read_u32_le(data, 68),
            proto_ids_size: read_u32_le(data, 72),
            proto_ids_off: read_u32_le(data, 76),
            field_ids_size: read_u32_le(data, 80),
            field_ids_off: read_u32_le(data, 84),
            method_ids_size: read_u32_le(data, 88),
            method_ids_off: read_u32_le(data, 92),
            class_defs_size: read_u32_le(data, 96),
            class_defs_off: read_u32_le(data, 100),
            data_size: read_u32_le(data, 104),
            data_off: read_u32_le(data, 108),
        })
    }

    pub fn dex_version(&self) -> &str {
        std::str::from_utf8(&self.magic[4..7]).unwrap_or("???")
    }
}

#[derive(Debug, Clone)]
pub struct StringIdItem {
    pub string_data_off: u32,
}

pub fn parse_string_ids(data: &[u8], offset: u32, count: u32) -> Vec<StringIdItem> {
    let mut result = Vec::with_capacity(count as usize);
    for i in 0..count {
        let off = (offset + i * 4) as usize;
        result.push(StringIdItem {
            string_data_off: read_u32_le(data, off),
        });
    }
    result
}

pub fn parse_string_data(data: &[u8], string_data_off: u32) -> Result<String, String> {
    let off = string_data_off as usize;
    let (utf16_len, pos) = leb128::decode_uleb128(data, off);
    let s = mutf8::decode_mutf8_with_length(&data[pos..], utf16_len as usize)
        .map_err(|e| format!("MUTF-8 decode error: {}", e))?;
    Ok(s)
}

#[derive(Debug, Clone)]
pub struct TypeIdItem {
    pub descriptor_idx: u32,
}

pub fn parse_type_ids(data: &[u8], offset: u32, count: u32) -> Vec<TypeIdItem> {
    let mut result = Vec::with_capacity(count as usize);
    for i in 0..count {
        let off = (offset + i * 4) as usize;
        result.push(TypeIdItem {
            descriptor_idx: read_u32_le(data, off),
        });
    }
    result
}

#[derive(Debug, Clone)]
pub struct ProtoIdItem {
    pub shorty_idx: u32,
    pub return_type_idx: u32,
    pub parameters_off: u32,
}

pub fn parse_proto_ids(data: &[u8], offset: u32, count: u32) -> Vec<ProtoIdItem> {
    let mut result = Vec::with_capacity(count as usize);
    for i in 0..count {
        let off = (offset + i * 12) as usize;
        result.push(ProtoIdItem {
            shorty_idx: read_u32_le(data, off),
            return_type_idx: read_u32_le(data, off + 4),
            parameters_off: read_u32_le(data, off + 8),
        });
    }
    result
}

pub fn parse_proto_parameter_types(data: &[u8], parameters_off: u32) -> Vec<u32> {
    if parameters_off == 0 {
        return Vec::new();
    }
    let off = parameters_off as usize;
    let size = read_u32_le(data, off) as usize;
    let mut result = Vec::with_capacity(size);
    for i in 0..size {
        result.push(read_u16_le(data, off + 4 + i * 2) as u32);
    }
    result
}

#[derive(Debug, Clone)]
pub struct FieldIdItem {
    pub class_idx: u16,
    pub type_idx: u16,
    pub name_idx: u32,
}

pub fn parse_field_ids(data: &[u8], offset: u32, count: u32) -> Vec<FieldIdItem> {
    let mut result = Vec::with_capacity(count as usize);
    for i in 0..count {
        let off = (offset + i * 8) as usize;
        result.push(FieldIdItem {
            class_idx: read_u16_le(data, off),
            type_idx: read_u16_le(data, off + 2),
            name_idx: read_u32_le(data, off + 4),
        });
    }
    result
}

#[derive(Debug, Clone)]
pub struct MethodIdItem {
    pub class_idx: u16,
    pub proto_idx: u16,
    pub name_idx: u32,
}

pub fn parse_method_ids(data: &[u8], offset: u32, count: u32) -> Vec<MethodIdItem> {
    let mut result = Vec::with_capacity(count as usize);
    for i in 0..count {
        let off = (offset + i * 8) as usize;
        result.push(MethodIdItem {
            class_idx: read_u16_le(data, off),
            proto_idx: read_u16_le(data, off + 2),
            name_idx: read_u32_le(data, off + 4),
        });
    }
    result
}

#[derive(Debug, Clone)]
pub struct ClassDefItem {
    pub class_idx: u32,
    pub access_flags: u32,
    pub superclass_idx: u32,
    pub interfaces_off: u32,
    pub source_file_idx: u32,
    pub annotations_off: u32,
    pub class_data_off: u32,
    pub static_values_off: u32,
}

pub fn parse_class_defs(data: &[u8], offset: u32, count: u32) -> Vec<ClassDefItem> {
    let mut result = Vec::with_capacity(count as usize);
    for i in 0..count {
        let off = (offset + i * 32) as usize;
        result.push(ClassDefItem {
            class_idx: read_u32_le(data, off),
            access_flags: read_u32_le(data, off + 4),
            superclass_idx: read_u32_le(data, off + 8),
            interfaces_off: read_u32_le(data, off + 12),
            source_file_idx: read_u32_le(data, off + 16),
            annotations_off: read_u32_le(data, off + 20),
            class_data_off: read_u32_le(data, off + 24),
            static_values_off: read_u32_le(data, off + 28),
        });
    }
    result
}

pub fn parse_interface_list(data: &[u8], interfaces_off: u32) -> Vec<u16> {
    if interfaces_off == 0 {
        return Vec::new();
    }
    let off = interfaces_off as usize;
    let size = read_u32_le(data, off) as usize;
    let mut result = Vec::with_capacity(size);
    for i in 0..size {
        result.push(read_u16_le(data, off + 4 + i * 2));
    }
    result
}

#[derive(Debug, Clone)]
pub struct ClassDataHeader {
    pub static_fields_size: u32,
    pub instance_fields_size: u32,
    pub direct_methods_size: u32,
    pub virtual_methods_size: u32,
}

#[derive(Debug, Clone)]
pub struct ClassDataField {
    pub field_idx_diff: u32,
    pub access_flags: u32,
}

#[derive(Debug, Clone)]
pub struct ClassDataMethod {
    pub method_idx_diff: u32,
    pub access_flags: u32,
    pub code_off: u32,
}

pub fn parse_class_data(data: &[u8], class_data_off: u32) -> Result<(ClassDataHeader, Vec<ClassDataField>, Vec<ClassDataField>, Vec<ClassDataMethod>, Vec<ClassDataMethod>), String> {
    let mut pos = class_data_off as usize;

    let static_fields_size = leb128::decode_uleb128(data, pos).0;
    pos = leb128::decode_uleb128(data, pos).1;
    let instance_fields_size = leb128::decode_uleb128(data, pos).0;
    pos = leb128::decode_uleb128(data, pos).1;
    let direct_methods_size = leb128::decode_uleb128(data, pos).0;
    pos = leb128::decode_uleb128(data, pos).1;
    let virtual_methods_size = leb128::decode_uleb128(data, pos).0;
    pos = leb128::decode_uleb128(data, pos).1;

    let header = ClassDataHeader {
        static_fields_size,
        instance_fields_size,
        direct_methods_size,
        virtual_methods_size,
    };

    let static_fields = parse_fields(data, &mut pos, static_fields_size as usize);
    let instance_fields = parse_fields(data, &mut pos, instance_fields_size as usize);
    let direct_methods = parse_methods(data, &mut pos, direct_methods_size as usize);
    let virtual_methods = parse_methods(data, &mut pos, virtual_methods_size as usize);

    Ok((header, static_fields, instance_fields, direct_methods, virtual_methods))
}

fn parse_fields(data: &[u8], pos: &mut usize, count: usize) -> Vec<ClassDataField> {
    let mut result = Vec::with_capacity(count);
    let mut field_idx = 0u32;
    for _ in 0..count {
        let (idx_diff, new_pos) = leb128::decode_uleb128(data, *pos);
        *pos = new_pos;
        let (access_flags, new_pos) = leb128::decode_uleb128(data, *pos);
        *pos = new_pos;
        field_idx += idx_diff;
        result.push(ClassDataField {
            field_idx_diff: field_idx,
            access_flags,
        });
    }
    result
}

fn parse_methods(data: &[u8], pos: &mut usize, count: usize) -> Vec<ClassDataMethod> {
    let mut result = Vec::with_capacity(count);
    let mut method_idx = 0u32;
    for _ in 0..count {
        let (idx_diff, new_pos) = leb128::decode_uleb128(data, *pos);
        *pos = new_pos;
        let (access_flags, new_pos) = leb128::decode_uleb128(data, *pos);
        *pos = new_pos;
        let (code_off, new_pos) = leb128::decode_uleb128(data, *pos);
        *pos = new_pos;
        method_idx += idx_diff;
        result.push(ClassDataMethod {
            method_idx_diff: method_idx,
            access_flags,
            code_off,
        });
    }
    result
}

#[derive(Debug, Clone)]
pub struct CodeItem {
    pub registers_size: u16,
    pub ins_size: u16,
    pub outs_size: u16,
    pub tries_size: u16,
    pub debug_info_off: u32,
    pub insns: Vec<u16>,
    pub try_blocks: Vec<TryBlockRaw>,
}

#[derive(Debug, Clone)]
pub struct TryBlockRaw {
    pub start_addr: u32,
    pub insn_count: u16,
    pub handler_off: u16,
}

pub fn parse_code_item(data: &[u8], code_off: u32) -> Result<CodeItem, String> {
    if code_off == 0 {
        return Err("code_off is 0".to_string());
    }
    let off = code_off as usize;

    if off + 16 > data.len() {
        return Err(format!("code_item header at {} exceeds data length {}", off, data.len()));
    }

    let registers_size = read_u16_le(data, off);
    let ins_size = read_u16_le(data, off + 2);
    let outs_size = read_u16_le(data, off + 4);
    let tries_size = read_u16_le(data, off + 6);
    let debug_info_off = read_u32_le(data, off + 8);

    let insns_size = read_u32_le(data, off + 12) as usize;
    let insns_start = off + 16;
    let insns_end = insns_start + insns_size * 2;
    if insns_end > data.len() {
        return Err(format!(
            "insns_size {} at offset {} exceeds data length {} (end would be {})",
            insns_size, off, data.len(), insns_end
        ));
    }
    let mut insns = Vec::with_capacity(insns_size);
    for i in 0..insns_size {
        insns.push(read_u16_le(data, insns_start + i * 2));
    }

    let mut try_blocks = Vec::new();
    if tries_size > 0 {
        let padding = if insns_size % 2 == 1 { 2 } else { 0 };
        let tries_start = insns_start + insns_size * 2 + padding;
        for i in 0..tries_size as usize {
            let try_off = tries_start + i * 8;
            try_blocks.push(TryBlockRaw {
                start_addr: read_u32_le(data, try_off) & 0x00FFFFFF,
                insn_count: read_u16_le(data, try_off + 2),
                handler_off: read_u16_le(data, try_off + 4),
            });
        }
    }

    Ok(CodeItem {
        registers_size,
        ins_size,
        outs_size,
        tries_size,
        debug_info_off,
        insns,
        try_blocks,
    })
}

#[inline]
pub fn read_u16_le(data: &[u8], offset: usize) -> u16 {
    u16::from_le_bytes([data[offset], data[offset + 1]])
}

#[inline]
pub fn read_u32_le(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

#[inline]
pub fn read_i16_le(data: &[u8], offset: usize) -> i16 {
    i16::from_le_bytes([data[offset], data[offset + 1]])
}

#[inline]
pub fn read_i32_le(data: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
}

#[inline]
pub fn read_i64_le(data: &[u8], offset: usize) -> i64 {
    i64::from_le_bytes([
        data[offset], data[offset + 1], data[offset + 2], data[offset + 3],
        data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7],
    ])
}
