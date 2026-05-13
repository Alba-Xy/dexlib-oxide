use crate::iface::reference::{
    FieldReference, MethodProtoReference, MethodReference, Reference, ReferenceHolder,
    ReferenceType, StringReference, TypeReference,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImmutableStringReference {
    pub string: String,
}

impl ImmutableStringReference {
    pub fn new(string: impl Into<String>) -> Self {
        Self { string: string.into() }
    }
}

impl Reference for ImmutableStringReference {
    fn as_string_reference(&self) -> Option<&StringReference> { None }
    fn reference_type(&self) -> ReferenceType { ReferenceType::String }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImmutableTypeReference {
    pub type_descriptor: String,
}

impl ImmutableTypeReference {
    pub fn new(type_descriptor: impl Into<String>) -> Self {
        Self { type_descriptor: type_descriptor.into() }
    }
}

impl Reference for ImmutableTypeReference {
    fn as_type_reference(&self) -> Option<&TypeReference> { None }
    fn reference_type(&self) -> ReferenceType { ReferenceType::Type }
}

#[derive(Debug, Clone)]
pub struct ImmutableFieldReference {
    pub defining_class: String,
    pub field_name: String,
    pub field_type: String,
}

impl ImmutableFieldReference {
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
}

impl Reference for ImmutableFieldReference {
    fn as_field_reference(&self) -> Option<&FieldReference> { None }
    fn reference_type(&self) -> ReferenceType { ReferenceType::Field }
}

#[derive(Debug, Clone)]
pub struct ImmutableMethodReference {
    pub defining_class: String,
    pub method_name: String,
    pub parameter_types: Vec<String>,
    pub return_type: String,
}

impl ImmutableMethodReference {
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
}

impl Reference for ImmutableMethodReference {
    fn as_method_reference(&self) -> Option<&MethodReference> { None }
    fn reference_type(&self) -> ReferenceType { ReferenceType::Method }
}

#[derive(Debug, Clone)]
pub struct ImmutableMethodProtoReference {
    pub parameter_types: Vec<String>,
    pub return_type: String,
}

impl ImmutableMethodProtoReference {
    pub fn new(parameter_types: Vec<String>, return_type: impl Into<String>) -> Self {
        Self {
            parameter_types,
            return_type: return_type.into(),
        }
    }
}

impl Reference for ImmutableMethodProtoReference {
    fn as_method_proto_reference(&self) -> Option<&MethodProtoReference> { None }
    fn reference_type(&self) -> ReferenceType { ReferenceType::MethodProto }
}

impl From<&StringReference> for ImmutableStringReference {
    fn from(r: &StringReference) -> Self { Self::new(&r.string) }
}

impl From<&TypeReference> for ImmutableTypeReference {
    fn from(r: &TypeReference) -> Self { Self::new(&r.type_descriptor) }
}

impl From<&FieldReference> for ImmutableFieldReference {
    fn from(r: &FieldReference) -> Self {
        Self::new(&r.defining_class, &r.field_name, &r.field_type)
    }
}

impl From<&MethodReference> for ImmutableMethodReference {
    fn from(r: &MethodReference) -> Self {
        Self::new(&r.defining_class, &r.method_name, r.parameter_types.clone(), &r.return_type)
    }
}

impl From<&MethodProtoReference> for ImmutableMethodProtoReference {
    fn from(r: &MethodProtoReference) -> Self {
        Self::new(r.parameter_types.clone(), &r.return_type)
    }
}

impl From<&ReferenceHolder> for ImmutableReferenceHolder {
    fn from(r: &ReferenceHolder) -> Self {
        match r {
            ReferenceHolder::None => ImmutableReferenceHolder::None,
            ReferenceHolder::String(s) => ImmutableReferenceHolder::String(ImmutableStringReference::from(s)),
            ReferenceHolder::Type(t) => ImmutableReferenceHolder::Type(ImmutableTypeReference::from(t)),
            ReferenceHolder::Field(f) => ImmutableReferenceHolder::Field(ImmutableFieldReference::from(f)),
            ReferenceHolder::Method(m) => ImmutableReferenceHolder::Method(ImmutableMethodReference::from(m)),
            ReferenceHolder::MethodProto(p) => ImmutableReferenceHolder::MethodProto(ImmutableMethodProtoReference::from(p)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ImmutableReferenceHolder {
    None,
    String(ImmutableStringReference),
    Type(ImmutableTypeReference),
    Field(ImmutableFieldReference),
    Method(ImmutableMethodReference),
    MethodProto(ImmutableMethodProtoReference),
}

impl ImmutableReferenceHolder {
    pub fn as_holder(&self) -> ReferenceHolder {
        match self {
            Self::None => ReferenceHolder::None,
            Self::String(s) => ReferenceHolder::String(StringReference::new(&s.string)),
            Self::Type(t) => ReferenceHolder::Type(TypeReference::new(&t.type_descriptor)),
            Self::Field(f) => ReferenceHolder::Field(FieldReference::new(&f.defining_class, &f.field_name, &f.field_type)),
            Self::Method(m) => ReferenceHolder::Method(MethodReference::new(&m.defining_class, &m.method_name, m.parameter_types.clone(), &m.return_type)),
            Self::MethodProto(p) => ReferenceHolder::MethodProto(MethodProtoReference::new(p.parameter_types.clone(), &p.return_type)),
        }
    }
}

impl Reference for ImmutableReferenceHolder {
    fn reference_type(&self) -> ReferenceType {
        match self {
            Self::None => ReferenceType::None,
            Self::String(_) => ReferenceType::String,
            Self::Type(_) => ReferenceType::Type,
            Self::Field(_) => ReferenceType::Field,
            Self::Method(_) => ReferenceType::Method,
            Self::MethodProto(_) => ReferenceType::MethodProto,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_string_reference() {
        let r = ImmutableStringReference::new("hello");
        assert_eq!(r.string, "hello");
        assert_eq!(r.reference_type(), ReferenceType::String);
    }

    #[test]
    fn test_immutable_field_reference() {
        let r = ImmutableFieldReference::new("Lcom/A;", "x", "I");
        assert_eq!(r.defining_class, "Lcom/A;");
        assert_eq!(r.field_name, "x");
        assert_eq!(r.field_type, "I");
        assert_eq!(r.reference_type(), ReferenceType::Field);
    }

    #[test]
    fn test_from_iface_reference() {
        let sr = StringReference::new("test");
        let isr = ImmutableStringReference::from(&sr);
        assert_eq!(isr.string, "test");

        let fr = FieldReference::new("Lcom/A;", "x", "I");
        let ifr = ImmutableFieldReference::from(&fr);
        assert_eq!(ifr.defining_class, "Lcom/A;");
    }

    #[test]
    fn test_immutable_reference_holder_from_holder() {
        let holder = ReferenceHolder::String(StringReference::new("test"));
        let ih = ImmutableReferenceHolder::from(&holder);
        assert_eq!(ih.reference_type(), ReferenceType::String);
    }
}
