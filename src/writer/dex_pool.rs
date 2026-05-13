use crate::base::opcode::InstructionFormat;
use crate::iface::class_def::ClassDef;
use crate::iface::instruction::*;
use crate::iface::reference::ReferenceHolder;
use crate::immutable::immutable_instruction::*;
use crate::writer::dex_writer::DexWriter;
use crate::writer::memory_data_store::{DataStore, MemoryDataStore};
use crate::writer::pools::*;

pub struct DexPool {
    pub string_pool: StringPool,
    pub type_pool: TypePool,
    pub proto_pool: ProtoPool,
    pub field_pool: FieldPool,
    pub method_pool: MethodPool,
    pub class_pool: ClassPool,
}

impl DexPool {
    pub fn new() -> Self {
        Self {
            string_pool: StringPool::new(),
            type_pool: TypePool::new(StringPool::new()),
            proto_pool: ProtoPool::new(),
            field_pool: FieldPool::new(),
            method_pool: MethodPool::new(),
            class_pool: ClassPool::new(),
        }
    }

    pub fn intern_class(&mut self, class_def: &dyn ClassDef) {
        self.string_pool.intern(class_def.type_descriptor());
        self.type_pool.intern(class_def.type_descriptor());

        if let Some(superclass) = class_def.superclass() {
            self.string_pool.intern(superclass);
            self.type_pool.intern(superclass);
        }

        for iface in class_def.interfaces() {
            self.string_pool.intern(iface);
            self.type_pool.intern(iface);
        }

        if let Some(source_file) = class_def.source_file() {
            self.string_pool.intern(source_file);
        }

        for field in class_def.static_fields().iter().chain(class_def.instance_fields().iter()) {
            self.string_pool.intern(field.name());
            self.string_pool.intern(field.field_type());
            self.type_pool.intern(field.field_type());
            self.field_pool.intern(class_def.type_descriptor(), field.name(), field.field_type());
        }

        for method in class_def.direct_methods().iter().chain(class_def.virtual_methods().iter()) {
            self.string_pool.intern(method.name());
            self.string_pool.intern(method.return_type());
            self.type_pool.intern(method.return_type());

            let mut shorty = String::new();
            shorty.push(shorty_char(method.return_type()));
            let mut params = Vec::new();
            for p in method.parameter_types() {
                self.string_pool.intern(p);
                self.type_pool.intern(p);
                shorty.push(shorty_char(p));
                params.push(p.clone());
            }

            let proto_idx = self.proto_pool.intern(&shorty, method.return_type(), params.clone());
            let proto_key = self.proto_pool.get(proto_idx).unwrap().clone();
            self.method_pool.intern(class_def.type_descriptor(), method.name(), proto_key);

            if let Some(imp) = method.implementation() {
                for instr in imp.instructions() {
                    self.intern_instruction_references(instr.as_ref());
                }
            }
        }

        let mut class_entry = ClassEntry {
            type_descriptor: class_def.type_descriptor().to_string(),
            access_flags: class_def.access_flags().bits(),
            superclass_idx: class_def.superclass().and_then(|sc| self.type_pool.get_index(sc)),
            interfaces: class_def.interfaces().to_vec(),
            source_file: class_def.source_file().map(|s| s.to_string()),
            static_fields: Vec::new(),
            instance_fields: Vec::new(),
            direct_methods: Vec::new(),
            virtual_methods: Vec::new(),
        };

        for field in class_def.static_fields() {
            class_entry.static_fields.push(FieldEntry {
                name: field.name().to_string(),
                field_type: field.field_type().to_string(),
                access_flags: field.access_flags().bits(),
            });
        }

        for field in class_def.instance_fields() {
            class_entry.instance_fields.push(FieldEntry {
                name: field.name().to_string(),
                field_type: field.field_type().to_string(),
                access_flags: field.access_flags().bits(),
            });
        }

        for method in class_def.direct_methods() {
            let mut shorty = String::new();
            shorty.push(shorty_char(method.return_type()));
            for p in method.parameter_types() {
                shorty.push(shorty_char(p));
            }
            let params = method.parameter_types().to_vec();
            let proto_idx = self.proto_pool.intern(&shorty, method.return_type(), params);
            let proto_key = self.proto_pool.get(proto_idx).unwrap().clone();
            let impl_entry = method.implementation().map(|imp| {
                let mut insns: Vec<Box<dyn crate::iface::instruction::Instruction>> = Vec::new();
                for instr in imp.instructions().iter() {
                    insns.push(instr.clone_boxed());
                }
                MethodImplEntry {
                    register_count: imp.register_count(),
                    instructions: insns,
                    debug_info_off: 0,
                }
            });
            class_entry.direct_methods.push(MethodEntry {
                name: method.name().to_string(),
                proto: proto_key,
                access_flags: method.access_flags().bits(),
                implementation: impl_entry,
            });
        }

        for method in class_def.virtual_methods() {
            let mut shorty = String::new();
            shorty.push(shorty_char(method.return_type()));
            for p in method.parameter_types() {
                shorty.push(shorty_char(p));
            }
            let params = method.parameter_types().to_vec();
            let proto_idx = self.proto_pool.intern(&shorty, method.return_type(), params);
            let proto_key = self.proto_pool.get(proto_idx).unwrap().clone();
            let impl_entry = method.implementation().map(|imp| {
                let mut insns: Vec<Box<dyn crate::iface::instruction::Instruction>> = Vec::new();
                for instr in imp.instructions().iter() {
                    insns.push(instr.clone_boxed());
                }
                MethodImplEntry {
                    register_count: imp.register_count(),
                    instructions: insns,
                    debug_info_off: 0,
                }
            });
            class_entry.virtual_methods.push(MethodEntry {
                name: method.name().to_string(),
                proto: proto_key,
                access_flags: method.access_flags().bits(),
                implementation: impl_entry,
            });
        }

        self.class_pool.intern(class_entry);
    }

    fn intern_instruction_references(&mut self, instr: &dyn Instruction) {
        let format = instr.format();
        match format {
            InstructionFormat::Format21c | InstructionFormat::Format31c => {
                if let Some(i) = instr.as_any().downcast_ref::<ImmutableInstruction21c>() {
                    self.intern_reference(&i.reference);
                } else if let Some(i) = instr.as_any().downcast_ref::<ImmutableInstruction31c>() {
                    self.intern_reference(&i.reference);
                }
            }
            InstructionFormat::Format22c => {
                if let Some(i) = instr.as_any().downcast_ref::<ImmutableInstruction22c>() {
                    self.intern_reference(&i.reference);
                }
            }
            InstructionFormat::Format35c => {
                if let Some(i) = instr.as_any().downcast_ref::<ImmutableInstruction35c>() {
                    self.intern_reference(&i.reference);
                }
            }
            InstructionFormat::Format3rc => {
                if let Some(i) = instr.as_any().downcast_ref::<ImmutableInstruction3rc>() {
                    self.intern_reference(&i.reference);
                }
            }
            _ => {}
        }
    }

    fn intern_reference(&mut self, reference: &ReferenceHolder) {
        match reference {
            ReferenceHolder::None => {}
            ReferenceHolder::String(s) => {
                self.string_pool.intern(s.string());
            }
            ReferenceHolder::Type(t) => {
                self.string_pool.intern(t.type_descriptor());
                self.type_pool.intern(t.type_descriptor());
            }
            ReferenceHolder::Field(f) => {
                self.string_pool.intern(f.defining_class());
                self.type_pool.intern(f.defining_class());
                self.string_pool.intern(&f.field_name);
                self.string_pool.intern(f.field_type());
                self.type_pool.intern(f.field_type());
                self.field_pool.intern(f.defining_class(), &f.field_name, f.field_type());
            }
            ReferenceHolder::Method(m) => {
                self.string_pool.intern(m.defining_class());
                self.type_pool.intern(m.defining_class());
                self.string_pool.intern(&m.method_name);
                self.string_pool.intern(m.return_type());
                self.type_pool.intern(m.return_type());
                let mut shorty = String::new();
                shorty.push(shorty_char(m.return_type()));
                let mut params = Vec::new();
                for p in m.parameter_types() {
                    self.string_pool.intern(p);
                    self.type_pool.intern(p);
                    shorty.push(shorty_char(p));
                    params.push(p.clone());
                }
                let proto_idx = self.proto_pool.intern(&shorty, m.return_type(), params);
                let proto_key = self.proto_pool.get(proto_idx).unwrap().clone();
                self.method_pool.intern(m.defining_class(), &m.method_name, proto_key);
            }
            ReferenceHolder::MethodProto(p) => {
                self.string_pool.intern(p.return_type());
                self.type_pool.intern(p.return_type());
                let mut shorty = String::new();
                shorty.push(shorty_char(p.return_type()));
                for t in &p.parameter_types {
                    self.string_pool.intern(t);
                    self.type_pool.intern(t);
                    shorty.push(shorty_char(t));
                }
                self.proto_pool.intern(&shorty, &p.return_type, p.parameter_types.clone());
            }
        }
    }

    pub fn write_to(&self, data_store: &mut dyn DataStore) -> Result<u32, String> {
        let mut writer = DexWriter::new(data_store);

        let header_size: u32 = 0x70;
        let string_ids_size = self.string_pool.count() * 4;
        let type_ids_size = self.type_pool.count() * 4;
        let proto_ids_size = self.proto_pool.count() * 12;
        let field_ids_size = self.field_pool.count() * 8;
        let method_ids_size = self.method_pool.count() * 8;
        let class_defs_size = self.class_pool.count() * 32;

        let string_ids_off = header_size;
        let type_ids_off = string_ids_off + string_ids_size;
        let proto_ids_off = type_ids_off + type_ids_size;
        let field_ids_off = proto_ids_off + proto_ids_size;
        let method_ids_off = field_ids_off + field_ids_size;
        let class_defs_off = method_ids_off + method_ids_size;
        let data_start = class_defs_off + class_defs_size;

        writer.set_offset(data_start);

        let mut string_data_off_vec: Vec<u32> = Vec::new();
        for i in 0..self.string_pool.count() {
            let s = self.string_pool.get(i).unwrap();
            let off = writer.write_string_data(s);
            string_data_off_vec.push(off);
        }

        let mut proto_params_offsets: Vec<Option<u32>> = vec![None; self.proto_pool.count() as usize];
        for i in 0..self.proto_pool.count() {
            let proto = self.proto_pool.get(i).unwrap();
            if !proto.parameters.is_empty() {
                writer.align_to(4);
                proto_params_offsets[i as usize] = Some(writer.current_offset());
                writer.write_u32(proto.parameters.len() as u32);
                for param in &proto.parameters {
                    let type_idx = self.type_pool.get_index(param).unwrap_or(0) as u16;
                    writer.write_u16(type_idx);
                }
            }
        }

        let mut type_list_offsets: Vec<Option<u32>> = vec![None; self.class_pool.count() as usize];
        for ci in 0..self.class_pool.count() {
            let class = self.class_pool.get(ci).unwrap();
            if !class.interfaces.is_empty() {
                writer.align_to(4);
                type_list_offsets[ci as usize] = Some(writer.current_offset());
                writer.write_u32(class.interfaces.len() as u32);
                for iface in &class.interfaces {
                    let type_idx = self.type_pool.get_index(iface).unwrap_or(0) as u16;
                    writer.write_u16(type_idx);
                }
            }
        }

        let mut code_item_offsets: Vec<Vec<Option<u32>>> = Vec::new();
        for ci in 0..self.class_pool.count() {
            let class = self.class_pool.get(ci).unwrap();
            let mut method_code_offsets: Vec<Option<u32>> = Vec::new();

            for method in class.direct_methods.iter().chain(class.virtual_methods.iter()) {
                if let Some(ref impl_entry) = method.implementation {
                    writer.align_to(4);
                    let code_off = writer.current_offset();
                    method_code_offsets.push(Some(code_off));

                    let encoded = encode_instructions(
                        &impl_entry.instructions,
                        &self.string_pool,
                        &self.type_pool,
                        &self.field_pool,
                        &self.method_pool,
                        &self.proto_pool,
                    );

                    writer.write_u16(impl_entry.register_count);
                    writer.write_u16(0);
                    writer.write_u16(0);
                    writer.write_u16(0);
                    writer.write_u32(0);
                    writer.write_u32(encoded.len() as u32);
                    for unit in &encoded {
                        writer.write_u16(*unit);
                    }
                    if encoded.len() % 2 == 1 {
                        writer.write_u16(0);
                    }
                } else {
                    method_code_offsets.push(None);
                }
            }

            code_item_offsets.push(method_code_offsets);
        }

        let mut class_data_offsets: Vec<Option<u32>> = vec![None; self.class_pool.count() as usize];
        for ci in 0..self.class_pool.count() {
            let class = self.class_pool.get(ci).unwrap();
            let has_fields = !class.static_fields.is_empty() || !class.instance_fields.is_empty();
            let has_methods = !class.direct_methods.is_empty() || !class.virtual_methods.is_empty();
            if !has_fields && !has_methods {
                continue;
            }

            class_data_offsets[ci as usize] = Some(writer.current_offset());

            let mut static_field_entries: Vec<(u32, u32)> = Vec::new();
            for field in &class.static_fields {
                if let Some(idx) = self.field_pool.get_index(
                    &class.type_descriptor,
                    &field.name,
                    &field.field_type,
                ) {
                    static_field_entries.push((idx, field.access_flags));
                }
            }
            static_field_entries.sort_by_key(|e| e.0);

            let mut instance_field_entries: Vec<(u32, u32)> = Vec::new();
            for field in &class.instance_fields {
                if let Some(idx) = self.field_pool.get_index(
                    &class.type_descriptor,
                    &field.name,
                    &field.field_type,
                ) {
                    instance_field_entries.push((idx, field.access_flags));
                }
            }
            instance_field_entries.sort_by_key(|e| e.0);

            let mut direct_method_entries: Vec<(u32, u32, u32)> = Vec::new();
            let mut code_offset_idx = 0;
            for method in &class.direct_methods {
                if let Some(idx) = self.method_pool.get_index(
                    &class.type_descriptor,
                    &method.name,
                    &method.proto,
                ) {
                    let code_off = code_item_offsets[ci as usize][code_offset_idx].unwrap_or(0);
                    direct_method_entries.push((idx, method.access_flags, code_off));
                }
                code_offset_idx += 1;
            }
            direct_method_entries.sort_by_key(|e| e.0);

            let mut virtual_method_entries: Vec<(u32, u32, u32)> = Vec::new();
            for method in &class.virtual_methods {
                if let Some(idx) = self.method_pool.get_index(
                    &class.type_descriptor,
                    &method.name,
                    &method.proto,
                ) {
                    let code_off = code_item_offsets[ci as usize][code_offset_idx].unwrap_or(0);
                    virtual_method_entries.push((idx, method.access_flags, code_off));
                }
                code_offset_idx += 1;
            }
            virtual_method_entries.sort_by_key(|e| e.0);

            writer.write_uleb128(static_field_entries.len() as u32);
            writer.write_uleb128(instance_field_entries.len() as u32);
            writer.write_uleb128(direct_method_entries.len() as u32);
            writer.write_uleb128(virtual_method_entries.len() as u32);

            let mut field_idx: u32 = 0;
            for (new_idx, access_flags) in &static_field_entries {
                writer.write_uleb128(*new_idx - field_idx);
                writer.write_uleb128(*access_flags);
                field_idx = *new_idx;
            }

            field_idx = 0;
            for (new_idx, access_flags) in &instance_field_entries {
                writer.write_uleb128(*new_idx - field_idx);
                writer.write_uleb128(*access_flags);
                field_idx = *new_idx;
            }

            let mut method_idx: u32 = 0;
            for (new_idx, access_flags, code_off) in &direct_method_entries {
                writer.write_uleb128(*new_idx - method_idx);
                writer.write_uleb128(*access_flags);
                writer.write_uleb128(*code_off);
                method_idx = *new_idx;
            }

            method_idx = 0;
            for (new_idx, access_flags, code_off) in &virtual_method_entries {
                writer.write_uleb128(*new_idx - method_idx);
                writer.write_uleb128(*access_flags);
                writer.write_uleb128(*code_off);
                method_idx = *new_idx;
            }
        }

        writer.set_offset(string_ids_off);
        for off in &string_data_off_vec {
            writer.write_u32(*off);
        }

        for i in 0..self.type_pool.count() {
            let type_descriptor = self.type_pool.get(i).unwrap();
            let string_idx = self.string_pool.get_index(type_descriptor).unwrap_or(0);
            writer.write_u32(string_idx);
        }

        for i in 0..self.proto_pool.count() {
            let proto = self.proto_pool.get(i).unwrap();
            let shorty_idx = self.string_pool.get_index(&proto.shorty).unwrap_or(0);
            let return_type_idx = self.type_pool.get_index(&proto.return_type).unwrap_or(0);
            let parameters_off = proto_params_offsets[i as usize].unwrap_or(0);
            writer.write_u32(shorty_idx);
            writer.write_u32(return_type_idx);
            writer.write_u32(parameters_off);
        }

        for i in 0..self.field_pool.count() {
            let field = self.field_pool.get(i).unwrap();
            let class_idx = self.type_pool.get_index(&field.defining_class).unwrap_or(0) as u16;
            let type_idx = self.type_pool.get_index(&field.field_type).unwrap_or(0) as u16;
            let name_idx = self.string_pool.get_index(&field.name).unwrap_or(0);
            writer.write_u16(class_idx);
            writer.write_u16(type_idx);
            writer.write_u32(name_idx);
        }

        for i in 0..self.method_pool.count() {
            let method = self.method_pool.get(i).unwrap();
            let class_idx = self.type_pool.get_index(&method.defining_class).unwrap_or(0) as u16;
            let proto_idx = self.proto_pool.get_index(
                &method.proto.shorty,
                &method.proto.return_type,
                &method.proto.parameters,
            ).unwrap_or(0) as u16;
            let name_idx = self.string_pool.get_index(&method.name).unwrap_or(0);
            writer.write_u16(class_idx);
            writer.write_u16(proto_idx);
            writer.write_u32(name_idx);
        }

        for ci in 0..self.class_pool.count() {
            let class = self.class_pool.get(ci).unwrap();
            let class_idx = self.type_pool.get_index(&class.type_descriptor).unwrap_or(0);
            let superclass_idx = class.superclass_idx.unwrap_or(0xFFFFFFFF);
            let interfaces_off = type_list_offsets[ci as usize].unwrap_or(0);
            let source_file_idx = class.source_file.as_ref()
                .and_then(|s| self.string_pool.get_index(s))
                .unwrap_or(0xFFFFFFFF);
            let annotations_off: u32 = 0;
            let class_data_off = class_data_offsets[ci as usize].unwrap_or(0);
            let static_values_off: u32 = 0;
            writer.write_u32(class_idx);
            writer.write_u32(class.access_flags);
            writer.write_u32(superclass_idx);
            writer.write_u32(interfaces_off);
            writer.write_u32(source_file_idx);
            writer.write_u32(annotations_off);
            writer.write_u32(class_data_off);
            writer.write_u32(static_values_off);
        }

        let file_size = writer.current_offset();
        let data_size = file_size - data_start;

        writer.set_offset(0);
        writer.write_bytes(b"dex\n035\0");
        writer.write_u32(0);
        writer.write_bytes(&[0u8; 20]);
        writer.write_u32(file_size);
        writer.write_u32(header_size);
        writer.write_u32(0x12345678);
        writer.write_u32(0);
        writer.write_u32(0);
        writer.write_u32(0);
        writer.write_u32(self.string_pool.count());
        writer.write_u32(string_ids_off);
        writer.write_u32(self.type_pool.count());
        writer.write_u32(type_ids_off);
        writer.write_u32(self.proto_pool.count());
        writer.write_u32(proto_ids_off);
        writer.write_u32(self.field_pool.count());
        writer.write_u32(field_ids_off);
        writer.write_u32(self.method_pool.count());
        writer.write_u32(method_ids_off);
        writer.write_u32(self.class_pool.count());
        writer.write_u32(class_defs_off);
        writer.write_u32(data_size);
        writer.write_u32(data_start);

        Ok(file_size)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let mut store = MemoryDataStore::with_capacity(4096);
        self.write_to(&mut store)?;
        Ok(store.get_buffer().to_vec())
    }
}

impl Default for DexPool {
    fn default() -> Self { Self::new() }
}

fn shorty_char(type_descriptor: &str) -> char {
    match type_descriptor.chars().next() {
        Some('V') => 'V',
        Some('Z') => 'Z',
        Some('B') => 'B',
        Some('S') => 'S',
        Some('C') => 'C',
        Some('I') => 'I',
        Some('J') => 'J',
        Some('F') => 'F',
        Some('D') => 'D',
        Some('L') | Some('[') => 'L',
        _ => 'L',
    }
}

fn resolve_reference_index(
    reference: &ReferenceHolder,
    string_pool: &StringPool,
    type_pool: &TypePool,
    field_pool: &FieldPool,
    method_pool: &MethodPool,
    proto_pool: &ProtoPool,
) -> u16 {
    match reference {
        ReferenceHolder::None => 0,
        ReferenceHolder::String(s) => {
            string_pool.get_index(s.string()).unwrap_or(0) as u16
        }
        ReferenceHolder::Type(t) => {
            type_pool.get_index(t.type_descriptor()).unwrap_or(0) as u16
        }
        ReferenceHolder::Field(f) => {
            field_pool.get_index(f.defining_class(), f.field_name(), f.field_type()).unwrap_or(0) as u16
        }
        ReferenceHolder::Method(m) => {
            let mut shorty = String::new();
            shorty.push(shorty_char(m.return_type()));
            for p in m.parameter_types() {
                shorty.push(shorty_char(p));
            }
            let proto_key = ProtoKey {
                shorty,
                return_type: m.return_type().to_string(),
                parameters: m.parameter_types().to_vec(),
            };
            method_pool.get_index(m.defining_class(), m.method_name(), &proto_key).unwrap_or(0) as u16
        }
        ReferenceHolder::MethodProto(p) => {
            let mut shorty = String::new();
            shorty.push(shorty_char(p.return_type()));
            for pt in p.parameter_types() {
                shorty.push(shorty_char(pt));
            }
            proto_pool.get_index(&shorty, p.return_type(), p.parameter_types()).unwrap_or(0) as u16
        }
    }
}

fn encode_instruction(
    instr: &dyn Instruction,
    string_pool: &StringPool,
    type_pool: &TypePool,
    field_pool: &FieldPool,
    method_pool: &MethodPool,
    proto_pool: &ProtoPool,
) -> Vec<u16> {
    let opcode = instr.opcode() as u16;
    match instr.format() {
        InstructionFormat::Format10x => vec![opcode],
        InstructionFormat::Format12x => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction12x>().unwrap();
            vec![opcode | ((i.register_a as u16) << 8) | ((i.register_b as u16) << 12)]
        }
        InstructionFormat::Format11n => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction11n>().unwrap();
            vec![opcode | ((i.register_a as u16 & 0xF) << 8) | (((i.literal as u16) & 0xF) << 12)]
        }
        InstructionFormat::Format11x => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction11x>().unwrap();
            vec![opcode | (i.register_a << 8)]
        }
        InstructionFormat::Format10t => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction10t>().unwrap();
            vec![opcode | (((i.code_offset as u16) & 0xFF) << 8)]
        }
        InstructionFormat::Format21c => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction21c>().unwrap();
            let idx = resolve_reference_index(i.reference(), string_pool, type_pool, field_pool, method_pool, proto_pool);
            vec![opcode | ((i.register_a as u16) << 8), idx]
        }
        InstructionFormat::Format31c => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction31c>().unwrap();
            let idx = resolve_reference_index(i.reference(), string_pool, type_pool, field_pool, method_pool, proto_pool) as u32;
            vec![opcode | ((i.register_a as u16) << 8), idx as u16, (idx >> 16) as u16]
        }
        InstructionFormat::Format22c => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction22c>().unwrap();
            let idx = resolve_reference_index(i.reference(), string_pool, type_pool, field_pool, method_pool, proto_pool);
            vec![opcode | ((i.register_a as u16 & 0xF) << 8) | ((i.register_b as u16 & 0xF) << 12), idx]
        }
        InstructionFormat::Format35c => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction35c>().unwrap();
            let idx = resolve_reference_index(i.reference(), string_pool, type_pool, field_pool, method_pool, proto_pool);
            vec![
                opcode | ((i.register_count as u16 & 0xF) << 12) | ((i.register_g as u16 & 0xF) << 8),
                idx,
                ((i.register_f as u16 & 0xF) << 12) | ((i.register_e as u16 & 0xF) << 8) | ((i.register_d as u16 & 0xF) << 4) | (i.register_c as u16 & 0xF),
            ]
        }
        InstructionFormat::Format3rc => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction3rc>().unwrap();
            let idx = resolve_reference_index(i.reference(), string_pool, type_pool, field_pool, method_pool, proto_pool);
            vec![opcode | ((i.register_count as u16) << 8), idx, i.start_register]
        }
        InstructionFormat::Format21s => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction21s>().unwrap();
            vec![opcode | ((i.register_a as u16) << 8), i.literal as u16]
        }
        InstructionFormat::Format31i => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction31i>().unwrap();
            let lit = i.literal as u32;
            vec![opcode | ((i.register_a as u16) << 8), lit as u16, (lit >> 16) as u16]
        }
        InstructionFormat::Format51l => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction51l>().unwrap();
            let lit = i.literal as u64;
            vec![
                opcode | ((i.register_a as u16) << 8),
                lit as u16,
                (lit >> 16) as u16,
                (lit >> 32) as u16,
                (lit >> 48) as u16,
            ]
        }
        InstructionFormat::Format21t => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction21t>().unwrap();
            vec![opcode | ((i.register_a as u16) << 8), i.code_offset as u16]
        }
        InstructionFormat::Format22t => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction22t>().unwrap();
            vec![opcode | ((i.register_a as u16 & 0xF) << 8) | ((i.register_b as u16 & 0xF) << 12), i.code_offset as u16]
        }
        InstructionFormat::Format22b => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction22b>().unwrap();
            vec![opcode | ((i.register_a as u16) << 8), (i.literal as u16 & 0xFF) | ((i.register_b as u16) << 8)]
        }
        InstructionFormat::Format22s => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction22s>().unwrap();
            vec![opcode | ((i.register_a as u16 & 0xF) << 8) | ((i.register_b as u16 & 0xF) << 12), i.literal as u16]
        }
        InstructionFormat::Format22x => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction22x>().unwrap();
            vec![opcode | ((i.register_a as u16) << 8), i.register_b]
        }
        InstructionFormat::Format23x => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction23x>().unwrap();
            vec![opcode | ((i.register_a as u16) << 8), (i.register_c as u16) | ((i.register_b as u16) << 8)]
        }
        InstructionFormat::Format20t => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction20t>().unwrap();
            vec![opcode, i.code_offset as u16]
        }
        InstructionFormat::Format30t => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction30t>().unwrap();
            let off = i.code_offset as u32;
            vec![opcode, off as u16, (off >> 16) as u16]
        }
        InstructionFormat::Format31t => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction31t>().unwrap();
            let off = i.code_offset as u32;
            vec![opcode | ((i.register_a as u16) << 8), off as u16, (off >> 16) as u16]
        }
        InstructionFormat::Format32x => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction32x>().unwrap();
            vec![opcode, i.register_a, i.register_b]
        }
        InstructionFormat::Format21ih | InstructionFormat::Format21lh => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction21s>().unwrap();
            vec![opcode | ((i.register_a as u16) << 8), i.literal as u16]
        }
        InstructionFormat::Format22cs => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction22c>().unwrap();
            vec![opcode | ((i.register_a as u16 & 0xF) << 8) | ((i.register_b as u16 & 0xF) << 12), 0]
        }
        InstructionFormat::Format35mi => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction35c>().unwrap();
            vec![
                opcode | ((i.register_count as u16 & 0xF) << 12) | ((i.register_g as u16 & 0xF) << 8),
                ((i.register_f as u16 & 0xF) << 12) | ((i.register_e as u16 & 0xF) << 8) | ((i.register_d as u16 & 0xF) << 4) | (i.register_c as u16 & 0xF),
                0,
            ]
        }
        InstructionFormat::Format35ms => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction35c>().unwrap();
            vec![
                opcode | ((i.register_count as u16 & 0xF) << 12) | ((i.register_g as u16 & 0xF) << 8),
                ((i.register_f as u16 & 0xF) << 12) | ((i.register_e as u16 & 0xF) << 8) | ((i.register_d as u16 & 0xF) << 4) | (i.register_c as u16 & 0xF),
                0,
            ]
        }
        InstructionFormat::Format3rmi => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction3rc>().unwrap();
            vec![opcode | ((i.register_count as u16) << 8), 0, i.start_register]
        }
        InstructionFormat::Format3rms => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction3rc>().unwrap();
            vec![opcode | ((i.register_count as u16) << 8), 0, i.start_register]
        }
        InstructionFormat::Format45cc => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction35c>().unwrap();
            let idx = resolve_reference_index(i.reference(), string_pool, type_pool, field_pool, method_pool, proto_pool);
            vec![
                opcode | ((i.register_count as u16 & 0xF) << 12) | ((i.register_g as u16 & 0xF) << 8),
                ((i.register_f as u16 & 0xF) << 12) | ((i.register_e as u16 & 0xF) << 8) | ((i.register_d as u16 & 0xF) << 4) | (i.register_c as u16 & 0xF),
                idx,
                0,
            ]
        }
        InstructionFormat::Format4rcc => {
            let i = instr.as_any().downcast_ref::<ImmutableInstruction3rc>().unwrap();
            let idx = resolve_reference_index(i.reference(), string_pool, type_pool, field_pool, method_pool, proto_pool);
            vec![opcode | ((i.register_count as u16) << 8), idx, i.start_register, 0]
        }
        InstructionFormat::Format20bc => {
            vec![opcode, 0]
        }
        InstructionFormat::PackedSwitchPayload
        | InstructionFormat::SparseSwitchPayload
        | InstructionFormat::ArrayPayload => {
            if let Some(payload) = instr.as_any().downcast_ref::<ImmutablePayloadInstruction>() {
                payload.raw_data.clone()
            } else {
                vec![opcode]
            }
        }
    }
}

fn encode_instructions(
    instructions: &[Box<dyn Instruction>],
    string_pool: &StringPool,
    type_pool: &TypePool,
    field_pool: &FieldPool,
    method_pool: &MethodPool,
    proto_pool: &ProtoPool,
) -> Vec<u16> {
    let mut result = Vec::new();
    for instr in instructions.iter() {
        let encoded = encode_instruction(instr.as_ref(), string_pool, type_pool, field_pool, method_pool, proto_pool);
        result.extend_from_slice(&encoded);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::access_flags::AccessFlags;
    use crate::immutable::immutable_class_def::ImmutableClassDef;

    #[test]
    fn test_dex_pool_new() {
        let pool = DexPool::new();
        assert_eq!(pool.string_pool.count(), 0);
        assert_eq!(pool.type_pool.count(), 0);
        assert_eq!(pool.class_pool.count(), 0);
    }

    #[test]
    fn test_dex_pool_intern_class() {
        let mut pool = DexPool::new();
        let class = ImmutableClassDef::new("Lcom/A;", AccessFlags::ACC_PUBLIC);
        pool.intern_class(&class);
        assert!(pool.string_pool.get_index("Lcom/A;").is_some());
        assert!(pool.type_pool.get_index("Lcom/A;").is_some());
        assert_eq!(pool.class_pool.count(), 1);
    }

    #[test]
    fn test_dex_pool_write_empty() {
        let pool = DexPool::new();
        let result = pool.to_bytes();
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(bytes.len() >= 0x70);
        assert_eq!(&bytes[0..4], b"dex\n");
    }

    #[test]
    fn test_shorty_char() {
        assert_eq!(shorty_char("V"), 'V');
        assert_eq!(shorty_char("I"), 'I');
        assert_eq!(shorty_char("Lcom/A;"), 'L');
        assert_eq!(shorty_char("[I"), 'L');
    }
}
