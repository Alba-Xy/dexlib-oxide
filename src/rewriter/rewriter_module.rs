use crate::iface::class_def::ClassDef;
use crate::iface::field::Field;
use crate::iface::method::{Method, MethodImplementation};
use crate::iface::instruction::Instruction;
use crate::immutable::immutable_class_def::ImmutableClassDef;
use crate::immutable::immutable_field::ImmutableField;
use crate::immutable::immutable_method::{ImmutableMethod, ImmutableMethodImplementation};

pub trait RewriterModule {
    fn rewrite_class_def(&self, class_def: &dyn ClassDef) -> Box<dyn ClassDef> {
        let mut static_fields = Vec::new();
        for f in class_def.static_fields() {
            static_fields.push(self.rewrite_field(f.as_ref()));
        }
        
        let mut instance_fields = Vec::new();
        for f in class_def.instance_fields() {
            instance_fields.push(self.rewrite_field(f.as_ref()));
        }
        
        let mut direct_methods = Vec::new();
        for m in class_def.direct_methods() {
            direct_methods.push(self.rewrite_method(m.as_ref()));
        }
        
        let mut virtual_methods = Vec::new();
        for m in class_def.virtual_methods() {
            virtual_methods.push(self.rewrite_method(m.as_ref()));
        }
        
        Box::new(ImmutableClassDef {
            type_descriptor: class_def.type_descriptor().to_string(),
            access_flags: class_def.access_flags(),
            superclass: class_def.superclass().map(|s| s.to_string()),
            interfaces: class_def.interfaces().to_vec(),
            source_file: class_def.source_file().map(|s| s.to_string()),
            annotations: class_def.annotations().clone(),
            static_fields,
            instance_fields,
            direct_methods,
            virtual_methods,
        })
    }

    fn rewrite_method(&self, method: &dyn Method) -> Box<dyn Method> {
        let implementation = method.implementation().map(|imp| self.rewrite_method_impl(imp));
        Box::new(ImmutableMethod {
            defining_class: method.defining_class().to_string(),
            name: method.name().to_string(),
            access_flags: method.access_flags(),
            return_type: method.return_type().to_string(),
            parameter_types: method.parameter_types().to_vec(),
            annotations: Vec::new(),
            hidden_api_restrictions: 0,
            implementation,
        })
    }

    fn rewrite_field(&self, field: &dyn Field) -> Box<dyn Field> {
        Box::new(ImmutableField {
            defining_class: field.defining_class().to_string(),
            name: field.name().to_string(),
            field_type: field.field_type().to_string(),
            access_flags: field.access_flags(),
            annotations: Vec::new(),
            initial_value: None,
            hidden_api_restrictions: 0,
        })
    }

    fn rewrite_method_impl(&self, impl_data: &dyn MethodImplementation) -> ImmutableMethodImplementation {
        let mut instructions = Vec::new();
        for i in impl_data.instructions() {
            instructions.push(self.rewrite_instruction(i.as_ref()));
        }
        
        ImmutableMethodImplementation {
            register_count: impl_data.register_count(),
            instructions,
            try_blocks: impl_data.try_blocks().to_vec(),
            debug_items: impl_data.debug_items().to_vec(),
        }
    }

    fn rewrite_instruction(&self, instruction: &dyn Instruction) -> Box<dyn Instruction> {
        instruction.clone_boxed()
    }


}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::access_flags::AccessFlags;
    use crate::base::opcode::Opcode;
    use crate::immutable::immutable_instruction::ImmutableInstruction10x;

    struct TestRewriter;
    impl RewriterModule for TestRewriter {}

    #[test]
    fn test_default_rewriter_module() {
        let rewriter = TestRewriter;
        
        let class_def = ImmutableClassDef::new("Lcom/Test;", AccessFlags::ACC_PUBLIC);
        let rewritten = rewriter.rewrite_class_def(&class_def);
        assert_eq!(rewritten.type_descriptor(), "Lcom/Test;");
    }

    #[test]
    fn test_rewrite_instruction() {
        let rewriter = TestRewriter;
        let instr = ImmutableInstruction10x { opcode: Opcode::Nop };
        let rewritten = rewriter.rewrite_instruction(&instr);
        assert_eq!(rewritten.opcode(), Opcode::Nop);
    }


}
