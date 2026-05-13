use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationSet;
use crate::iface::encoded_value::EncodedValue;
use crate::iface::field::Field;
use crate::iface::method::Method;

pub trait ClassDef: std::fmt::Debug + Send + Sync {
    fn type_descriptor(&self) -> &str;
    fn access_flags(&self) -> AccessFlags;
    fn superclass(&self) -> Option<&str>;
    fn interfaces(&self) -> &[String];
    fn source_file(&self) -> Option<&str>;
    fn annotations(&self) -> &AnnotationSet;
    fn static_fields(&self) -> &[Box<dyn Field>];
    fn instance_fields(&self) -> &[Box<dyn Field>];
    fn direct_methods(&self) -> &[Box<dyn Method>];
    fn virtual_methods(&self) -> &[Box<dyn Method>];
    fn find_method(&self, name: &str, _proto: &str) -> Option<&dyn Method> {
        for m in self.direct_methods() {
            if m.name() == name {
                return Some(m.as_ref());
            }
        }
        for m in self.virtual_methods() {
            if m.name() == name {
                return Some(m.as_ref());
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct ClassDefData {
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
    pub static_values: Option<Vec<EncodedValue>>,
}

impl ClassDef for ClassDefData {
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
