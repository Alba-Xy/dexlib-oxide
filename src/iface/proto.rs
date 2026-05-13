use crate::iface::reference::MethodProtoReference;

pub trait MethodProto: std::fmt::Debug + Send + Sync {
    fn parameter_types(&self) -> &[String];
    fn return_type(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodProtoData {
    pub parameter_types: Vec<String>,
    pub return_type: String,
}

impl MethodProto for MethodProtoData {
    fn parameter_types(&self) -> &[String] { &self.parameter_types }
    fn return_type(&self) -> &str { &self.return_type }
}

impl MethodProtoData {
    pub fn to_method_proto_reference(&self) -> MethodProtoReference {
        MethodProtoReference::new(self.parameter_types.clone(), &self.return_type)
    }
}
