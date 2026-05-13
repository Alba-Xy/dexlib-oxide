use crate::base::opcode::{InstructionFormat, Opcode};
use crate::iface::instruction::*;
use crate::iface::reference::ReferenceHolder;

#[derive(Debug, Clone)]
pub struct ImmutableInstruction10x {
    pub opcode: Opcode,
}

impl Instruction for ImmutableInstruction10x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction11x {
    pub opcode: Opcode,
    pub register_a: u16,
}

impl Instruction for ImmutableInstruction11x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl OneRegisterInstruction for ImmutableInstruction11x {
    fn register_a(&self) -> u16 { self.register_a }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction12x {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
}

impl Instruction for ImmutableInstruction12x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl TwoRegisterInstruction for ImmutableInstruction12x {
    fn register_a(&self) -> u8 { self.register_a }
    fn register_b(&self) -> u8 { self.register_b }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction11n {
    pub opcode: Opcode,
    pub register_a: u8,
    pub literal: i8,
}

impl Instruction for ImmutableInstruction11n {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction10t {
    pub opcode: Opcode,
    pub code_offset: i16,
}

impl Instruction for ImmutableInstruction10t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction21c {
    pub opcode: Opcode,
    pub register_a: u8,
    pub reference: ReferenceHolder,
}

impl Instruction for ImmutableInstruction21c {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction21c for ImmutableInstruction21c {
    fn register_a(&self) -> u8 { self.register_a }
    fn reference(&self) -> &crate::iface::reference::ReferenceHolder {
        &self.reference
    }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction31c {
    pub opcode: Opcode,
    pub register_a: u8,
    pub reference: ReferenceHolder,
}

impl Instruction for ImmutableInstruction31c {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction31c for ImmutableInstruction31c {
    fn register_a(&self) -> u8 { self.register_a }
    fn reference(&self) -> &crate::iface::reference::ReferenceHolder {
        &self.reference
    }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction22c {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub reference: ReferenceHolder,
}

impl Instruction for ImmutableInstruction22c {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction22c for ImmutableInstruction22c {
    fn register_a(&self) -> u8 { self.register_a }
    fn register_b(&self) -> u8 { self.register_b }
    fn reference(&self) -> &crate::iface::reference::ReferenceHolder {
        &self.reference
    }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction35c {
    pub opcode: Opcode,
    pub register_count: u8,
    pub register_c: u16,
    pub register_d: u16,
    pub register_e: u16,
    pub register_f: u16,
    pub register_g: u16,
    pub reference: ReferenceHolder,
}

impl Instruction for ImmutableInstruction35c {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction35c for ImmutableInstruction35c {
    fn register_count(&self) -> u8 { self.register_count }
    fn register_c(&self) -> u16 { self.register_c }
    fn register_d(&self) -> u16 { self.register_d }
    fn register_e(&self) -> u16 { self.register_e }
    fn register_f(&self) -> u16 { self.register_f }
    fn register_g(&self) -> u16 { self.register_g }
    fn reference(&self) -> &crate::iface::reference::ReferenceHolder {
        &self.reference
    }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction3rc {
    pub opcode: Opcode,
    pub start_register: u16,
    pub register_count: u8,
    pub reference: ReferenceHolder,
}

impl Instruction for ImmutableInstruction3rc {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction3rc for ImmutableInstruction3rc {
    fn start_register(&self) -> u16 { self.start_register }
    fn register_count(&self) -> u8 { self.register_count }
    fn reference(&self) -> &crate::iface::reference::ReferenceHolder {
        &self.reference
    }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction21s {
    pub opcode: Opcode,
    pub register_a: u8,
    pub literal: i16,
}

impl Instruction for ImmutableInstruction21s {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction21s for ImmutableInstruction21s {
    fn register_a(&self) -> u8 { self.register_a }
    fn literal(&self) -> i16 { self.literal }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction31i {
    pub opcode: Opcode,
    pub register_a: u8,
    pub literal: i32,
}

impl Instruction for ImmutableInstruction31i {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction31i for ImmutableInstruction31i {
    fn register_a(&self) -> u8 { self.register_a }
    fn literal(&self) -> i32 { self.literal }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction51l {
    pub opcode: Opcode,
    pub register_a: u8,
    pub literal: i64,
}

impl Instruction for ImmutableInstruction51l {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction51l for ImmutableInstruction51l {
    fn register_a(&self) -> u8 { self.register_a }
    fn literal(&self) -> i64 { self.literal }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction21t {
    pub opcode: Opcode,
    pub register_a: u8,
    pub code_offset: i16,
}

impl Instruction for ImmutableInstruction21t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction21t for ImmutableInstruction21t {
    fn register_a(&self) -> u8 { self.register_a }
    fn code_offset(&self) -> i16 { self.code_offset }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction22t {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub code_offset: i16,
}

impl Instruction for ImmutableInstruction22t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction22t for ImmutableInstruction22t {
    fn register_a(&self) -> u8 { self.register_a }
    fn register_b(&self) -> u8 { self.register_b }
    fn code_offset(&self) -> i16 { self.code_offset }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction22b {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub literal: i8,
}

impl Instruction for ImmutableInstruction22b {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction22b for ImmutableInstruction22b {
    fn register_a(&self) -> u8 { self.register_a }
    fn register_b(&self) -> u8 { self.register_b }
    fn literal(&self) -> i8 { self.literal }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction22s {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub literal: i16,
}

impl Instruction for ImmutableInstruction22s {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction22s for ImmutableInstruction22s {
    fn register_a(&self) -> u8 { self.register_a }
    fn register_b(&self) -> u8 { self.register_b }
    fn literal(&self) -> i16 { self.literal }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction23x {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub register_c: u8,
}

impl Instruction for ImmutableInstruction23x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl ThreeRegisterInstruction for ImmutableInstruction23x {
    fn register_a(&self) -> u8 { self.register_a }
    fn register_b(&self) -> u8 { self.register_b }
    fn register_c(&self) -> u8 { self.register_c }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction20t {
    pub opcode: Opcode,
    pub code_offset: i32,
}

impl Instruction for ImmutableInstruction20t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction20t for ImmutableInstruction20t {
    fn code_offset(&self) -> i32 { self.code_offset }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction30t {
    pub opcode: Opcode,
    pub code_offset: i32,
}

impl Instruction for ImmutableInstruction30t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction30t for ImmutableInstruction30t {
    fn code_offset(&self) -> i32 { self.code_offset }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction31t {
    pub opcode: Opcode,
    pub register_a: u8,
    pub code_offset: i32,
}

impl Instruction for ImmutableInstruction31t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

impl Instruction31t for ImmutableInstruction31t {
    fn register_a(&self) -> u8 { self.register_a }
    fn code_offset(&self) -> i32 { self.code_offset }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction22x {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u16,
}

impl Instruction for ImmutableInstruction22x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct ImmutableInstruction32x {
    pub opcode: Opcode,
    pub register_a: u16,
    pub register_b: u16,
}

impl Instruction for ImmutableInstruction32x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct ImmutablePayloadInstruction {
    pub opcode: Opcode,
    pub raw_data: Vec<u16>,
}

impl Instruction for ImmutablePayloadInstruction {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
    fn format(&self) -> InstructionFormat {
        match self.raw_data.first() {
            Some(&0x0100) => InstructionFormat::PackedSwitchPayload,
            Some(&0x0200) => InstructionFormat::SparseSwitchPayload,
            Some(&0x0300) => InstructionFormat::ArrayPayload,
            _ => self.opcode.format(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::opcode::InstructionFormat;
    use crate::iface::reference::{Reference, ReferenceType, ReferenceHolder, StringReference, MethodReference};

    #[test]
    fn test_immutable_instruction_10x() {
        let instr = ImmutableInstruction10x { opcode: Opcode::Nop };
        assert_eq!(instr.opcode(), Opcode::Nop);
        assert_eq!(instr.format(), InstructionFormat::Format10x);
        assert_eq!(instr.code_units(), 1);
    }

    #[test]
    fn test_immutable_instruction_21c() {
        let instr = ImmutableInstruction21c {
            opcode: Opcode::ConstString,
            register_a: 0,
            reference: ReferenceHolder::String(StringReference::new("hello")),
        };
        assert_eq!(instr.opcode(), Opcode::ConstString);
        assert_eq!(instr.register_a(), 0);
        let ref_holder = instr.reference();
        assert_eq!(ref_holder.reference_type(), ReferenceType::String);
    }

    #[test]
    fn test_immutable_instruction_35c() {
        let instr = ImmutableInstruction35c {
            opcode: Opcode::InvokeVirtual,
            register_count: 1,
            register_c: 0,
            register_d: 0,
            register_e: 0,
            register_f: 0,
            register_g: 1,
            reference: ReferenceHolder::Method(MethodReference::new(
                "Lcom/A;", "foo", vec![], "V",
            )),
        };
        assert_eq!(instr.opcode(), Opcode::InvokeVirtual);
        assert_eq!(instr.register_count(), 1);
        let ref_holder = instr.reference();
        assert_eq!(ref_holder.reference_type(), ReferenceType::Method);
    }

    #[test]
    fn test_immutable_instruction_51l() {
        let instr = ImmutableInstruction51l {
            opcode: Opcode::ConstWide,
            register_a: 2,
            literal: 12345678i64,
        };
        assert_eq!(instr.opcode(), Opcode::ConstWide);
        assert_eq!(instr.literal(), 12345678i64);
    }
}
