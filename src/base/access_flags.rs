bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AccessFlags: u32 {
        const ACC_PUBLIC       = 0x1;
        const ACC_PRIVATE      = 0x2;
        const ACC_PROTECTED    = 0x4;
        const ACC_STATIC       = 0x8;
        const ACC_FINAL        = 0x10;
        const ACC_SYNCHRONIZED = 0x20;
        const ACC_VOLATILE     = 0x40;
        const ACC_BRIDGE       = 0x40;
        const ACC_TRANSIENT    = 0x80;
        const ACC_VARARGS      = 0x80;
        const ACC_NATIVE       = 0x100;
        const ACC_INTERFACE    = 0x200;
        const ACC_ABSTRACT     = 0x400;
        const ACC_STRICT       = 0x800;
        const ACC_SYNTHETIC    = 0x1000;
        const ACC_ANNOTATION   = 0x2000;
        const ACC_ENUM         = 0x4000;
        const ACC_CONSTRUCTOR  = 0x10000;
        const ACC_DECLARED_SYNCHRONIZED = 0x20000;
    }
}

impl AccessFlags {
    pub fn is_valid_for_class(self) -> bool {
        let valid = AccessFlags::ACC_PUBLIC
            | AccessFlags::ACC_PRIVATE
            | AccessFlags::ACC_PROTECTED
            | AccessFlags::ACC_STATIC
            | AccessFlags::ACC_FINAL
            | AccessFlags::ACC_INTERFACE
            | AccessFlags::ACC_ABSTRACT
            | AccessFlags::ACC_SYNTHETIC
            | AccessFlags::ACC_ANNOTATION
            | AccessFlags::ACC_ENUM;
        self.intersects(valid)
    }

    pub fn is_valid_for_method(self) -> bool {
        let valid = AccessFlags::ACC_PUBLIC
            | AccessFlags::ACC_PRIVATE
            | AccessFlags::ACC_PROTECTED
            | AccessFlags::ACC_STATIC
            | AccessFlags::ACC_FINAL
            | AccessFlags::ACC_SYNCHRONIZED
            | AccessFlags::ACC_BRIDGE
            | AccessFlags::ACC_VARARGS
            | AccessFlags::ACC_NATIVE
            | AccessFlags::ACC_ABSTRACT
            | AccessFlags::ACC_STRICT
            | AccessFlags::ACC_SYNTHETIC
            | AccessFlags::ACC_CONSTRUCTOR
            | AccessFlags::ACC_DECLARED_SYNCHRONIZED;
        self.intersects(valid)
    }

    pub fn is_valid_for_field(self) -> bool {
        let valid = AccessFlags::ACC_PUBLIC
            | AccessFlags::ACC_PRIVATE
            | AccessFlags::ACC_PROTECTED
            | AccessFlags::ACC_STATIC
            | AccessFlags::ACC_FINAL
            | AccessFlags::ACC_VOLATILE
            | AccessFlags::ACC_TRANSIENT
            | AccessFlags::ACC_SYNTHETIC
            | AccessFlags::ACC_ENUM;
        self.intersects(valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitflags_operations() {
        let flags = AccessFlags::ACC_PUBLIC | AccessFlags::ACC_STATIC | AccessFlags::ACC_FINAL;
        assert!(flags.contains(AccessFlags::ACC_PUBLIC));
        assert!(flags.contains(AccessFlags::ACC_STATIC));
        assert!(flags.contains(AccessFlags::ACC_FINAL));
        assert!(!flags.contains(AccessFlags::ACC_PRIVATE));
    }

    #[test]
    fn test_flag_values() {
        assert_eq!(AccessFlags::ACC_PUBLIC.bits(), 0x1);
        assert_eq!(AccessFlags::ACC_STATIC.bits(), 0x8);
        assert_eq!(AccessFlags::ACC_CONSTRUCTOR.bits(), 0x10000);
        assert_eq!(AccessFlags::ACC_DECLARED_SYNCHRONIZED.bits(), 0x20000);
    }

    #[test]
    fn test_combined_flags() {
        let flags = AccessFlags::ACC_PUBLIC | AccessFlags::ACC_STATIC;
        assert_eq!(flags.bits(), 0x9);
    }

    #[test]
    fn test_validity_checks() {
        let class_flags = AccessFlags::ACC_INTERFACE;
        assert!(class_flags.is_valid_for_class());
        assert!(!class_flags.is_valid_for_method());

        let method_flags = AccessFlags::ACC_NATIVE;
        assert!(method_flags.is_valid_for_method());
        assert!(!method_flags.is_valid_for_field());

        let field_flags = AccessFlags::ACC_ENUM;
        assert!(field_flags.is_valid_for_field());
        assert!(!field_flags.is_valid_for_method());
    }
}
