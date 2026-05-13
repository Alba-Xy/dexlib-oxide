use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationEntry;
use crate::iface::encoded_value::EncodedValue;
use crate::iface::reference::FieldReference;

pub trait Field: std::fmt::Debug + Send + Sync {
    fn defining_class(&self) -> &str;
    fn name(&self) -> &str;
    fn field_type(&self) -> &str;
    fn access_flags(&self) -> AccessFlags;
    fn annotations(&self) -> &[AnnotationEntry];
    fn initial_value(&self) -> Option<&EncodedValue>;
    fn hidden_api_restrictions(&self) -> u32 { 0 }
}

#[derive(Debug, Clone)]
pub struct FieldData {
    pub defining_class: String,
    pub name: String,
    pub field_type: String,
    pub access_flags: AccessFlags,
    pub annotations: Vec<AnnotationEntry>,
    pub initial_value: Option<EncodedValue>,
    pub hidden_api_restrictions: u32,
}

impl Field for FieldData {
    fn defining_class(&self) -> &str { &self.defining_class }
    fn name(&self) -> &str { &self.name }
    fn field_type(&self) -> &str { &self.field_type }
    fn access_flags(&self) -> AccessFlags { self.access_flags }
    fn annotations(&self) -> &[AnnotationEntry] { &self.annotations }
    fn initial_value(&self) -> Option<&EncodedValue> { self.initial_value.as_ref() }
    fn hidden_api_restrictions(&self) -> u32 { self.hidden_api_restrictions }
}

impl FieldData {
    pub fn to_field_reference(&self) -> FieldReference {
        FieldReference::new(&self.defining_class, &self.name, &self.field_type)
    }
}
