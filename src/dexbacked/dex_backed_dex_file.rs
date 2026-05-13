use crate::base::access_flags::AccessFlags;
use crate::base::opcode::{Opcode, InstructionFormat};
use crate::dexbacked::dex_backed_class_def::DexBackedClassDef;
use crate::dexbacked::dex_backed_field::DexBackedField;
use crate::dexbacked::dex_backed_instruction::{decode_all_instructions, DexBackedInstruction};
use crate::dexbacked::dex_backed_method::{DexBackedMethod, DexBackedMethodImplementation};
use crate::dexbacked::header::*;
use crate::iface::annotation::AnnotationSet;
use crate::iface::class_def::ClassDef;
use crate::iface::dex_file::DexFile;
use crate::iface::field::Field;
use crate::iface::method::{Method, TryBlock};
use crate::iface::reference::*;
use crate::iface::instruction::Instruction;

#[derive(Debug)]
pub struct DexBackedDexFile {
    data: Vec<u8>,
    header: DexHeader,
    string_ids: Vec<StringIdItem>,
    type_ids: Vec<TypeIdItem>,
    proto_ids: Vec<ProtoIdItem>,
    field_ids: Vec<FieldIdItem>,
    method_ids: Vec<MethodIdItem>,
    class_def_items: Vec<ClassDefItem>,
}

impl DexBackedDexFile {
    pub fn from_bytes(data: Vec<u8>) -> Result<Self, String> {
        let header = DexHeader::parse(&data)?;

        let string_ids = parse_string_ids(&data, header.string_ids_off, header.string_ids_size);
        let type_ids = parse_type_ids(&data, header.type_ids_off, header.type_ids_size);
        let proto_ids = parse_proto_ids(&data, header.proto_ids_off, header.proto_ids_size);
        let field_ids = parse_field_ids(&data, header.field_ids_off, header.field_ids_size);
        let method_ids = parse_method_ids(&data, header.method_ids_off, header.method_ids_size);
        let class_def_items = parse_class_defs(&data, header.class_defs_off, header.class_defs_size);

        Ok(Self {
            data,
            header,
            string_ids,
            type_ids,
            proto_ids,
            field_ids,
            method_ids,
            class_def_items,
        })
    }

    pub fn header(&self) -> &DexHeader { &self.header }

    pub fn string_count(&self) -> u32 { self.header.string_ids_size }
    pub fn type_count(&self) -> u32 { self.header.type_ids_size }
    pub fn proto_count(&self) -> u32 { self.header.proto_ids_size }
    pub fn field_count(&self) -> u32 { self.header.field_ids_size }
    pub fn method_count(&self) -> u32 { self.header.method_ids_size }
    pub fn class_count(&self) -> u32 { self.header.class_defs_size }

    pub fn get_string(&self, idx: u32) -> Option<String> {
        if idx as usize >= self.string_ids.len() {
            return None;
        }
        let string_data_off = self.string_ids[idx as usize].string_data_off;
        parse_string_data(&self.data, string_data_off).ok()
    }

    pub fn get_type_descriptor(&self, idx: u32) -> Option<String> {
        if idx as usize >= self.type_ids.len() {
            return None;
        }
        let descriptor_idx = self.type_ids[idx as usize].descriptor_idx;
        self.get_string(descriptor_idx)
    }

    pub fn get_proto_shorty(&self, idx: u32) -> Option<String> {
        if idx as usize >= self.proto_ids.len() {
            return None;
        }
        self.get_string(self.proto_ids[idx as usize].shorty_idx)
    }

    pub fn get_proto_return_type(&self, idx: u32) -> Option<String> {
        if idx as usize >= self.proto_ids.len() {
            return None;
        }
        self.get_type_descriptor(self.proto_ids[idx as usize].return_type_idx)
    }

    pub fn get_proto_parameter_types(&self, idx: u32) -> Vec<String> {
        if idx as usize >= self.proto_ids.len() {
            return Vec::new();
        }
        let parameters_off = self.proto_ids[idx as usize].parameters_off;
        let type_indices = parse_proto_parameter_types(&self.data, parameters_off);
        type_indices.iter()
            .filter_map(|&ti| self.get_type_descriptor(ti))
            .collect()
    }

    pub fn get_field_reference(&self, idx: u32) -> Option<(String, String, String)> {
        if idx as usize >= self.field_ids.len() {
            return None;
        }
        let field = &self.field_ids[idx as usize];
        let defining_class = self.get_type_descriptor(field.class_idx as u32)?;
        let field_type = self.get_type_descriptor(field.type_idx as u32)?;
        let name = self.get_string(field.name_idx)?;
        Some((defining_class, name, field_type))
    }

    pub fn get_method_reference(&self, idx: u32) -> Option<(String, String, Vec<String>, String)> {
        if idx as usize >= self.method_ids.len() {
            return None;
        }
        let method = &self.method_ids[idx as usize];
        let defining_class = self.get_type_descriptor(method.class_idx as u32)?;
        let name = self.get_string(method.name_idx)?;
        let return_type = self.get_proto_return_type(method.proto_idx as u32)?;
        let parameter_types = self.get_proto_parameter_types(method.proto_idx as u32);
        Some((defining_class, name, parameter_types, return_type))
    }

    pub fn parse_classes(&self) -> Vec<Box<dyn ClassDef>> {
        let mut result = Vec::new();
        for class_def_item in &self.class_def_items {
            if let Some(class_def) = self.parse_class_def(class_def_item) {
                result.push(Box::new(class_def) as Box<dyn ClassDef>);
            }
        }
        result
    }

    fn parse_class_def(&self, item: &ClassDefItem) -> Option<DexBackedClassDef> {
        let type_descriptor = self.get_type_descriptor(item.class_idx)?;

        let superclass = if item.superclass_idx == 0xFFFFFFFF {
            None
        } else {
            self.get_type_descriptor(item.superclass_idx)
        };

        let interfaces = parse_interface_list(&self.data, item.interfaces_off)
            .iter()
            .filter_map(|&idx| self.get_type_descriptor(idx as u32))
            .collect();

        let source_file = if item.source_file_idx == 0xFFFFFFFF {
            None
        } else {
            self.get_string(item.source_file_idx)
        };

        let (static_fields, instance_fields, direct_methods, virtual_methods) =
            if item.class_data_off != 0 {
                self.parse_class_data(item.class_data_off)
            } else {
                (Vec::new(), Vec::new(), Vec::new(), Vec::new())
            };

        Some(DexBackedClassDef {
            type_descriptor,
            access_flags: AccessFlags::from_bits_truncate(item.access_flags),
            superclass,
            interfaces,
            source_file,
            annotations: AnnotationSet { annotations: Vec::new() },
            static_fields,
            instance_fields,
            direct_methods,
            virtual_methods,
        })
    }

    fn parse_class_data(
        &self,
        class_data_off: u32,
    ) -> (
        Vec<Box<dyn Field>>,
        Vec<Box<dyn Field>>,
        Vec<Box<dyn Method>>,
        Vec<Box<dyn Method>>,
    ) {
        let (header, static_fields_data, instance_fields_data, direct_methods_data, virtual_methods_data) =
            parse_class_data(&self.data, class_data_off).unwrap_or_else(|_| {
                (ClassDataHeader {
                    static_fields_size: 0,
                    instance_fields_size: 0,
                    direct_methods_size: 0,
                    virtual_methods_size: 0,
                }, Vec::new(), Vec::new(), Vec::new(), Vec::new())
            });

        let defining_class = String::new();

        let static_fields: Vec<Box<dyn Field>> = static_fields_data.iter()
            .filter_map(|f| self.build_field(&defining_class, f.field_idx_diff, f.access_flags))
            .map(|f| Box::new(f) as Box<dyn Field>)
            .collect();

        let instance_fields: Vec<Box<dyn Field>> = instance_fields_data.iter()
            .filter_map(|f| self.build_field(&defining_class, f.field_idx_diff, f.access_flags))
            .map(|f| Box::new(f) as Box<dyn Field>)
            .collect();

        let direct_methods: Vec<Box<dyn Method>> = direct_methods_data.iter()
            .filter_map(|m| self.build_method(&defining_class, m.method_idx_diff, m.access_flags, m.code_off))
            .map(|m| Box::new(m) as Box<dyn Method>)
            .collect();

        let virtual_methods: Vec<Box<dyn Method>> = virtual_methods_data.iter()
            .filter_map(|m| self.build_method(&defining_class, m.method_idx_diff, m.access_flags, m.code_off))
            .map(|m| Box::new(m) as Box<dyn Method>)
            .collect();

        let _ = header;
        (static_fields, instance_fields, direct_methods, virtual_methods)
    }

    fn build_field(&self, _defining_class: &str, field_idx: u32, access_flags: u32) -> Option<DexBackedField> {
        let (defining_class, name, field_type) = self.get_field_reference(field_idx)?;
        Some(DexBackedField {
            defining_class,
            name,
            field_type,
            access_flags: AccessFlags::from_bits_truncate(access_flags),
            field_index: field_idx,
            annotations: Vec::new(),
            initial_value: None,
            hidden_api_restrictions: 0,
        })
    }

    fn build_method(&self, _defining_class: &str, method_idx: u32, access_flags: u32, code_off: u32) -> Option<DexBackedMethod> {
        let (defining_class, name, parameter_types, return_type) = self.get_method_reference(method_idx)?;

        let implementation = if code_off != 0 {
            self.parse_method_implementation(code_off)
        } else {
            None
        };

        Some(DexBackedMethod {
            defining_class,
            name,
            parameter_types,
            return_type,
            access_flags: AccessFlags::from_bits_truncate(access_flags),
            method_index: method_idx,
            annotations: Vec::new(),
            hidden_api_restrictions: 0,
            implementation,
        })
    }

    fn parse_method_implementation(&self, code_off: u32) -> Option<DexBackedMethodImplementation> {
        let code_item = parse_code_item(&self.data, code_off).ok()?;

        let instructions = decode_all_instructions(&code_item.insns);

        let try_blocks = code_item.try_blocks.iter().map(|tb| TryBlock {
            start_code_address: tb.start_addr,
            code_unit_count: tb.insn_count,
            handlers: Vec::new(),
        }).collect();

        Some(DexBackedMethodImplementation {
            register_count: code_item.registers_size,
            instructions,
            try_blocks,
            debug_items: Vec::new(),
        })
    }

    pub fn resolve_reference(&self, index: u32, opcode: Opcode) -> ReferenceHolder {
        match opcode {
            Opcode::ConstString | Opcode::ConstStringJumbo => {
                if let Some(s) = self.get_string(index) {
                    ReferenceHolder::String(StringReference::new(s))
                } else {
                    ReferenceHolder::String(StringReference::new(""))
                }
            }
            Opcode::ConstClass | Opcode::CheckCast | Opcode::NewInstance
            | Opcode::FilledNewArray | Opcode::FilledNewArrayRange => {
                if let Some(t) = self.get_type_descriptor(index) {
                    ReferenceHolder::Type(TypeReference::new(t))
                } else {
                    ReferenceHolder::Type(TypeReference::new("Ljava/lang/Object;"))
                }
            }
            Opcode::Sget | Opcode::SgetWide | Opcode::SgetObject | Opcode::SgetBoolean
            | Opcode::SgetByte | Opcode::SgetChar | Opcode::SgetShort
            | Opcode::Sput | Opcode::SputWide | Opcode::SputObject | Opcode::SputBoolean
            | Opcode::SputByte | Opcode::SputChar | Opcode::SputShort
            | Opcode::Iget | Opcode::IgetWide | Opcode::IgetObject | Opcode::IgetBoolean
            | Opcode::IgetByte | Opcode::IgetChar | Opcode::IgetShort
            | Opcode::Iput | Opcode::IputWide | Opcode::IputObject | Opcode::IputBoolean
            | Opcode::IputByte | Opcode::IputChar | Opcode::IputShort => {
                if let Some((defining_class, name, field_type)) = self.get_field_reference(index) {
                    ReferenceHolder::Field(FieldReference::new(defining_class, name, field_type))
                } else {
                    ReferenceHolder::Field(FieldReference::new("", "", ""))
                }
            }
            _ => {
                if let Some((defining_class, name, parameter_types, return_type)) = self.get_method_reference(index) {
                    ReferenceHolder::Method(MethodReference::new(
                        defining_class, name, parameter_types, return_type,
                    ))
                } else {
                    ReferenceHolder::Method(MethodReference::new("", "", vec![], ""))
                }
            }
        }
    }

    pub fn convert_instruction_to_immutable(&self, instr: &dyn Instruction) -> Box<dyn Instruction> {
        use crate::immutable::*;
        let db_instr = match instr.as_any().downcast_ref::<DexBackedInstruction>() {
            Some(d) => d,
            None => return instr.clone_boxed(),
        };
        let opcode = db_instr.opcode();
        let format = opcode.format();

        match format {
            InstructionFormat::Format10x => Box::new(ImmutableInstruction10x { opcode }),
            InstructionFormat::Format12x => Box::new(ImmutableInstruction12x {
                opcode,
                register_a: db_instr.register_a_byte(),
                register_b: db_instr.register_b_byte(),
            }),
            InstructionFormat::Format11n => Box::new(ImmutableInstruction11n {
                opcode,
                register_a: db_instr.register_a_byte(),
                literal: db_instr.literal_nibble(),
            }),
            InstructionFormat::Format11x => Box::new(ImmutableInstruction11x {
                opcode,
                register_a: db_instr.register_a_short(),
            }),
            InstructionFormat::Format10t => Box::new(ImmutableInstruction10t {
                opcode,
                code_offset: db_instr.code_offset_8(),
            }),
            InstructionFormat::Format20t => Box::new(ImmutableInstruction20t {
                opcode,
                code_offset: db_instr.code_offset_32(),
            }),
            InstructionFormat::Format30t => Box::new(ImmutableInstruction30t {
                opcode,
                code_offset: db_instr.code_offset_32(),
            }),
            InstructionFormat::Format21c => Box::new(ImmutableInstruction21c {
                    opcode,
                    register_a: db_instr.register_a_byte(),
                    reference: self.resolve_reference(db_instr.reference_index_16() as u32, opcode),
                }),
                InstructionFormat::Format21ih | InstructionFormat::Format21lh => Box::new(ImmutableInstruction21s {
                    opcode,
                    register_a: db_instr.register_a_byte(),
                    literal: db_instr.literal_16(),
                }),
            InstructionFormat::Format31c => Box::new(ImmutableInstruction31c {
                opcode,
                register_a: db_instr.register_a_byte(),
                reference: self.resolve_reference(db_instr.reference_index_32(), opcode),
            }),
            InstructionFormat::Format22c | InstructionFormat::Format22cs => Box::new(ImmutableInstruction22c {
                opcode,
                register_a: db_instr.register_b_nibble(),
                register_b: db_instr.register_c_nibble(),
                reference: self.resolve_reference(db_instr.reference_index_16() as u32, opcode),
            }),
            InstructionFormat::Format35c | InstructionFormat::Format35mi | InstructionFormat::Format35ms
            | InstructionFormat::Format45cc => Box::new(ImmutableInstruction35c {
                opcode,
                register_count: db_instr.invoke_register_count(),
                register_c: db_instr.invoke_register_c(),
                register_d: db_instr.invoke_register_d(),
                register_e: db_instr.invoke_register_e(),
                register_f: db_instr.invoke_register_f(),
                register_g: db_instr.invoke_register_g(),
                reference: self.resolve_reference(db_instr.reference_index_16() as u32, opcode),
            }),
            InstructionFormat::Format3rc | InstructionFormat::Format3rmi | InstructionFormat::Format3rms
            | InstructionFormat::Format4rcc => Box::new(ImmutableInstruction3rc {
                opcode,
                register_count: db_instr.invoke_range_count(),
                start_register: db_instr.invoke_range_start(),
                reference: self.resolve_reference(db_instr.reference_index_16() as u32, opcode),
            }),
            InstructionFormat::Format21s => Box::new(ImmutableInstruction21s {
                opcode,
                register_a: db_instr.register_a_byte(),
                literal: db_instr.literal_16(),
            }),
            InstructionFormat::Format31i => Box::new(ImmutableInstruction31i {
                opcode,
                register_a: db_instr.register_a_byte(),
                literal: db_instr.literal_32(),
            }),
            InstructionFormat::Format51l => Box::new(ImmutableInstruction51l {
                opcode,
                register_a: db_instr.register_a_byte(),
                literal: db_instr.literal_64(),
            }),
            InstructionFormat::Format21t => Box::new(ImmutableInstruction21t {
                opcode,
                register_a: db_instr.register_a_byte(),
                code_offset: db_instr.code_offset_16(),
            }),
            InstructionFormat::Format22t => Box::new(ImmutableInstruction22t {
                opcode,
                register_a: db_instr.register_a_byte(),
                register_b: db_instr.register_b_byte(),
                code_offset: db_instr.code_offset_16(),
            }),
            InstructionFormat::Format22b => Box::new(ImmutableInstruction22b {
                opcode,
                register_a: db_instr.register_a_byte(),
                register_b: db_instr.register_b_byte(),
                literal: db_instr.literal_byte(),
            }),
            InstructionFormat::Format22s => Box::new(ImmutableInstruction22s {
                opcode,
                register_a: db_instr.register_a_byte(),
                register_b: db_instr.register_b_byte(),
                literal: db_instr.literal_16(),
            }),
            InstructionFormat::Format22x => Box::new(ImmutableInstruction22x {
                opcode,
                register_a: db_instr.register_a_22x(),
                register_b: db_instr.register_b_22x(),
            }),
            InstructionFormat::Format23x => Box::new(ImmutableInstruction23x {
                opcode,
                register_a: db_instr.register_a_byte(),
                register_b: db_instr.register_b_byte(),
                register_c: db_instr.register_c_byte(),
            }),
            InstructionFormat::Format31t => Box::new(ImmutableInstruction31t {
                opcode,
                register_a: db_instr.register_a_byte(),
                code_offset: db_instr.code_offset_32(),
            }),
            InstructionFormat::Format32x => Box::new(ImmutableInstruction32x {
                opcode,
                register_a: db_instr.register_a_32x(),
                register_b: db_instr.register_b_32x(),
            }),
            _ => instr.clone_boxed(),
        }
    }
}

impl DexFile for DexBackedDexFile {
    fn classes(&self) -> &[Box<dyn ClassDef>] {
        unimplemented!("Use parse_classes() instead")
    }
    fn opcodes(&self) -> &Opcode {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dex_file_too_small() {
        let result = DexBackedDexFile::from_bytes(vec![0; 10]);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_magic() {
        let mut data = vec![0; 0x70];
        data[0..4].copy_from_slice(b"test");
        let result = DexBackedDexFile::from_bytes(data);
        assert!(result.is_err());
    }
}
