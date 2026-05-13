use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationSet;
use crate::iface::class_def::ClassDef;
use crate::iface::field::Field;
use crate::iface::method::Method;

#[derive(Debug)]
pub struct ImmutableClassDef {
    pub type_descriptor: String,
    pub access_flags: AccessFlags,
    pub superclass: Option<String>,
    pub interfaces: Vec<String>,
    pub source_file: Option<String>,
    pub annotations: AnnotationSet,
    pub static_fields: Vec<Box<dyn Field>>,
    pub instance_fields: Vec<Box<dyn Field>>,
    pub direct_methods: Vec<Box<dyn Method>>,
    pub virtual_methods: Vec<Box<dyn Method>>,
}

impl ImmutableClassDef {
    pub fn new(
        type_descriptor: impl Into<String>,
        access_flags: AccessFlags,
    ) -> Self {
        Self {
            type_descriptor: type_descriptor.into(),
            access_flags,
            superclass: None,
            interfaces: Vec::new(),
            source_file: None,
            annotations: AnnotationSet { annotations: Vec::new() },
            static_fields: Vec::new(),
            instance_fields: Vec::new(),
            direct_methods: Vec::new(),
            virtual_methods: Vec::new(),
        }
    }
}

impl ClassDef for ImmutableClassDef {
    fn type_descriptor(&self) -> &str { &self.type_descriptor }
    fn access_flags(&self) -> AccessFlags { self.access_flags }
    fn superclass(&self) -> Option<&str> { self.superclass.as_deref() }
    fn interfaces(&self) -> &[String] { &self.interfaces }
    fn source_file(&self) -> Option<&str> { self.source_file.as_deref() }
    fn annotations(&self) -> &AnnotationSet { &self.annotations }
    fn static_fields(&self) -> &[Box<dyn Field>] { &self.static_fields }
    fn instance_fields(&self) -> &[Box<dyn Field>] { &self.instance_fields }
    fn direct_methods(&self) -> &[Box<dyn Method>] { &self.direct_methods }
    fn virtual_methods(&self) -> &[Box<dyn Method>] { &self.virtual_methods }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_class_def() {
        let cd = ImmutableClassDef::new("Lcom/A;", AccessFlags::ACC_PUBLIC);
        assert_eq!(cd.type_descriptor(), "Lcom/A;");
        assert!(cd.access_flags().contains(AccessFlags::ACC_PUBLIC));
        assert!(cd.superclass().is_none());
        assert!(cd.interfaces().is_empty());
        assert!(cd.source_file().is_none());
        assert!(cd.static_fields().is_empty());
        assert!(cd.instance_fields().is_empty());
        assert!(cd.direct_methods().is_empty());
        assert!(cd.virtual_methods().is_empty());
    }

    #[test]
    fn test_immutable_class_def_with_superclass() {
        let mut cd = ImmutableClassDef::new("Lcom/B;", AccessFlags::ACC_PUBLIC);
        cd.superclass = Some("Lcom/A;".to_string());
        assert_eq!(cd.superclass(), Some("Lcom/A;"));
    }
}
