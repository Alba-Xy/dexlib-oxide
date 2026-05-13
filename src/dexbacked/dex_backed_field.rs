use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationEntry;
use crate::iface::encoded_value::EncodedValue;
use crate::iface::field::Field;
use crate::iface::reference::FieldReference;

#[derive(Debug)]
pub struct DexBackedField {
    pub defining_class: String,
    pub name: String,
    pub field_type: String,
    pub access_flags: AccessFlags,
    pub field_index: u32,
    pub annotations: Vec<AnnotationEntry>,
    pub initial_value: Option<EncodedValue>,
    pub hidden_api_restrictions: u32,
}

impl Field for DexBackedField {
    fn defining_class(&self) -> &str { &self.defining_class }
    fn name(&self) -> &str { &self.name }
    fn field_type(&self) -> &str { &self.field_type }
    fn access_flags(&self) -> AccessFlags { self.access_flags }
    fn annotations(&self) -> &[AnnotationEntry] { &self.annotations }
    fn initial_value(&self) -> Option<&EncodedValue> { self.initial_value.as_ref() }
    fn hidden_api_restrictions(&self) -> u32 { self.hidden_api_restrictions }
}

impl DexBackedField {
    pub fn to_field_reference(&self) -> FieldReference {
        FieldReference::new(&self.defining_class, &self.name, &self.field_type)
    }
}
