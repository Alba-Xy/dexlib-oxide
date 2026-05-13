use crate::base::opcode::Opcode;
use crate::iface::instruction::Instruction;
use crate::immutable::immutable_reference::ImmutableReferenceHolder;

#[derive(Debug, Clone)]
pub struct BuilderInstruction10x {
    pub opcode: Opcode,
}

impl Instruction for BuilderInstruction10x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction11x {
    pub opcode: Opcode,
    pub register_a: u16,
}

impl Instruction for BuilderInstruction11x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction12x {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
}

impl Instruction for BuilderInstruction12x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction11n {
    pub opcode: Opcode,
    pub register_a: u8,
    pub literal: i8,
}

impl Instruction for BuilderInstruction11n {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction10t {
    pub opcode: Opcode,
    pub code_offset: i16,
}

impl Instruction for BuilderInstruction10t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction20t {
    pub opcode: Opcode,
    pub code_offset: i32,
}

impl Instruction for BuilderInstruction20t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction30t {
    pub opcode: Opcode,
    pub code_offset: i32,
}

impl Instruction for BuilderInstruction30t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction21c {
    pub opcode: Opcode,
    pub register_a: u8,
    pub reference: ImmutableReferenceHolder,
}

impl Instruction for BuilderInstruction21c {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction31c {
    pub opcode: Opcode,
    pub register_a: u8,
    pub reference: ImmutableReferenceHolder,
}

impl Instruction for BuilderInstruction31c {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction22c {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub reference: ImmutableReferenceHolder,
}

impl Instruction for BuilderInstruction22c {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction35c {
    pub opcode: Opcode,
    pub register_count: u8,
    pub register_c: u16,
    pub register_d: u16,
    pub register_e: u16,
    pub register_f: u16,
    pub register_g: u16,
    pub reference: ImmutableReferenceHolder,
}

impl Instruction for BuilderInstruction35c {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction3rc {
    pub opcode: Opcode,
    pub start_register: u16,
    pub register_count: u8,
    pub reference: ImmutableReferenceHolder,
}

impl Instruction for BuilderInstruction3rc {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction21s {
    pub opcode: Opcode,
    pub register_a: u8,
    pub literal: i16,
}

impl Instruction for BuilderInstruction21s {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction31i {
    pub opcode: Opcode,
    pub register_a: u8,
    pub literal: i32,
}

impl Instruction for BuilderInstruction31i {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction51l {
    pub opcode: Opcode,
    pub register_a: u8,
    pub literal: i64,
}

impl Instruction for BuilderInstruction51l {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction21t {
    pub opcode: Opcode,
    pub register_a: u8,
    pub code_offset: i16,
}

impl Instruction for BuilderInstruction21t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction22t {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub code_offset: i16,
}

impl Instruction for BuilderInstruction22t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction22b {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub literal: i8,
}

impl Instruction for BuilderInstruction22b {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction22s {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub literal: i16,
}

impl Instruction for BuilderInstruction22s {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction23x {
    pub opcode: Opcode,
    pub register_a: u8,
    pub register_b: u8,
    pub register_c: u8,
}

impl Instruction for BuilderInstruction23x {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

#[derive(Debug, Clone)]
pub struct BuilderInstruction31t {
    pub opcode: Opcode,
    pub register_a: u8,
    pub code_offset: i32,
}

impl Instruction for BuilderInstruction31t {
    fn opcode(&self) -> Opcode { self.opcode }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(self.clone()) }
}

pub type BuilderInstruction = Box<dyn Instruction>;

pub fn make_nop() -> BuilderInstruction {
    Box::new(BuilderInstruction10x { opcode: Opcode::Nop })
}

pub fn make_return_void() -> BuilderInstruction {
    Box::new(BuilderInstruction10x { opcode: Opcode::ReturnVoid })
}

pub fn make_const_string(register: u8, reference: ImmutableReferenceHolder) -> BuilderInstruction {
    Box::new(BuilderInstruction21c {
        opcode: Opcode::ConstString,
        register_a: register,
        reference,
    })
}

pub fn make_invoke_static(
    register_count: u8,
    registers: [u16; 5],
    reference: ImmutableReferenceHolder,
) -> BuilderInstruction {
    Box::new(BuilderInstruction35c {
        opcode: Opcode::InvokeStatic,
        register_count,
        register_c: registers[0],
        register_d: registers[1],
        register_e: registers[2],
        register_f: registers[3],
        register_g: registers[4],
        reference,
    })
}

pub fn make_move_result(register: u8) -> BuilderInstruction {
    Box::new(BuilderInstruction11x {
        opcode: Opcode::MoveResult,
        register_a: register as u16,
    })
}

pub fn make_goto(offset: i16) -> BuilderInstruction {
    Box::new(BuilderInstruction10t {
        opcode: Opcode::Goto,
        code_offset: offset,
    })
}
