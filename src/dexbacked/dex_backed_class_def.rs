use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationSet;
use crate::iface::class_def::ClassDef;
use crate::iface::field::Field;
use crate::iface::method::Method;

#[derive(Debug)]
pub struct DexBackedClassDef {
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

impl ClassDef for DexBackedClassDef {
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
