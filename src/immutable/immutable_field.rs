use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationEntry;
use crate::iface::encoded_value::EncodedValue;
use crate::iface::field::Field;
use crate::iface::reference::FieldReference;

#[derive(Debug)]
pub struct ImmutableField {
    pub defining_class: String,
    pub name: String,
    pub field_type: String,
    pub access_flags: AccessFlags,
    pub annotations: Vec<AnnotationEntry>,
    pub initial_value: Option<EncodedValue>,
    pub hidden_api_restrictions: u32,
}

impl ImmutableField {
    pub fn new(
        defining_class: impl Into<String>,
        name: impl Into<String>,
        field_type: impl Into<String>,
        access_flags: AccessFlags,
    ) -> Self {
        Self {
            defining_class: defining_class.into(),
            name: name.into(),
            field_type: field_type.into(),
            access_flags,
            annotations: Vec::new(),
            initial_value: None,
            hidden_api_restrictions: 0,
        }
    }

    pub fn to_field_reference(&self) -> FieldReference {
        FieldReference::new(&self.defining_class, &self.name, &self.field_type)
    }
}

impl Field for ImmutableField {
    fn defining_class(&self) -> &str { &self.defining_class }
    fn name(&self) -> &str { &self.name }
    fn field_type(&self) -> &str { &self.field_type }
    fn access_flags(&self) -> AccessFlags { self.access_flags }
    fn annotations(&self) -> &[AnnotationEntry] { &self.annotations }
    fn initial_value(&self) -> Option<&EncodedValue> { self.initial_value.as_ref() }
    fn hidden_api_restrictions(&self) -> u32 { self.hidden_api_restrictions }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_field() {
        let f = ImmutableField::new("Lcom/A;", "x", "I", AccessFlags::ACC_PUBLIC | AccessFlags::ACC_STATIC);
        assert_eq!(f.defining_class(), "Lcom/A;");
        assert_eq!(f.name(), "x");
        assert_eq!(f.field_type(), "I");
        assert!(f.access_flags().contains(AccessFlags::ACC_PUBLIC));
        assert!(f.access_flags().contains(AccessFlags::ACC_STATIC));
    }

    #[test]
    fn test_immutable_field_reference() {
        let f = ImmutableField::new("Lcom/A;", "x", "I", AccessFlags::ACC_PUBLIC);
        let r = f.to_field_reference();
        assert_eq!(r.defining_class(), "Lcom/A;");
        assert_eq!(r.field_name(), "x");
    }
}
