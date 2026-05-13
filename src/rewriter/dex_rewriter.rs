use crate::iface::class_def::ClassDef;
use crate::iface::dex_file::DexFile;
use crate::rewriter::rewriter_module::RewriterModule;

pub struct DexRewriter<'a> {
    module: &'a dyn RewriterModule,
}

impl<'a> DexRewriter<'a> {
    pub fn new(module: &'a dyn RewriterModule) -> Self {
        Self { module }
    }

    pub fn rewrite_dex_file(&self, dex_file: &dyn DexFile) -> RewrittenDexFile {
        let class_defs = dex_file.classes()
            .iter()
            .map(|c| self.rewrite_class_def(c.as_ref()))
            .collect();
        
        RewrittenDexFile { class_defs }
    }

    pub fn rewrite_classes(&self, classes: &[Box<dyn ClassDef>]) -> RewrittenDexFile {
        let class_defs = classes
            .iter()
            .map(|c| self.rewrite_class_def(c.as_ref()))
            .collect();
        
        RewrittenDexFile { class_defs }
    }

    pub fn rewrite_class_def(&self, class_def: &dyn ClassDef) -> Box<dyn ClassDef> {
        self.module.rewrite_class_def(class_def)
    }
}

#[derive(Debug)]
pub struct RewrittenDexFile {
    class_defs: Vec<Box<dyn ClassDef>>,
}

impl RewrittenDexFile {
    pub fn classes(&self) -> &[Box<dyn ClassDef>] {
        &self.class_defs
    }

    pub fn classes_mut(&mut self) -> &mut Vec<Box<dyn ClassDef>> {
        &mut self.class_defs
    }
}

impl DexFile for RewrittenDexFile {
    fn classes(&self) -> &[Box<dyn ClassDef>] {
        &self.class_defs
    }

    fn opcodes(&self) -> &crate::base::opcode::Opcode {
        &crate::base::opcode::Opcode::Nop
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::access_flags::AccessFlags;
    use crate::base::opcode::Opcode;
    use crate::immutable::immutable_class_def::ImmutableClassDef;
    use crate::immutable::immutable_method::{ImmutableMethod, ImmutableMethodImplementation};
    use crate::immutable::immutable_instruction::ImmutableInstruction10x;
    use crate::rewriter::rewriter_module::RewriterModule;

    struct PassThroughRewriter;
    impl RewriterModule for PassThroughRewriter {}

    #[test]
    fn test_dex_rewriter_new() {
        let rewriter = PassThroughRewriter;
        let _dex_rewriter = DexRewriter::new(&rewriter);
        assert!(true);
    }

    #[test]
    fn test_pass_through_rewrite() {
        let rewriter = PassThroughRewriter;
        let dex_rewriter = DexRewriter::new(&rewriter);
        
        let mut class_def = ImmutableClassDef::new("Lcom/Test;", AccessFlags::ACC_PUBLIC);
        
        let mut method = ImmutableMethod::new(
            "Lcom/Test;",
            "test",
            vec![],
            "V",
            AccessFlags::ACC_PUBLIC,
        );
        
        let impl_data = ImmutableMethodImplementation {
            register_count: 1,
            instructions: vec![
                Box::new(ImmutableInstruction10x { opcode: Opcode::Nop }),
                Box::new(ImmutableInstruction10x { opcode: Opcode::ReturnVoid }),
            ],
            try_blocks: vec![],
            debug_items: vec![],
        };
        method.implementation = Some(impl_data);
        
        class_def.direct_methods.push(Box::new(method));
        
        let rewritten_class = dex_rewriter.rewrite_class_def(&class_def);
        
        assert_eq!(rewritten_class.type_descriptor(), "Lcom/Test;");
        assert_eq!(rewritten_class.direct_methods().len(), 1);
    }

    #[test]
    fn test_rewritten_dex_file_impl() {
        let class_def = ImmutableClassDef::new("Lcom/Test;", AccessFlags::ACC_PUBLIC);
        let rewritten_dex = RewrittenDexFile {
            class_defs: vec![Box::new(class_def)],
        };
        
        assert_eq!(rewritten_dex.classes().len(), 1);
        let _ = rewritten_dex.opcodes();
    }
}
