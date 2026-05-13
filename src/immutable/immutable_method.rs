use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationEntry;
use crate::iface::debug_item::DebugItem;
use crate::iface::instruction::Instruction;
use crate::iface::method::{Method, MethodImplementation, TryBlock};
use crate::iface::reference::MethodReference;

#[derive(Debug)]
pub struct ImmutableMethod {
    pub defining_class: String,
    pub name: String,
    pub parameter_types: Vec<String>,
    pub return_type: String,
    pub access_flags: AccessFlags,
    pub annotations: Vec<AnnotationEntry>,
    pub hidden_api_restrictions: u32,
    pub implementation: Option<ImmutableMethodImplementation>,
}

impl ImmutableMethod {
    pub fn new(
        defining_class: impl Into<String>,
        name: impl Into<String>,
        parameter_types: Vec<String>,
        return_type: impl Into<String>,
        access_flags: AccessFlags,
    ) -> Self {
        Self {
            defining_class: defining_class.into(),
            name: name.into(),
            parameter_types,
            return_type: return_type.into(),
            access_flags,
            annotations: Vec::new(),
            hidden_api_restrictions: 0,
            implementation: None,
        }
    }

    pub fn to_method_reference(&self) -> MethodReference {
        MethodReference::new(
            &self.defining_class,
            &self.name,
            self.parameter_types.clone(),
            &self.return_type,
        )
    }
}

impl Method for ImmutableMethod {
    fn defining_class(&self) -> &str { &self.defining_class }
    fn name(&self) -> &str { &self.name }
    fn parameter_types(&self) -> &[String] { &self.parameter_types }
    fn return_type(&self) -> &str { &self.return_type }
    fn access_flags(&self) -> AccessFlags { self.access_flags }
    fn annotations(&self) -> &[AnnotationEntry] { &self.annotations }
    fn hidden_api_restrictions(&self) -> u32 { self.hidden_api_restrictions }
    fn implementation(&self) -> Option<&dyn MethodImplementation> {
        self.implementation.as_ref().map(|i| i as &dyn MethodImplementation)
    }
}

#[derive(Debug)]
pub struct ImmutableMethodImplementation {
    pub register_count: u16,
    pub instructions: Vec<Box<dyn Instruction>>,
    pub try_blocks: Vec<TryBlock>,
    pub debug_items: Vec<DebugItem>,
}

impl MethodImplementation for ImmutableMethodImplementation {
    fn register_count(&self) -> u16 { self.register_count }
    fn instructions(&self) -> &[Box<dyn Instruction>] { &self.instructions }
    fn try_blocks(&self) -> &[TryBlock] { &self.try_blocks }
    fn debug_items(&self) -> &[DebugItem] { &self.debug_items }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_method() {
        let m = ImmutableMethod::new(
            "Lcom/A;",
            "foo",
            vec!["I".to_string()],
            "V",
            AccessFlags::ACC_PUBLIC | AccessFlags::ACC_STATIC,
        );
        assert_eq!(m.defining_class(), "Lcom/A;");
        assert_eq!(m.name(), "foo");
        assert_eq!(m.parameter_types(), &["I".to_string()]);
        assert_eq!(m.return_type(), "V");
        assert!(m.access_flags().contains(AccessFlags::ACC_PUBLIC));
        assert!(m.implementation().is_none());
    }

    #[test]
    fn test_immutable_method_reference() {
        let m = ImmutableMethod::new(
            "Lcom/A;", "foo", vec!["I".to_string()], "V", AccessFlags::ACC_PUBLIC,
        );
        let r = m.to_method_reference();
        assert_eq!(r.defining_class(), "Lcom/A;");
        assert_eq!(r.method_name(), "foo");
    }

    #[test]
    fn test_immutable_method_implementation() {
        let impl_ = ImmutableMethodImplementation {
            register_count: 2,
            instructions: Vec::new(),
            try_blocks: Vec::new(),
            debug_items: Vec::new(),
        };
        assert_eq!(impl_.register_count(), 2);
        assert!(impl_.instructions().is_empty());
    }
}
