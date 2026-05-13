use crate::base::access_flags::AccessFlags;
use crate::iface::annotation::AnnotationEntry;
use crate::iface::debug_item::DebugItem;
use crate::iface::instruction::Instruction;
use crate::iface::reference::MethodReference;

pub trait Method: std::fmt::Debug + Send + Sync {
    fn defining_class(&self) -> &str;
    fn name(&self) -> &str;
    fn parameter_types(&self) -> &[String];
    fn return_type(&self) -> &str;
    fn access_flags(&self) -> AccessFlags;
    fn annotations(&self) -> &[AnnotationEntry];
    fn hidden_api_restrictions(&self) -> u32 { 0 }
    fn implementation(&self) -> Option<&dyn MethodImplementation>;
}

pub trait MethodImplementation: std::fmt::Debug + Send + Sync {
    fn register_count(&self) -> u16;
    fn instructions(&self) -> &[Box<dyn Instruction>];
    fn try_blocks(&self) -> &[TryBlock];
    fn debug_items(&self) -> &[DebugItem];
}

#[derive(Debug, Clone)]
pub struct TryBlock {
    pub start_code_address: u32,
    pub code_unit_count: u16,
    pub handlers: Vec<ExceptionHandler>,
}

impl TryBlock {
    pub fn end_code_address(&self) -> u32 {
        self.start_code_address + self.code_unit_count as u32
    }
}

#[derive(Debug, Clone)]
pub struct ExceptionHandler {
    pub handler_type: Option<String>,
    pub handler_code_address: u32,
}

impl ExceptionHandler {
    pub fn is_catch_all(&self) -> bool {
        self.handler_type.is_none()
    }
}

#[derive(Debug)]
pub struct MethodData {
    pub defining_class: String,
    pub name: String,
    pub parameter_types: Vec<String>,
    pub return_type: String,
    pub access_flags: AccessFlags,
    pub annotations: Vec<AnnotationEntry>,
    pub hidden_api_restrictions: u32,
    pub implementation: Option<MethodImplementationData>,
}

impl Method for MethodData {
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

impl MethodData {
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
pub struct MethodImplementationData {
    pub register_count: u16,
    pub instructions: Vec<Box<dyn Instruction>>,
    pub try_blocks: Vec<TryBlock>,
    pub debug_items: Vec<DebugItem>,
}

impl MethodImplementation for MethodImplementationData {
    fn register_count(&self) -> u16 { self.register_count }
    fn instructions(&self) -> &[Box<dyn Instruction>] { &self.instructions }
    fn try_blocks(&self) -> &[TryBlock] { &self.try_blocks }
    fn debug_items(&self) -> &[DebugItem] { &self.debug_items }
}
