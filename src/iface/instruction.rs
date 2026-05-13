use crate::base::opcode::{InstructionFormat, Opcode, OpcodeFlags};
use crate::iface::reference::ReferenceHolder;

pub trait Instruction: std::fmt::Debug + Send + Sync {
    fn opcode(&self) -> Opcode;
    fn as_any(&self) -> &dyn std::any::Any;
    fn clone_boxed(&self) -> Box<dyn Instruction>;
    fn code_units(&self) -> u32 {
        self.opcode().code_units()
    }
    fn format(&self) -> InstructionFormat {
        self.opcode().format()
    }
    fn flags(&self) -> OpcodeFlags {
        self.opcode().flags()
    }
    fn can_throw(&self) -> bool {
        self.opcode().can_throw()
    }
    fn can_continue(&self) -> bool {
        self.opcode().can_continue()
    }
}

pub trait OneRegisterInstruction: Instruction {
    fn register_a(&self) -> u16;
}

pub trait TwoRegisterInstruction: Instruction {
    fn register_a(&self) -> u8;
    fn register_b(&self) -> u8;
}

pub trait ThreeRegisterInstruction: Instruction {
    fn register_a(&self) -> u8;
    fn register_b(&self) -> u8;
    fn register_c(&self) -> u8;
}

pub trait RegisterRangeInstruction: Instruction {
    fn start_register(&self) -> u16;
    fn register_count(&self) -> u16;
}

pub trait Instruction10t: Instruction {
    fn code_offset(&self) -> i16;
}

pub trait Instruction20t: Instruction {
    fn code_offset(&self) -> i32;
}

pub trait Instruction30t: Instruction {
    fn code_offset(&self) -> i32;
}

pub trait Instruction21t: Instruction {
    fn register_a(&self) -> u8;
    fn code_offset(&self) -> i16;
}

pub trait Instruction22t: Instruction {
    fn register_a(&self) -> u8;
    fn register_b(&self) -> u8;
    fn code_offset(&self) -> i16;
}

pub trait Instruction21c: Instruction {
    fn register_a(&self) -> u8;
    fn reference(&self) -> &ReferenceHolder;
}

pub trait Instruction31c: Instruction {
    fn register_a(&self) -> u8;
    fn reference(&self) -> &ReferenceHolder;
}

pub trait Instruction22c: Instruction {
    fn register_a(&self) -> u8;
    fn register_b(&self) -> u8;
    fn reference(&self) -> &ReferenceHolder;
}

pub trait Instruction22cs: Instruction {
    fn register_a(&self) -> u8;
    fn register_b(&self) -> u8;
    fn field_offset(&self) -> u16;
}

pub trait Instruction35c: Instruction {
    fn register_count(&self) -> u8;
    fn register_c(&self) -> u16;
    fn register_d(&self) -> u16;
    fn register_e(&self) -> u16;
    fn register_f(&self) -> u16;
    fn register_g(&self) -> u16;
    fn reference(&self) -> &ReferenceHolder;
}

pub trait Instruction3rc: Instruction {
    fn start_register(&self) -> u16;
    fn register_count(&self) -> u8;
    fn reference(&self) -> &ReferenceHolder;
}

pub trait Instruction35ms: Instruction {
    fn register_count(&self) -> u8;
    fn register_c(&self) -> u16;
    fn register_d(&self) -> u16;
    fn register_e(&self) -> u16;
    fn register_f(&self) -> u16;
    fn register_g(&self) -> u16;
    fn vtable_index(&self) -> u16;
}

pub trait Instruction3rms: Instruction {
    fn start_register(&self) -> u16;
    fn register_count(&self) -> u8;
    fn vtable_index(&self) -> u16;
}

pub trait Instruction35mi: Instruction {
    fn register_count(&self) -> u8;
    fn register_c(&self) -> u16;
    fn register_d(&self) -> u16;
    fn register_e(&self) -> u16;
    fn register_f(&self) -> u16;
    fn register_g(&self) -> u16;
    fn inline_index(&self) -> u16;
}

pub trait Instruction3rmi: Instruction {
    fn start_register(&self) -> u16;
    fn register_count(&self) -> u8;
    fn inline_index(&self) -> u16;
}

pub trait Instruction45cc: Instruction {
    fn register_count(&self) -> u8;
    fn register_c(&self) -> u16;
    fn register_d(&self) -> u16;
    fn register_e(&self) -> u16;
    fn register_f(&self) -> u16;
    fn register_g(&self) -> u16;
    fn reference(&self) -> &ReferenceHolder;
    fn reference2(&self) -> &ReferenceHolder;
}

pub trait Instruction4rcc: Instruction {
    fn start_register(&self) -> u16;
    fn register_count(&self) -> u8;
    fn reference(&self) -> &ReferenceHolder;
    fn reference2(&self) -> &ReferenceHolder;
}

pub trait Instruction21s: Instruction {
    fn register_a(&self) -> u8;
    fn literal(&self) -> i16;
}

pub trait Instruction31i: Instruction {
    fn register_a(&self) -> u8;
    fn literal(&self) -> i32;
}

pub trait Instruction51l: Instruction {
    fn register_a(&self) -> u8;
    fn literal(&self) -> i64;
}

pub trait Instruction22s: Instruction {
    fn register_a(&self) -> u8;
    fn register_b(&self) -> u8;
    fn literal(&self) -> i16;
}

pub trait Instruction22b: Instruction {
    fn register_a(&self) -> u8;
    fn register_b(&self) -> u8;
    fn literal(&self) -> i8;
}

pub trait Instruction11n: Instruction {
    fn register_a(&self) -> u8;
    fn literal(&self) -> i8;
}

pub trait Instruction31t: Instruction {
    fn register_a(&self) -> u8;
    fn code_offset(&self) -> i32;
}

pub trait Instruction20bc: Instruction {
    fn verification_error(&self) -> u16;
}

pub trait PackedSwitchPayload: Instruction {
    fn switch_count(&self) -> u16;
    fn first_key(&self) -> i32;
    fn targets(&self) -> &[i32];
}

pub trait SparseSwitchPayload: Instruction {
    fn switch_count(&self) -> u16;
    fn keys(&self) -> &[i32];
    fn targets(&self) -> &[i32];
}

pub trait ArrayPayload: Instruction {
    fn element_width(&self) -> u16;
    fn array_length(&self) -> u32;
    fn data(&self) -> &[u8];
}