use std::fmt;
use std::hash::{Hash, Hasher};

pub trait Reference: fmt::Debug + Clone + Send + Sync {
    fn as_string_reference(&self) -> Option<&StringReference> { None }
    fn as_type_reference(&self) -> Option<&TypeReference> { None }
    fn as_field_reference(&self) -> Option<&FieldReference> { None }
    fn as_method_reference(&self) -> Option<&MethodReference> { None }
    fn as_method_proto_reference(&self) -> Option<&MethodProtoReference> { None }
    fn reference_type(&self) -> ReferenceType;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReferenceType {
    None,
    String,
    Type,
    Field,
    Method,
    MethodProto,
    CallSite,
    MethodHandle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringReference {
    pub string: String,
}

impl StringReference {
    pub fn new(string: impl Into<String>) -> Self {
        Self { string: string.into() }
    }

    pub fn string(&self) -> &str {
        &self.string
    }
}

impl Reference for StringReference {
    fn as_string_reference(&self) -> Option<&StringReference> { Some(self) }
    fn reference_type(&self) -> ReferenceType { ReferenceType::String }
}

impl fmt::Display for StringReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self.string)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeReference {
    pub type_descriptor: String,
}

impl TypeReference {
    pub fn new(type_descriptor: impl Into<String>) -> Self {
        Self { type_descriptor: type_descriptor.into() }
    }

    pub fn type_descriptor(&self) -> &str {
        &self.type_descriptor
    }
}

impl Reference for TypeReference {
    fn as_type_reference(&self) -> Option<&TypeReference> { Some(self) }
    fn reference_type(&self) -> ReferenceType { ReferenceType::Type }
}

impl fmt::Display for TypeReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.type_descriptor)
    }
}

#[derive(Debug, Clone)]
pub struct FieldReference {
    pub defining_class: String,
    pub field_name: String,
    pub field_type: String,
}

impl FieldReference {
    pub fn new(
        defining_class: impl Into<String>,
        field_name: impl Into<String>,
        field_type: impl Into<String>,
    ) -> Self {
        Self {
            defining_class: defining_class.into(),
            field_name: field_name.into(),
            field_type: field_type.into(),
        }
    }

    pub fn defining_class(&self) -> &str { &self.defining_class }
    pub fn field_name(&self) -> &str { &self.field_name }
    pub fn field_type(&self) -> &str { &self.field_type }
}

impl PartialEq for FieldReference {
    fn eq(&self, other: &Self) -> bool {
        self.defining_class == other.defining_class
            && self.field_name == other.field_name
            && self.field_type == other.field_type
    }
}

impl Eq for FieldReference {}

impl Hash for FieldReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.defining_class.hash(state);
        self.field_name.hash(state);
        self.field_type.hash(state);
    }
}

impl Reference for FieldReference {
    fn as_field_reference(&self) -> Option<&FieldReference> { Some(self) }
    fn reference_type(&self) -> ReferenceType { ReferenceType::Field }
}

impl fmt::Display for FieldReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}->{}:{}", self.defining_class, self.field_name, self.field_type)
    }
}

#[derive(Debug, Clone)]
pub struct MethodReference {
    pub defining_class: String,
    pub method_name: String,
    pub parameter_types: Vec<String>,
    pub return_type: String,
}

impl MethodReference {
    pub fn new(
        defining_class: impl Into<String>,
        method_name: impl Into<String>,
        parameter_types: Vec<String>,
        return_type: impl Into<String>,
    ) -> Self {
        Self {
            defining_class: defining_class.into(),
            method_name: method_name.into(),
            parameter_types,
            return_type: return_type.into(),
        }
    }

    pub fn defining_class(&self) -> &str { &self.defining_class }
    pub fn method_name(&self) -> &str { &self.method_name }
    pub fn parameter_types(&self) -> &[String] { &self.parameter_types }
    pub fn return_type(&self) -> &str { &self.return_type }
}

impl PartialEq for MethodReference {
    fn eq(&self, other: &Self) -> bool {
        self.defining_class == other.defining_class
            && self.method_name == other.method_name
            && self.parameter_types == other.parameter_types
            && self.return_type == other.return_type
    }
}

impl Eq for MethodReference {}

impl Hash for MethodReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.defining_class.hash(state);
        self.method_name.hash(state);
        self.parameter_types.hash(state);
        self.return_type.hash(state);
    }
}

impl Reference for MethodReference {
    fn as_method_reference(&self) -> Option<&MethodReference> { Some(self) }
    fn reference_type(&self) -> ReferenceType { ReferenceType::Method }
}

impl fmt::Display for MethodReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.parameter_types.join("");
        write!(f, "{}->{}({}){}", self.defining_class, self.method_name, params, self.return_type)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodProtoReference {
    pub parameter_types: Vec<String>,
    pub return_type: String,
}

impl MethodProtoReference {
    pub fn new(parameter_types: Vec<String>, return_type: impl Into<String>) -> Self {
        Self {
            parameter_types,
            return_type: return_type.into(),
        }
    }

    pub fn parameter_types(&self) -> &[String] { &self.parameter_types }
    pub fn return_type(&self) -> &str { &self.return_type }
}

impl Reference for MethodProtoReference {
    fn as_method_proto_reference(&self) -> Option<&MethodProtoReference> { Some(self) }
    fn reference_type(&self) -> ReferenceType { ReferenceType::MethodProto }
}

impl fmt::Display for MethodProtoReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params = self.parameter_types.join("");
        write!(f, "({}){}", params, self.return_type)
    }
}

#[derive(Debug, Clone)]
pub enum ReferenceHolder {
    None,
    String(StringReference),
    Type(TypeReference),
    Field(FieldReference),
    Method(MethodReference),
    MethodProto(MethodProtoReference),
}

impl Reference for ReferenceHolder {
    fn as_string_reference(&self) -> Option<&StringReference> {
        match self {
            ReferenceHolder::String(r) => Some(r),
            _ => None,
        }
    }
    fn as_type_reference(&self) -> Option<&TypeReference> {
        match self {
            ReferenceHolder::Type(r) => Some(r),
            _ => None,
        }
    }
    fn as_field_reference(&self) -> Option<&FieldReference> {
        match self {
            ReferenceHolder::Field(r) => Some(r),
            _ => None,
        }
    }
    fn as_method_reference(&self) -> Option<&MethodReference> {
        match self {
            ReferenceHolder::Method(r) => Some(r),
            _ => None,
        }
    }
    fn as_method_proto_reference(&self) -> Option<&MethodProtoReference> {
        match self {
            ReferenceHolder::MethodProto(r) => Some(r),
            _ => None,
        }
    }
    fn reference_type(&self) -> ReferenceType {
        match self {
            ReferenceHolder::None => ReferenceType::None,
            ReferenceHolder::String(_) => ReferenceType::String,
            ReferenceHolder::Type(_) => ReferenceType::Type,
            ReferenceHolder::Field(_) => ReferenceType::Field,
            ReferenceHolder::Method(_) => ReferenceType::Method,
            ReferenceHolder::MethodProto(_) => ReferenceType::MethodProto,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_reference() {
        let r = StringReference::new("hello");
        assert_eq!(r.string(), "hello");
        assert_eq!(r.reference_type(), ReferenceType::String);
        assert!(r.as_string_reference().is_some());
        assert!(r.as_type_reference().is_none());
    }

    #[test]
    fn test_type_reference() {
        let r = TypeReference::new("Lcom/example/MyClass;");
        assert_eq!(r.type_descriptor(), "Lcom/example/MyClass;");
        assert_eq!(r.reference_type(), ReferenceType::Type);
    }

    #[test]
    fn test_field_reference_hash_eq() {
        let r1 = FieldReference::new("Lcom/A;", "field", "I");
        let r2 = FieldReference::new("Lcom/A;", "field", "I");
        let r3 = FieldReference::new("Lcom/B;", "field", "I");
        assert_eq!(r1, r2);
        assert_ne!(r1, r3);
        let mut map = std::collections::HashMap::new();
        map.insert(r1.clone(), 1);
        assert_eq!(map.get(&r2), Some(&1));
    }

    #[test]
    fn test_method_reference_hash_eq() {
        let r1 = MethodReference::new("Lcom/A;", "foo", vec!["I".to_string()], "V");
        let r2 = MethodReference::new("Lcom/A;", "foo", vec!["I".to_string()], "V");
        assert_eq!(r1, r2);
        let mut map = std::collections::HashMap::new();
        map.insert(r1.clone(), 1);
        assert_eq!(map.get(&r2), Some(&1));
    }

    #[test]
    fn test_reference_holder() {
        let h = ReferenceHolder::String(StringReference::new("test"));
        assert_eq!(h.reference_type(), ReferenceType::String);
        assert!(h.as_string_reference().is_some());
        assert!(h.as_method_reference().is_none());
    }

    #[test]
    fn test_display() {
        let fr = FieldReference::new("Lcom/A;", "x", "I");
        assert_eq!(fr.to_string(), "Lcom/A;->x:I");
        let mr = MethodReference::new("Lcom/A;", "foo", vec!["I".to_string(), "J".to_string()], "V");
        assert_eq!(mr.to_string(), "Lcom/A;->foo(IJ)V");
    }
}
