use crate::iface::instruction::Instruction;
use indexmap::IndexSet;
use std::collections::HashMap;

pub struct StringPool {
    strings: IndexSet<String>,
    sorted: bool,
    sorted_indices: Vec<usize>,
}

impl StringPool {
    pub fn new() -> Self {
        Self {
            strings: IndexSet::new(),
            sorted: false,
            sorted_indices: Vec::new(),
        }
    }

    pub fn intern(&mut self, s: &str) -> u32 {
        self.sorted = false;
        let (idx, _) = self.strings.insert_full(s.to_string());
        idx as u32
    }

    pub fn get_index(&self, s: &str) -> Option<u32> {
        self.strings.get_index_of(s).map(|i| i as u32)
    }

    pub fn count(&self) -> u32 {
        self.strings.len() as u32
    }

    pub fn get(&self, idx: u32) -> Option<&str> {
        self.strings.get_index(idx as usize).map(|s| s.as_str())
    }

    pub fn ensure_sorted(&mut self) {
        if self.sorted {
            return;
        }
        let mut indexed: Vec<(usize, &String)> = self.strings.iter().enumerate().collect();
        indexed.sort_by(|a, b| {
            let a_utf16: Vec<u16> = a.1.encode_utf16().collect();
            let b_utf16: Vec<u16> = b.1.encode_utf16().collect();
            a_utf16.cmp(&b_utf16)
        });
        self.sorted_indices = indexed.iter().map(|(i, _)| *i).collect();
        self.sorted = true;
    }

    pub fn sorted_index(&self, original_idx: u32) -> u32 {
        if let Some(pos) = self.sorted_indices.iter().position(|&i| i == original_idx as usize) {
            pos as u32
        } else {
            original_idx
        }
    }

    pub fn iter_sorted(&self) -> impl Iterator<Item = (u32, &str)> {
        self.sorted_indices.iter().enumerate().map(move |(sorted_idx, &orig_idx)| {
            (sorted_idx as u32, self.strings.get_index(orig_idx).unwrap().as_str())
        })
    }
}

impl Default for StringPool {
    fn default() -> Self { Self::new() }
}

pub struct TypePool {
    types: IndexSet<String>,
    string_pool: StringPool,
}

impl TypePool {
    pub fn new(string_pool: StringPool) -> Self {
        Self { types: IndexSet::new(), string_pool }
    }

    pub fn intern(&mut self, type_descriptor: &str) -> u32 {
        let (idx, _) = self.types.insert_full(type_descriptor.to_string());
        idx as u32
    }

    pub fn get_index(&self, type_descriptor: &str) -> Option<u32> {
        self.types.get_index_of(type_descriptor).map(|i| i as u32)
    }

    pub fn count(&self) -> u32 {
        self.types.len() as u32
    }

    pub fn get(&self, idx: u32) -> Option<&str> {
        self.types.get_index(idx as usize).map(|s| s.as_str())
    }

    pub fn string_pool(&self) -> &StringPool { &self.string_pool }
    pub fn string_pool_mut(&mut self) -> &mut StringPool { &mut self.string_pool }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProtoKey {
    pub shorty: String,
    pub return_type: String,
    pub parameters: Vec<String>,
}

pub struct ProtoPool {
    protos: IndexSet<ProtoKey>,
}

impl ProtoPool {
    pub fn new() -> Self {
        Self { protos: IndexSet::new() }
    }

    pub fn intern(&mut self, shorty: &str, return_type: &str, parameters: Vec<String>) -> u32 {
        let key = ProtoKey {
            shorty: shorty.to_string(),
            return_type: return_type.to_string(),
            parameters,
        };
        let (idx, _) = self.protos.insert_full(key);
        idx as u32
    }

    pub fn get_index(&self, shorty: &str, return_type: &str, parameters: &[String]) -> Option<u32> {
        self.protos.iter().position(|p| {
            p.shorty == shorty && p.return_type == return_type && p.parameters == parameters
        }).map(|i| i as u32)
    }

    pub fn count(&self) -> u32 {
        self.protos.len() as u32
    }

    pub fn get(&self, idx: u32) -> Option<&ProtoKey> {
        self.protos.get_index(idx as usize)
    }
}

impl Default for ProtoPool {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldKey {
    pub defining_class: String,
    pub name: String,
    pub field_type: String,
}

pub struct FieldPool {
    fields: IndexSet<FieldKey>,
}

impl FieldPool {
    pub fn new() -> Self {
        Self { fields: IndexSet::new() }
    }

    pub fn intern(&mut self, defining_class: &str, name: &str, field_type: &str) -> u32 {
        let key = FieldKey {
            defining_class: defining_class.to_string(),
            name: name.to_string(),
            field_type: field_type.to_string(),
        };
        let (idx, _) = self.fields.insert_full(key);
        idx as u32
    }

    pub fn get_index(&self, defining_class: &str, name: &str, field_type: &str) -> Option<u32> {
        self.fields.iter().position(|f| {
            f.defining_class == defining_class && f.name == name && f.field_type == field_type
        }).map(|i| i as u32)
    }

    pub fn count(&self) -> u32 {
        self.fields.len() as u32
    }

    pub fn get(&self, idx: u32) -> Option<&FieldKey> {
        self.fields.get_index(idx as usize)
    }
}

impl Default for FieldPool {
    fn default() -> Self { Self::new() }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodKey {
    pub defining_class: String,
    pub name: String,
    pub proto: ProtoKey,
}

pub struct MethodPool {
    methods: IndexSet<MethodKey>,
}

impl MethodPool {
    pub fn new() -> Self {
        Self { methods: IndexSet::new() }
    }

    pub fn intern(&mut self, defining_class: &str, name: &str, proto: ProtoKey) -> u32 {
        let key = MethodKey {
            defining_class: defining_class.to_string(),
            name: name.to_string(),
            proto,
        };
        let (idx, _) = self.methods.insert_full(key);
        idx as u32
    }

    pub fn get_index(&self, defining_class: &str, name: &str, proto: &ProtoKey) -> Option<u32> {
        self.methods.iter().position(|m| {
            m.defining_class == defining_class && m.name == name && m.proto == *proto
        }).map(|i| i as u32)
    }

    pub fn count(&self) -> u32 {
        self.methods.len() as u32
    }

    pub fn get(&self, idx: u32) -> Option<&MethodKey> {
        self.methods.get_index(idx as usize)
    }
}

impl Default for MethodPool {
    fn default() -> Self { Self::new() }
}

pub struct ClassPool {
    class_defs: Vec<ClassEntry>,
    class_indices: HashMap<String, u32>,
}

#[derive(Debug)]
pub struct ClassEntry {
    pub type_descriptor: String,
    pub access_flags: u32,
    pub superclass_idx: Option<u32>,
    pub interfaces: Vec<String>,
    pub source_file: Option<String>,
    pub static_fields: Vec<FieldEntry>,
    pub instance_fields: Vec<FieldEntry>,
    pub direct_methods: Vec<MethodEntry>,
    pub virtual_methods: Vec<MethodEntry>,
}

#[derive(Debug, Clone)]
pub struct FieldEntry {
    pub name: String,
    pub field_type: String,
    pub access_flags: u32,
}

#[derive(Debug)]
pub struct MethodEntry {
    pub name: String,
    pub proto: ProtoKey,
    pub access_flags: u32,
    pub implementation: Option<MethodImplEntry>,
}

#[derive(Debug)]
pub struct MethodImplEntry {
    pub register_count: u16,
    pub instructions: Vec<Box<dyn Instruction>>,
    pub debug_info_off: u32,
}

impl ClassPool {
    pub fn new() -> Self {
        Self {
            class_defs: Vec::new(),
            class_indices: HashMap::new(),
        }
    }

    pub fn intern(&mut self, entry: ClassEntry) -> u32 {
        let idx = self.class_defs.len() as u32;
        self.class_indices.insert(entry.type_descriptor.clone(), idx);
        self.class_defs.push(entry);
        idx
    }

    pub fn count(&self) -> u32 {
        self.class_defs.len() as u32
    }

    pub fn get(&self, idx: u32) -> Option<&ClassEntry> {
        self.class_defs.get(idx as usize)
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &ClassEntry)> {
        self.class_defs.iter().enumerate().map(|(i, e)| (i as u32, e))
    }
}

impl Default for ClassPool {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_pool_intern() {
        let mut pool = StringPool::new();
        let idx1 = pool.intern("hello");
        let idx2 = pool.intern("world");
        let idx3 = pool.intern("hello");
        assert_eq!(idx1, idx3);
        assert_ne!(idx1, idx2);
        assert_eq!(pool.count(), 2);
    }

    #[test]
    fn test_string_pool_sorted() {
        let mut pool = StringPool::new();
        pool.intern("banana");
        pool.intern("apple");
        pool.intern("cherry");
        pool.ensure_sorted();
        let sorted: Vec<&str> = pool.iter_sorted().map(|(_, s)| s).collect();
        assert_eq!(sorted, vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn test_type_pool_intern() {
        let mut pool = TypePool::new(StringPool::new());
        let idx1 = pool.intern("Lcom/A;");
        let _idx2 = pool.intern("Lcom/B;");
        let idx3 = pool.intern("Lcom/A;");
        assert_eq!(idx1, idx3);
        assert_eq!(pool.count(), 2);
    }

    #[test]
    fn test_proto_pool_intern() {
        let mut pool = ProtoPool::new();
        let idx = pool.intern("V", "V", vec!["I".to_string()]);
        assert_eq!(pool.count(), 1);
        assert_eq!(pool.get(idx).unwrap().shorty, "V");
    }

    #[test]
    fn test_field_pool_intern() {
        let mut pool = FieldPool::new();
        pool.intern("Lcom/A;", "x", "I");
        pool.intern("Lcom/A;", "y", "I");
        assert_eq!(pool.count(), 2);
    }

    #[test]
    fn test_method_pool_intern() {
        let mut pool = MethodPool::new();
        let proto = ProtoKey {
            shorty: "V".to_string(),
            return_type: "V".to_string(),
            parameters: vec![],
        };
        pool.intern("Lcom/A;", "foo", proto.clone());
        assert_eq!(pool.count(), 1);
    }

    #[test]
    fn test_class_pool_intern() {
        let mut pool = ClassPool::new();
        let entry = ClassEntry {
            type_descriptor: "Lcom/A;".to_string(),
            access_flags: 1,
            superclass_idx: None,
            interfaces: vec![],
            source_file: None,
            static_fields: vec![],
            instance_fields: vec![],
            direct_methods: vec![],
            virtual_methods: vec![],
        };
        pool.intern(entry);
        assert_eq!(pool.count(), 1);
    }
}
