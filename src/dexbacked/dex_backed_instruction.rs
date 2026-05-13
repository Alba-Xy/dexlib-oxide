use crate::base::opcode::{InstructionFormat, Opcode};
use crate::iface::instruction::*;

#[derive(Debug)]
pub struct DexBackedInstruction {
    opcode: Opcode,
    raw_insns: Vec<u16>,
    offset: usize,
}

impl DexBackedInstruction {
    pub fn new(opcode: Opcode, raw_insns: Vec<u16>, offset: usize) -> Self {
        Self { opcode, raw_insns, offset }
    }

    fn insn(&self, index: usize) -> u16 {
        self.raw_insns.get(self.offset + index).copied().unwrap_or(0)
    }

    pub fn decode_instruction(insns: &[u16], offset: usize) -> Result<(Box<dyn Instruction>, usize), String> {
        if offset >= insns.len() {
            return Err(format!("offset {} out of bounds for insns of length {}", offset, insns.len()));
        }
        let full_word = insns[offset];
        let raw_opcode = (full_word & 0xFF) as u8;

        let code_units = match full_word {
            0x0100 => {
                if offset + 1 >= insns.len() {
                    return Err("truncated packed-switch payload".to_string());
                }
                let switch_count = insns[offset + 1] as u32;
                4 + switch_count * 2
            }
            0x0200 => {
                if offset + 1 >= insns.len() {
                    return Err("truncated sparse-switch payload".to_string());
                }
                let switch_count = insns[offset + 1] as u32;
                2 + switch_count * 4
            }
            0x0300 => {
                if offset + 3 >= insns.len() {
                    return Err("truncated array-payload".to_string());
                }
                let element_width = insns[offset + 1];
                let array_length = (insns[offset + 2] as u32) | ((insns[offset + 3] as u32) << 16);
                let data_size = array_length
                    .checked_mul(element_width as u32)
                    .map(|v| (v + 1) / 2)
                    .ok_or_else(|| format!(
                        "array-payload overflow: length={}, width={}",
                        array_length, element_width
                    ))?;
                4 + data_size
            }
            _ => {
                let opcode = Opcode::from_u8(raw_opcode)
                    .ok_or_else(|| format!("Unknown opcode: 0x{:02x}", raw_opcode))?;
                opcode.code_units()
            }
        };

        let opcode = if full_word == 0x0100
            || full_word == 0x0200
            || full_word == 0x0300
        {
            Opcode::Nop
        } else {
            Opcode::from_u8(raw_opcode)
                .ok_or_else(|| format!("Unknown opcode: 0x{:02x}", raw_opcode))?
        };

        let end = offset + code_units as usize;
        if end > insns.len() {
            return Err(format!(
                "instruction at offset {} requires {} code units but only {} remain",
                offset, code_units, insns.len() - offset
            ));
        }

        let raw = insns[offset..end].to_vec();
        let instr = DexBackedInstruction {
            opcode,
            raw_insns: raw,
            offset: 0,
        };

        Ok((Box::new(instr), code_units as usize))
    }
}

impl Instruction for DexBackedInstruction {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(DexBackedInstruction::new(self.opcode, self.raw_insns.clone(), self.offset)) }
    fn format(&self) -> InstructionFormat {
        match self.insn(0) {
            0x0100 => InstructionFormat::PackedSwitchPayload,
            0x0200 => InstructionFormat::SparseSwitchPayload,
            0x0300 => InstructionFormat::ArrayPayload,
            _ => self.opcode.format(),
        }
    }
    fn code_units(&self) -> u32 {
        match self.insn(0) {
            0x0100 => {
                let switch_count = self.insn(1) as u32;
                4 + switch_count * 2
            }
            0x0200 => {
                let switch_count = self.insn(1) as u32;
                2 + switch_count * 4
            }
            0x0300 => {
                let element_width = self.insn(1);
                let array_length = (self.insn(2) as u32) | ((self.insn(3) as u32) << 16);
                let data_size = array_length
                    .checked_mul(element_width as u32)
                    .map(|v| (v + 1) / 2)
                    .unwrap_or(u32::MAX);
                4 + data_size
            }
            _ => self.opcode.code_units(),
        }
    }
}

impl DexBackedInstruction {
    pub fn register_a_byte(&self) -> u8 {
        ((self.insn(0) >> 8) & 0xFF) as u8
    }

    pub fn register_b_byte(&self) -> u8 {
        (self.insn(1) & 0xFF) as u8
    }

    pub fn register_a_short(&self) -> u16 {
        self.insn(1)
    }

    pub fn register_b_short(&self) -> u16 {
        self.insn(2)
    }

    pub fn register_c_short(&self) -> u16 {
        self.insn(3)
    }

    pub fn literal_nibble(&self) -> i8 {
        (((self.insn(0) >> 8) & 0xF) as i8) << 4 >> 4
    }

    pub fn literal_16(&self) -> i16 {
        self.insn(1) as i16
    }

    pub fn literal_32(&self) -> i32 {
        (self.insn(1) as i32) | ((self.insn(2) as i32) << 16)
    }

    pub fn literal_64(&self) -> i64 {
        let lo = (self.insn(1) as u32) | ((self.insn(2) as u32) << 16);
        let hi = (self.insn(3) as u32) | ((self.insn(4) as u32) << 16);
        (((hi as u64) << 32) | lo as u64) as i64
    }

    pub fn code_offset_8(&self) -> i16 {
        (((self.insn(0) >> 8) & 0xFF) as i8) as i16
    }

    pub fn code_offset_16(&self) -> i16 {
        self.insn(1) as i16
    }

    pub fn code_offset_32(&self) -> i32 {
        (self.insn(1) as i32) | ((self.insn(2) as i32) << 16)
    }

    pub fn reference_index_16(&self) -> u16 {
        self.insn(1)
    }

    pub fn reference_index_32(&self) -> u32 {
        (self.insn(1) as u32) | ((self.insn(2) as u32) << 16)
    }

    pub fn invoke_register_count(&self) -> u8 {
        ((self.insn(1) >> 8) & 0xF) as u8
    }

    pub fn invoke_register_c(&self) -> u16 {
        self.insn(1) & 0xF
    }

    pub fn invoke_register_d(&self) -> u16 {
        (self.insn(1) >> 4) & 0xF
    }

    pub fn invoke_register_e(&self) -> u16 {
        self.insn(2) & 0xF
    }

    pub fn invoke_register_f(&self) -> u16 {
        (self.insn(2) >> 4) & 0xF
    }

    pub fn invoke_register_g(&self) -> u16 {
        (self.insn(0) >> 8) & 0xF
    }

    pub fn invoke_range_start(&self) -> u16 {
        self.insn(2)
    }

    pub fn invoke_range_count(&self) -> u8 {
        ((self.insn(1) >> 8) & 0xF) as u8
    }

    pub fn literal_byte(&self) -> i8 {
        (self.insn(1) & 0xFF) as i8
    }

    pub fn register_b_nibble(&self) -> u8 {
        ((self.insn(0) >> 12) & 0xF) as u8
    }

    pub fn register_c_nibble(&self) -> u8 {
        ((self.insn(0) >> 8) & 0xF) as u8
    }

    pub fn register_c_byte(&self) -> u8 {
        ((self.insn(1) >> 8) & 0xFF) as u8
    }

    pub fn register_a_22x(&self) -> u8 {
        ((self.insn(0) >> 8) & 0xFF) as u8
    }

    pub fn register_b_22x(&self) -> u16 {
        self.insn(1)
    }

    pub fn register_a_32x(&self) -> u16 {
        self.insn(1)
    }

    pub fn register_b_32x(&self) -> u16 {
        self.insn(2)
    }

    pub fn register_c_32x(&self) -> u16 {
        self.insn(2)
    }

    pub fn raw_code_units(&self) -> &[u16] {
        &self.raw_insns[self.offset..]
    }
}

pub fn decode_all_instructions(insns: &[u16]) -> Vec<Box<dyn Instruction>> {
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < insns.len() {
        match DexBackedInstruction::decode_instruction(insns, offset) {
            Ok((instr, code_units)) => {
                offset += code_units;
                result.push(instr);
            }
            Err(e) => {
                let raw_op = (insns[offset] & 0xFF) as u8;
                eprintln!("  WARNING: {} at offset {} (raw=0x{:04x}), skipping 1cu", e, offset, insns[offset]);
                offset += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_nop() {
        let insns = [0x0000u16];
        let (instr, size) = DexBackedInstruction::decode_instruction(&insns, 0).unwrap();
        assert_eq!(instr.opcode(), Opcode::Nop);
        assert_eq!(size, 1);
    }

    #[test]
    fn test_decode_return_void() {
        let insns = [0x000Eu16];
        let (instr, _) = DexBackedInstruction::decode_instruction(&insns, 0).unwrap();
        assert_eq!(instr.opcode(), Opcode::ReturnVoid);
    }

    #[test]
    fn test_decode_const_string() {
        let insns: Vec<u16> = vec![0x001A, 0x0005];
        let (instr, size) = DexBackedInstruction::decode_instruction(&insns, 0).unwrap();
        assert_eq!(instr.opcode(), Opcode::ConstString);
        assert_eq!(size, 2);
    }

    #[test]
    fn test_decode_invoke_static() {
        let insns: Vec<u16> = vec![0x0071, 0x0310, 0x2001];
        let (instr, size) = DexBackedInstruction::decode_instruction(&insns, 0).unwrap();
        assert_eq!(instr.opcode(), Opcode::InvokeStatic);
        assert_eq!(size, 3);
    }

    #[test]
    fn test_decode_const_wide() {
        let insns: Vec<u16> = vec![0x0018, 0x0000, 0x0000, 0x0000, 0x0000];
        let (instr, size) = DexBackedInstruction::decode_instruction(&insns, 0).unwrap();
        assert_eq!(instr.opcode(), Opcode::ConstWide);
        assert_eq!(size, 5);
    }

    #[test]
    fn test_decode_unknown_opcode() {
        let insns = [0x003Eu16];
        let result = DexBackedInstruction::decode_instruction(&insns, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_all_instructions() {
        let insns: Vec<u16> = vec![0x0000, 0x000E];
        let result = decode_all_instructions(&insns);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].opcode(), Opcode::Nop);
        assert_eq!(result[1].opcode(), Opcode::ReturnVoid);
    }
}
