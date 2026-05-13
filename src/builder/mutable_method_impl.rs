use crate::iface::debug_item::DebugItem;
use crate::iface::instruction::Instruction;
use crate::iface::method::{MethodImplementation, TryBlock};
use crate::builder::builder_instruction::BuilderInstruction;

#[derive(Debug)]
pub struct MutableMethodImplementation {
    register_count: u16,
    instructions: Vec<BuilderInstruction>,
    try_blocks: Vec<TryBlock>,
    debug_items: Vec<DebugItem>,
}

impl MutableMethodImplementation {
    pub fn new(register_count: u16, instructions: Vec<BuilderInstruction>) -> Self {
        Self {
            register_count,
            instructions,
            try_blocks: Vec::new(),
            debug_items: Vec::new(),
        }
    }

    pub fn from_implementation(impl_data: &dyn MethodImplementation) -> Self {
        let instructions: Vec<BuilderInstruction> = impl_data.instructions()
            .iter()
            .map(|i| {
                let opcode = i.opcode();
                Box::new(WrappedInstruction { opcode }) as BuilderInstruction
            })
            .collect();

        Self {
            register_count: impl_data.register_count(),
            instructions,
            try_blocks: impl_data.try_blocks().to_vec(),
            debug_items: impl_data.debug_items().to_vec(),
        }
    }

    pub fn with_register_count(register_count: u16) -> Self {
        Self {
            register_count,
            instructions: Vec::new(),
            try_blocks: Vec::new(),
            debug_items: Vec::new(),
        }
    }

    pub fn register_count(&self) -> u16 {
        self.register_count
    }

    pub fn set_register_count(&mut self, count: u16) {
        self.register_count = count;
    }

    pub fn instructions(&self) -> &[BuilderInstruction] {
        &self.instructions
    }

    pub fn add_instruction(&mut self, index: usize, instruction: BuilderInstruction) {
        if index > self.instructions.len() {
            return;
        }
        let code_units = instruction.code_units() as u32;
        let insert_addr = self.instruction_address(index);

        self.instructions.insert(index, instruction);

        for try_block in &mut self.try_blocks {
            if try_block.start_code_address >= insert_addr {
                try_block.start_code_address += code_units;
            } else if try_block.start_code_address + try_block.code_unit_count as u32 > insert_addr {
                try_block.code_unit_count += code_units as u16;
            }
        }
    }

    pub fn replace_instruction(&mut self, index: usize, instruction: BuilderInstruction) {
        if index >= self.instructions.len() {
            return;
        }
        let old_code_units = self.instructions[index].code_units() as u32;
        let new_code_units = instruction.code_units() as u32;
        let diff = new_code_units as i32 - old_code_units as i32;

        self.instructions[index] = instruction;

        if diff != 0 {
            let replace_addr = self.instruction_address(index);
            let end_addr = replace_addr + old_code_units;
            for try_block in &mut self.try_blocks {
                if try_block.start_code_address >= end_addr {
                    try_block.start_code_address = (try_block.start_code_address as i32 + diff) as u32;
                } else if try_block.start_code_address + try_block.code_unit_count as u32 > end_addr {
                    try_block.code_unit_count = (try_block.code_unit_count as i32 + diff) as u16;
                }
            }
        }
    }

    pub fn remove_instruction(&mut self, index: usize) {
        if index >= self.instructions.len() {
            return;
        }
        let removed_code_units = self.instructions[index].code_units() as u32;
        let remove_addr = self.instruction_address(index);

        self.instructions.remove(index);

        for try_block in &mut self.try_blocks {
            if try_block.start_code_address >= remove_addr + removed_code_units {
                try_block.start_code_address -= removed_code_units;
            } else if try_block.start_code_address >= remove_addr {
                try_block.code_unit_count = try_block.code_unit_count.saturating_sub(removed_code_units as u16);
                if try_block.code_unit_count == 0 {
                    try_block.start_code_address = 0;
                }
            } else if try_block.start_code_address + try_block.code_unit_count as u32 > remove_addr {
                try_block.code_unit_count = try_block.code_unit_count.saturating_sub(removed_code_units as u16);
            }
        }
    }

    pub fn swap_instructions(&mut self, i: usize, j: usize) {
        if i >= self.instructions.len() || j >= self.instructions.len() || i == j {
            return;
        }
        self.instructions.swap(i, j);
    }

    pub fn try_blocks(&self) -> &[TryBlock] {
        &self.try_blocks
    }

    pub fn try_blocks_mut(&mut self) -> &mut Vec<TryBlock> {
        &mut self.try_blocks
    }

    pub fn debug_items(&self) -> &[DebugItem] {
        &self.debug_items
    }

    pub fn debug_items_mut(&mut self) -> &mut Vec<DebugItem> {
        &mut self.debug_items
    }

    fn instruction_address(&self, index: usize) -> u32 {
        let mut addr = 0u32;
        for (i, instr) in self.instructions.iter().enumerate() {
            if i == index {
                break;
            }
            addr += instr.code_units();
        }
        addr
    }
}

impl MethodImplementation for MutableMethodImplementation {
    fn register_count(&self) -> u16 { self.register_count }
    fn instructions(&self) -> &[Box<dyn Instruction>] { &self.instructions }
    fn try_blocks(&self) -> &[TryBlock] { &self.try_blocks }
    fn debug_items(&self) -> &[DebugItem] { &self.debug_items }
}

#[derive(Debug)]
struct WrappedInstruction {
    opcode: crate::base::opcode::Opcode,
}

impl Instruction for WrappedInstruction {
    fn opcode(&self) -> crate::base::opcode::Opcode {
        self.opcode
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn clone_boxed(&self) -> Box<dyn Instruction> { Box::new(WrappedInstruction { opcode: self.opcode }) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::opcode::Opcode;
    use crate::builder::builder_instruction::*;
    use crate::iface::method::ExceptionHandler;

    fn make_simple_impl() -> MutableMethodImplementation {
        let mut impl_ = MutableMethodImplementation::new(2, Vec::new());
        impl_.add_instruction(0, make_nop());
        impl_.add_instruction(1, make_const_string(
            0,
            crate::immutable::immutable_reference::ImmutableReferenceHolder::String(
                crate::immutable::immutable_reference::ImmutableStringReference::new("test")
            ),
        ));
        impl_.add_instruction(2, make_return_void());
        impl_
    }

    #[test]
    fn test_create_from_scratch() {
        let impl_ = make_simple_impl();
        assert_eq!(impl_.register_count(), 2);
        assert_eq!(impl_.instructions().len(), 3);
        assert_eq!(impl_.instructions()[0].opcode(), Opcode::Nop);
        assert_eq!(impl_.instructions()[1].opcode(), Opcode::ConstString);
        assert_eq!(impl_.instructions()[2].opcode(), Opcode::ReturnVoid);
    }

    #[test]
    fn test_add_instruction_at_position() {
        let mut impl_ = make_simple_impl();
        impl_.add_instruction(1, make_nop());
        assert_eq!(impl_.instructions().len(), 4);
        assert_eq!(impl_.instructions()[0].opcode(), Opcode::Nop);
        assert_eq!(impl_.instructions()[1].opcode(), Opcode::Nop);
        assert_eq!(impl_.instructions()[2].opcode(), Opcode::ConstString);
        assert_eq!(impl_.instructions()[3].opcode(), Opcode::ReturnVoid);
    }

    #[test]
    fn test_replace_instruction() {
        let mut impl_ = make_simple_impl();
        impl_.replace_instruction(1, make_nop());
        assert_eq!(impl_.instructions().len(), 3);
        assert_eq!(impl_.instructions()[1].opcode(), Opcode::Nop);
    }

    #[test]
    fn test_remove_instruction() {
        let mut impl_ = make_simple_impl();
        impl_.remove_instruction(1);
        assert_eq!(impl_.instructions().len(), 2);
        assert_eq!(impl_.instructions()[0].opcode(), Opcode::Nop);
        assert_eq!(impl_.instructions()[1].opcode(), Opcode::ReturnVoid);
    }

    #[test]
    fn test_swap_instructions() {
        let mut impl_ = make_simple_impl();
        impl_.swap_instructions(0, 2);
        assert_eq!(impl_.instructions()[0].opcode(), Opcode::ReturnVoid);
        assert_eq!(impl_.instructions()[2].opcode(), Opcode::Nop);
    }

    #[test]
    fn test_set_register_count() {
        let mut impl_ = make_simple_impl();
        impl_.set_register_count(10);
        assert_eq!(impl_.register_count(), 10);
    }

    #[test]
    fn test_add_instruction_adjusts_try_block() {
        let mut impl_ = MutableMethodImplementation::new(2, Vec::new());
        impl_.add_instruction(0, make_nop());
        impl_.add_instruction(1, make_return_void());
        impl_.try_blocks.push(TryBlock {
            start_code_address: 0,
            code_unit_count: 2,
            handlers: vec![ExceptionHandler {
                handler_type: Some("Lcom/Exception;".to_string()),
                handler_code_address: 2,
            }],
        });

        impl_.add_instruction(1, make_nop());

        assert_eq!(impl_.try_blocks()[0].start_code_address, 0);
        assert_eq!(impl_.try_blocks()[0].code_unit_count, 3);
    }

    #[test]
    fn test_remove_instruction_adjusts_try_block() {
        let mut impl_ = MutableMethodImplementation::new(2, Vec::new());
        impl_.add_instruction(0, make_nop());
        impl_.add_instruction(1, make_nop());
        impl_.add_instruction(2, make_return_void());
        impl_.try_blocks.push(TryBlock {
            start_code_address: 0,
            code_unit_count: 3,
            handlers: vec![ExceptionHandler {
                handler_type: Some("Lcom/Exception;".to_string()),
                handler_code_address: 2,
            }],
        });

        impl_.remove_instruction(1);

        assert_eq!(impl_.try_blocks()[0].start_code_address, 0);
        assert_eq!(impl_.try_blocks()[0].code_unit_count, 2);
    }

    #[test]
    fn test_out_of_bounds_operations() {
        let mut impl_ = make_simple_impl();
        impl_.add_instruction(100, make_nop());
        assert_eq!(impl_.instructions().len(), 3);

        impl_.remove_instruction(100);
        assert_eq!(impl_.instructions().len(), 3);

        impl_.replace_instruction(100, make_nop());
        assert_eq!(impl_.instructions().len(), 3);

        impl_.swap_instructions(0, 100);
        assert_eq!(impl_.instructions()[0].opcode(), Opcode::Nop);
    }
}
