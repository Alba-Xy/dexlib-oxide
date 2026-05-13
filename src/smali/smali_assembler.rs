use std::collections::HashMap;

use crate::base::access_flags::AccessFlags;
use crate::base::opcode::Opcode;
use crate::iface::field::Field;
use crate::iface::method::Method;
use crate::iface::instruction::Instruction;
use crate::immutable::immutable_class_def::ImmutableClassDef;
use crate::immutable::immutable_field::ImmutableField;
use crate::immutable::immutable_method::{ImmutableMethod, ImmutableMethodImplementation};
use crate::iface::reference::{ReferenceHolder, StringReference, TypeReference, FieldReference, MethodReference};
use crate::immutable::immutable_instruction::*;
use crate::writer::dex_pool::DexPool;
use crate::smali::smali_parser::{self, SmaliClass, SmaliInstruction};

pub fn assemble_smali(smali_text: &str) -> Result<Vec<u8>, String> {
    let smali_class = smali_parser::parse_smali(smali_text)
        .map_err(|e| e.to_string())?;

    let class_def = smali_class_to_class_def(&smali_class);

    let mut pool = DexPool::new();
    pool.intern_class(&class_def);
    pool.to_bytes()
}

pub fn assemble_smali_files(files: &HashMap<String, String>) -> Result<Vec<u8>, String> {
    let mut pool = DexPool::new();

    for (_filename, content) in files {
        let smali_class = smali_parser::parse_smali(content)
            .map_err(|e| format!("{}: {}", _filename, e))?;
        let class_def = smali_class_to_class_def(&smali_class);
        pool.intern_class(&class_def);
    }

    pool.to_bytes()
}

fn smali_class_to_class_def(smali_class: &SmaliClass) -> ImmutableClassDef {
    let static_fields = smali_class.fields.iter()
        .filter(|f| f.access_flags.contains(AccessFlags::ACC_STATIC))
        .map(|f| smali_field_to_field(f))
        .collect();

    let instance_fields = smali_class.fields.iter()
        .filter(|f| !f.access_flags.contains(AccessFlags::ACC_STATIC))
        .map(|f| smali_field_to_field(f))
        .collect();

    let direct_methods = smali_class.methods.iter()
        .filter(|m| {
            m.access_flags.contains(AccessFlags::ACC_STATIC)
                || m.access_flags.contains(AccessFlags::ACC_PRIVATE)
                || m.name == "<init>"
        })
        .map(|m| smali_method_to_method(m, &smali_class.type_descriptor))
        .collect();

    let virtual_methods = smali_class.methods.iter()
        .filter(|m| {
            !m.access_flags.contains(AccessFlags::ACC_STATIC)
                && !m.access_flags.contains(AccessFlags::ACC_PRIVATE)
                && m.name != "<init>"
        })
        .map(|m| smali_method_to_method(m, &smali_class.type_descriptor))
        .collect();

    ImmutableClassDef {
        type_descriptor: smali_class.type_descriptor.clone(),
        access_flags: smali_class.access_flags,
        superclass: smali_class.superclass.clone(),
        interfaces: smali_class.interfaces.clone(),
        source_file: smali_class.source_file.clone(),
        annotations: crate::iface::annotation::AnnotationSet { annotations: Vec::new() },
        static_fields,
        instance_fields,
        direct_methods,
        virtual_methods,
    }
}

fn smali_field_to_field(field: &crate::smali::smali_parser::SmaliField) -> Box<dyn Field> {
    Box::new(ImmutableField {
        defining_class: String::new(),
        name: field.name.clone(),
        field_type: field.field_type.clone(),
        access_flags: field.access_flags,
        annotations: Vec::new(),
        initial_value: None,
        hidden_api_restrictions: 0,
    })
}

fn smali_method_to_method(method: &crate::smali::smali_parser::SmaliMethod, defining_class: &str) -> Box<dyn Method> {
    let implementation = if method.instructions.is_empty() {
        None
    } else {
        Some(smali_instructions_to_impl(method))
    };

    Box::new(ImmutableMethod {
        defining_class: defining_class.to_string(),
        name: method.name.clone(),
        parameter_types: method.parameter_types.clone(),
        return_type: method.return_type.clone(),
        access_flags: method.access_flags,
        annotations: Vec::new(),
        hidden_api_restrictions: 0,
        implementation,
    })
}

fn smali_instructions_to_impl(method: &crate::smali::smali_parser::SmaliMethod) -> ImmutableMethodImplementation {
    let instructions: Vec<Box<dyn Instruction>> = method.instructions.iter()
        .map(|instr| smali_instruction_to_immutable(instr))
        .collect();

    ImmutableMethodImplementation {
        register_count: method.register_count,
        instructions,
        try_blocks: Vec::new(),
        debug_items: Vec::new(),
    }
}

fn smali_instruction_to_immutable(instr: &SmaliInstruction) -> Box<dyn Instruction> {
    match instr {
        SmaliInstruction::Nop => Box::new(ImmutableInstruction10x {
            opcode: Opcode::Nop,
        }),
        SmaliInstruction::ReturnVoid => Box::new(ImmutableInstruction10x {
            opcode: Opcode::ReturnVoid,
        }),
        SmaliInstruction::ConstString { register, value } => {
            Box::new(ImmutableInstruction21c {
                opcode: Opcode::ConstString,
                register_a: *register,
                reference: ReferenceHolder::String(StringReference {
                    string: value.clone(),
                }),
            })
        }
        SmaliInstruction::Const { register, value } => {
            Box::new(ImmutableInstruction31i {
                opcode: Opcode::Const,
                register_a: *register,
                literal: *value,
            })
        }
        SmaliInstruction::ConstWide { register, value } => {
            Box::new(ImmutableInstruction51l {
                opcode: Opcode::ConstWide,
                register_a: *register,
                literal: *value,
            })
        }
        SmaliInstruction::InvokeStatic { method_ref, registers } => {
            let (c, d, e, f, g) = pack_invoke_registers(registers);
            Box::new(ImmutableInstruction35c {
                opcode: Opcode::InvokeStatic,
                register_count: registers.len() as u8,
                register_c: c as u16,
                register_d: d as u16,
                register_e: e as u16,
                register_f: f as u16,
                register_g: g as u16,
                reference: parse_method_ref(method_ref),
            })
        }
        SmaliInstruction::InvokeVirtual { method_ref, registers } => {
            let (c, d, e, f, g) = pack_invoke_registers(registers);
            Box::new(ImmutableInstruction35c {
                opcode: Opcode::InvokeVirtual,
                register_count: registers.len() as u8,
                register_c: c as u16,
                register_d: d as u16,
                register_e: e as u16,
                register_f: f as u16,
                register_g: g as u16,
                reference: parse_method_ref(method_ref),
            })
        }
        SmaliInstruction::InvokeDirect { method_ref, registers } => {
            let (c, d, e, f, g) = pack_invoke_registers(registers);
            Box::new(ImmutableInstruction35c {
                opcode: Opcode::InvokeDirect,
                register_count: registers.len() as u8,
                register_c: c as u16,
                register_d: d as u16,
                register_e: e as u16,
                register_f: f as u16,
                register_g: g as u16,
                reference: parse_method_ref(method_ref),
            })
        }
        SmaliInstruction::InvokeSuper { method_ref, registers } => {
            let (c, d, e, f, g) = pack_invoke_registers(registers);
            Box::new(ImmutableInstruction35c {
                opcode: Opcode::InvokeSuper,
                register_count: registers.len() as u8,
                register_c: c as u16,
                register_d: d as u16,
                register_e: e as u16,
                register_f: f as u16,
                register_g: g as u16,
                reference: parse_method_ref(method_ref),
            })
        }
        SmaliInstruction::InvokeInterface { method_ref, registers } => {
            let (c, d, e, f, g) = pack_invoke_registers(registers);
            Box::new(ImmutableInstruction35c {
                opcode: Opcode::InvokeInterface,
                register_count: registers.len() as u8,
                register_c: c as u16,
                register_d: d as u16,
                register_e: e as u16,
                register_f: f as u16,
                register_g: g as u16,
                reference: parse_method_ref(method_ref),
            })
        }
        SmaliInstruction::MoveResult { register } => {
            Box::new(ImmutableInstruction11x {
                opcode: Opcode::MoveResult,
                register_a: *register as u16,
            })
        }
        SmaliInstruction::MoveResultObject { register } => {
            Box::new(ImmutableInstruction11x {
                opcode: Opcode::MoveResultObject,
                register_a: *register as u16,
            })
        }
        SmaliInstruction::MoveResultWide { register } => {
            Box::new(ImmutableInstruction11x {
                opcode: Opcode::MoveResultWide,
                register_a: *register as u16,
            })
        }
        SmaliInstruction::Move { dest, src } => {
            Box::new(ImmutableInstruction12x {
                opcode: Opcode::Move,
                register_a: *dest,
                register_b: *src,
            })
        }
        SmaliInstruction::MoveWide { dest, src } => {
            Box::new(ImmutableInstruction12x {
                opcode: Opcode::MoveWide,
                register_a: *dest,
                register_b: *src,
            })
        }
        SmaliInstruction::MoveObject { dest, src } => {
            Box::new(ImmutableInstruction12x {
                opcode: Opcode::MoveObject,
                register_a: *dest,
                register_b: *src,
            })
        }
        SmaliInstruction::Return { register } => {
            Box::new(ImmutableInstruction11x {
                opcode: Opcode::Return,
                register_a: *register as u16,
            })
        }
        SmaliInstruction::ReturnObject { register } => {
            Box::new(ImmutableInstruction11x {
                opcode: Opcode::ReturnObject,
                register_a: *register as u16,
            })
        }
        SmaliInstruction::Throw { register } => {
            Box::new(ImmutableInstruction11x {
                opcode: Opcode::Throw,
                register_a: *register as u16,
            })
        }
        SmaliInstruction::Goto { .. } => {
            Box::new(ImmutableInstruction10t {
                opcode: Opcode::Goto,
                code_offset: 0,
            })
        }
        SmaliInstruction::IfEqz { register, .. } => {
            Box::new(ImmutableInstruction21t {
                opcode: Opcode::IfEqz,
                register_a: *register,
                code_offset: 0,
            })
        }
        SmaliInstruction::IfNez { register, .. } => {
            Box::new(ImmutableInstruction21t {
                opcode: Opcode::IfNez,
                register_a: *register,
                code_offset: 0,
            })
        }
        SmaliInstruction::NewInstance { register, type_ref } => {
            Box::new(ImmutableInstruction21c {
                opcode: Opcode::NewInstance,
                register_a: *register,
                reference: ReferenceHolder::Type(TypeReference {
                    type_descriptor: type_ref.clone(),
                }),
            })
        }
        SmaliInstruction::CheckCast { register, type_ref } => {
            Box::new(ImmutableInstruction21c {
                opcode: Opcode::CheckCast,
                register_a: *register,
                reference: ReferenceHolder::Type(TypeReference {
                    type_descriptor: type_ref.clone(),
                }),
            })
        }
        SmaliInstruction::IGet { dest, obj, field_ref } => {
            let (class, name, ftype) = parse_field_ref_string(field_ref);
            Box::new(ImmutableInstruction22c {
                opcode: Opcode::Iget,
                register_a: *dest,
                register_b: *obj,
                reference: ReferenceHolder::Field(FieldReference {
                    defining_class: class,
                    field_name: name,
                    field_type: ftype,
                }),
            })
        }
        SmaliInstruction::IPut { src, obj, field_ref } => {
            let (class, name, ftype) = parse_field_ref_string(field_ref);
            Box::new(ImmutableInstruction22c {
                opcode: Opcode::Iput,
                register_a: *src,
                register_b: *obj,
                reference: ReferenceHolder::Field(FieldReference {
                    defining_class: class,
                    field_name: name,
                    field_type: ftype,
                }),
            })
        }
        SmaliInstruction::SGet { dest, field_ref } => {
            let (class, name, ftype) = parse_field_ref_string(field_ref);
            Box::new(ImmutableInstruction21c {
                opcode: Opcode::Sget,
                register_a: *dest,
                reference: ReferenceHolder::Field(FieldReference {
                    defining_class: class,
                    field_name: name,
                    field_type: ftype,
                }),
            })
        }
        SmaliInstruction::SPut { src, field_ref } => {
            let (class, name, ftype) = parse_field_ref_string(field_ref);
            Box::new(ImmutableInstruction21c {
                opcode: Opcode::Sput,
                register_a: *src,
                reference: ReferenceHolder::Field(FieldReference {
                    defining_class: class,
                    field_name: name,
                    field_type: ftype,
                }),
            })
        }
        SmaliInstruction::Label { .. } => {
            Box::new(ImmutableInstruction10x {
                opcode: Opcode::Nop,
            })
        }
    }
}

fn pack_invoke_registers(registers: &[u8]) -> (u8, u8, u8, u8, u8) {
    let c = registers.get(0).copied().unwrap_or(0);
    let d = registers.get(1).copied().unwrap_or(0);
    let e = registers.get(2).copied().unwrap_or(0);
    let f = registers.get(3).copied().unwrap_or(0);
    let g = registers.get(4).copied().unwrap_or(0);
    (c, d, e, f, g)
}

fn parse_method_ref(method_ref: &str) -> ReferenceHolder {
    let arrow_pos = method_ref.find("->").unwrap_or(0);
    let defining_class = method_ref[..arrow_pos].to_string();
    let after_arrow = &method_ref[arrow_pos + 2..];

    let paren_open = after_arrow.find('(').unwrap_or(after_arrow.len());
    let method_name = after_arrow[..paren_open].to_string();
    let rest = &after_arrow[paren_open..];

    let paren_close = rest.rfind(')').unwrap_or(0);
    let params_str = &rest[1..paren_close];
    let return_type = rest[paren_close + 1..].to_string();

    let parameter_types = parse_type_list_from_smali(params_str);

    ReferenceHolder::Method(MethodReference {
        defining_class,
        method_name,
        parameter_types,
        return_type,
    })
}

fn parse_field_ref_string(field_ref: &str) -> (String, String, String) {
    let arrow_pos = field_ref.find("->").unwrap_or(0);
    let defining_class = field_ref[..arrow_pos].to_string();
    let after_arrow = &field_ref[arrow_pos + 2..];

    let colon_pos = after_arrow.find(':').unwrap_or(after_arrow.len());
    let name = after_arrow[..colon_pos].to_string();
    let ftype = after_arrow[colon_pos + 1..].to_string();

    (defining_class, name, ftype)
}

fn parse_type_list_from_smali(types: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = types.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            'V' | 'Z' | 'B' | 'S' | 'C' | 'I' | 'J' | 'F' | 'D' => {
                result.push(chars[i].to_string());
                i += 1;
            }
            'L' => {
                let start = i;
                while i < chars.len() && chars[i] != ';' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                result.push(chars[start..i].iter().collect());
            }
            '[' => {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] == '[' {
                    i += 1;
                }
                if i < chars.len() {
                    if chars[i] == 'L' {
                        while i < chars.len() && chars[i] != ';' {
                            i += 1;
                        }
                        if i < chars.len() {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }
                result.push(chars[start..i].iter().collect());
            }
            _ => {
                i += 1;
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iface::reference::{Reference, ReferenceType};

    #[test]
    fn test_assemble_simple_smali() {
        let smali = r#"
.class public Lcom/test/Hello;
.super Ljava/lang/Object;

.method public static main()V
    .registers 1
    const-string v0, "Hello World"
    return-void
.end method
"#;
        let result = assemble_smali(smali);
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(bytes.len() >= 0x70);
        assert_eq!(&bytes[0..4], b"dex\n");
    }

    #[test]
    fn test_assemble_with_invoke() {
        let smali = r#"
.class public Lcom/test/Hello;
.super Ljava/lang/Object;

.method public static foo()V
    .registers 2
    invoke-static {}, Lcom/Helper;->doStuff()V
    return-void
.end method
"#;
        let result = assemble_smali(smali);
        assert!(result.is_ok());
    }

    #[test]
    fn test_assemble_smali_files() {
        let mut files = HashMap::new();
        files.insert("Hello.smali".to_string(), r#"
.class public Lcom/test/Hello;
.super Ljava/lang/Object;

.method public static main()V
    .registers 1
    const-string v0, "Hello"
    return-void
.end method
"#.to_string());

        let result = assemble_smali_files(&files);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_method_ref() {
        let ref_holder = parse_method_ref("Lcom/A;->foo(ILjava/lang/String;)V");
        assert_eq!(ref_holder.reference_type(), ReferenceType::Method);
    }

    #[test]
    fn test_parse_field_ref_string() {
        let (class, name, ftype) = parse_field_ref_string("Lcom/A;->x:I");
        assert_eq!(class, "Lcom/A;");
        assert_eq!(name, "x");
        assert_eq!(ftype, "I");
    }
}
