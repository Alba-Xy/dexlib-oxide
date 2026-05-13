use crate::base::access_flags::AccessFlags;
use crate::iface::encoded_value::EncodedValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnnotationVisibility {
    Build = 0,
    Runtime = 1,
    System = 2,
}

impl AnnotationVisibility {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Build),
            1 => Some(Self::Runtime),
            2 => Some(Self::System),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnnotationElement {
    pub name: String,
    pub value: EncodedValue,
}

impl AnnotationElement {
    pub fn new(name: impl Into<String>, value: EncodedValue) -> Self {
        Self { name: name.into(), value }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn value(&self) -> &EncodedValue { &self.value }
}

pub trait Annotation: std::fmt::Debug + Send + Sync {
    fn visibility(&self) -> AnnotationVisibility;
    fn type_descriptor(&self) -> &str;
    fn elements(&self) -> &[AnnotationElement];
}

#[derive(Debug, Clone)]
pub struct AnnotationSet {
    pub annotations: Vec<AnnotationEntry>,
}

#[derive(Debug, Clone)]
pub struct AnnotationEntry {
    pub visibility: AnnotationVisibility,
    pub type_descriptor: String,
    pub elements: Vec<AnnotationElement>,
    pub access_flags: AccessFlags,
}

impl Annotation for AnnotationEntry {
    fn visibility(&self) -> AnnotationVisibility { self.visibility }
    fn type_descriptor(&self) -> &str { &self.type_descriptor }
    fn elements(&self) -> &[AnnotationElement] { &self.elements }
}
