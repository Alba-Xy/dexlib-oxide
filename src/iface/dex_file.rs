use crate::base::opcode::Opcode;
use crate::iface::class_def::ClassDef;

pub trait DexFile: std::fmt::Debug + Send + Sync {
    fn classes(&self) -> &[Box<dyn ClassDef>];
    fn find_class_by_name(&self, type_descriptor: &str) -> Option<&dyn ClassDef> {
        for class_def in self.classes() {
            if class_def.type_descriptor() == type_descriptor {
                return Some(class_def.as_ref());
            }
        }
        None
    }
    fn opcodes(&self) -> &Opcode;
}
