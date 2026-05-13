use crate::iface::reference::{FieldReference, MethodReference, MethodProtoReference, StringReference, TypeReference};

#[derive(Debug, Clone)]
pub enum EncodedValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    Char(char),
    String(StringReference),
    Type(TypeReference),
    Field(FieldReference),
    Method(MethodReference),
    MethodProto(MethodProtoReference),
    MethodHandle(u16),
    Null,
    Boolean(bool),
    Array(Vec<EncodedValue>),
    Annotation(AnnotationEncodedValue),
}

#[derive(Debug, Clone)]
pub struct AnnotationEncodedValue {
    pub type_descriptor: String,
    pub elements: Vec<(String, EncodedValue)>,
}

impl AnnotationEncodedValue {
    pub fn new(type_descriptor: impl Into<String>, elements: Vec<(String, EncodedValue)>) -> Self {
        Self { type_descriptor: type_descriptor.into(), elements }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EncodedValueType {
    Byte = 0x00,
    Short = 0x02,
    Int = 0x04,
    Long = 0x06,
    Float = 0x10,
    Double = 0x11,
    Char = 0x03,
    String = 0x17,
    Type = 0x18,
    Field = 0x19,
    Method = 0x1a,
    MethodProto = 0x1b,
    MethodHandle = 0x1c,
    Null = 0x1e,
    Boolean = 0x1f,
    Array = 0x1d,
    Annotation = 0x20,
}
