use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationEntry;
use crate::iface::debug_item::DebugItem;
use crate::iface::instruction::Instruction;
use crate::iface::method::{Method, MethodImplementation, TryBlock};
use crate::iface::reference::MethodReference;

#[derive(Debug)]
pub struct DexBackedMethod {
    pub defining_class: String,
    pub name: String,
    pub parameter_types: Vec<String>,
    pub return_type: String,
    pub access_flags: AccessFlags,
    pub method_index: u32,
    pub annotations: Vec<AnnotationEntry>,
    pub hidden_api_restrictions: u32,
    pub implementation: Option<DexBackedMethodImplementation>,
}

impl Method for DexBackedMethod {
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

impl DexBackedMethod {
    pub fn to_method_reference(&self) -> MethodReference {
        MethodReference::new(
            &self.defining_class,
            &self.name,
            self.parameter_types.clone(),
            &self.return_type,
        )
    }
}

#[derive(Debug)]
pub struct DexBackedMethodImplementation {
    pub register_count: u16,
    pub instructions: Vec<Box<dyn Instruction>>,
    pub try_blocks: Vec<TryBlock>,
    pub debug_items: Vec<DebugItem>,
}

impl MethodImplementation for DexBackedMethodImplementation {
    fn register_count(&self) -> u16 { self.register_count }
    fn instructions(&self) -> &[Box<dyn Instruction>] { &self.instructions }
    fn try_blocks(&self) -> &[TryBlock] { &self.try_blocks }
    fn debug_items(&self) -> &[DebugItem] { &self.debug_items }
}
