use std::io::Read;
use dexlib_oxide::dexbacked::DexBackedDexFile;
use dexlib_oxide::iface::ClassDef;
use dexlib_oxide::immutable::{
    ImmutableClassDef, ImmutableField, ImmutableMethod, ImmutableMethodImplementation,
};
use dexlib_oxide::builder::{
    MutableMethodImplementation, make_nop, make_return_void,
};
use dexlib_oxide::base::opcode::Opcode;
use dexlib_oxide::writer::DexPool;
use dexlib_oxide::rewriter::{DexRewriter, RewriterModule};
use dexlib_oxide::smali::assemble_smali;

const TEST_APK_PATH: &str = "dexlibApk/app-release-unsigned.apk";

fn get_test_apk_path() -> Option<String> {
    if std::path::Path::new(TEST_APK_PATH).exists() {
        return Some(TEST_APK_PATH.to_string());
    }
    if let Ok(env_path) = std::env::var("DEXLIB_TEST_APK") {
        if std::path::Path::new(&env_path).exists() {
            return Some(env_path);
        }
    }
    None
}

fn extract_classes_dex(apk_path: &str) -> Vec<Vec<u8>> {
    let file = std::fs::File::open(apk_path).expect("Failed to open APK file");
    let mut archive = zip::ZipArchive::new(file).expect("Failed to parse APK as ZIP");

    let mut dex_data_list = Vec::new();

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).expect("Failed to get entry");
        let name = entry.name().to_string();

        if name == "classes.dex"
            || name.starts_with("classes") && name.ends_with(".dex")
        {
            let mut data = Vec::new();
            entry.read_to_end(&mut data).expect("Failed to read DEX entry");
            dex_data_list.push(data);
        }
    }

    dex_data_list
}

#[test]
fn test_parse_real_apk_header() {
    let apk_path = match get_test_apk_path() {
        Some(p) => p,
        None => { eprintln!("Skipping test_parse_real_apk_header: test APK not found at '{}'", TEST_APK_PATH); return; }
    };
    let dex_list = extract_classes_dex(&apk_path);
    assert!(!dex_list.is_empty(), "APK should contain at least one classes.dex");

    for (i, dex_data) in dex_list.iter().enumerate() {
        let dex = DexBackedDexFile::from_bytes(dex_data.clone())
            .unwrap_or_else(|e| panic!("Failed to parse classes{}.dex: {}", if i == 0 { String::new() } else { format!("{}", i + 1) }, e));

        let header = dex.header();

        assert_eq!(&header.magic[0..4], b"dex\n", "Invalid DEX magic in DEX #{}", i);
        assert!(header.file_size > 0, "File size should be > 0");
        assert!(header.header_size >= 0x70, "Header size should be >= 0x70");
        assert_eq!(header.endian_tag, 0x12345678, "Endian tag should be 0x12345678");

        let version = header.dex_version();
        assert!(!version.is_empty(), "DEX version should not be empty");
        eprintln!("DEX #{} version: {}, classes: {}, strings: {}, types: {}, protos: {}, fields: {}, methods: {}",
            i, version, header.class_defs_size, header.string_ids_size,
            header.type_ids_size, header.proto_ids_size,
            header.field_ids_size, header.method_ids_size);
    }
}

#[test]
fn test_parse_real_apk_classes() {
    let apk_path = match get_test_apk_path() {
        Some(p) => p,
        None => { eprintln!("Skipping test_parse_real_apk_classes: test APK not found at '{}'", TEST_APK_PATH); return; }
    };
    let dex_list = extract_classes_dex(&apk_path);
    assert!(!dex_list.is_empty());

    for (i, dex_data) in dex_list.iter().enumerate() {
        let dex = DexBackedDexFile::from_bytes(dex_data.clone())
            .expect("Failed to parse DEX");

        let classes = dex.parse_classes();
        assert!(!classes.is_empty(), "DEX #{} should have classes", i);
        assert_eq!(classes.len(), dex.class_count() as usize,
            "Parsed class count should match header class count");

        for class in &classes {
            let type_desc = class.type_descriptor();
            assert!(!type_desc.is_empty(), "Class type descriptor should not be empty");

            if let Some(superclass) = class.superclass() {
                assert!(!superclass.is_empty(), "Superclass should not be empty string");
            }

            for method in class.direct_methods() {
                assert!(!method.name().is_empty(), "Method name should not be empty");
                assert!(!method.defining_class().is_empty(), "Method defining class should not be empty");
                assert!(!method.return_type().is_empty(), "Method return type should not be empty");
            }

            for method in class.virtual_methods() {
                assert!(!method.name().is_empty(), "Method name should not be empty");
            }

            for field in class.static_fields() {
                assert!(!field.name().is_empty(), "Field name should not be empty");
                assert!(!field.field_type().is_empty(), "Field type should not be empty");
            }

            for field in class.instance_fields() {
                assert!(!field.name().is_empty(), "Field name should not be empty");
                assert!(!field.field_type().is_empty(), "Field type should not be empty");
            }
        }

        eprintln!("DEX #{}: {} classes parsed successfully", i, classes.len());
    }
}

#[test]
fn test_parse_real_apk_method_implementations() {
    let apk_path = match get_test_apk_path() {
        Some(p) => p,
        None => { eprintln!("Skipping test_parse_real_apk_method_implementations: test APK not found at '{}'", TEST_APK_PATH); return; }
    };
    let dex_list = extract_classes_dex(&apk_path);
    assert!(!dex_list.is_empty());

    let mut total_methods = 0usize;
    let mut methods_with_impl = 0usize;
    let mut total_instructions = 0usize;

    for (i, dex_data) in dex_list.iter().enumerate() {
        let dex = DexBackedDexFile::from_bytes(dex_data.clone())
            .expect("Failed to parse DEX");

        let classes = dex.parse_classes();

        for class in &classes {
            for method in class.direct_methods().iter().chain(class.virtual_methods().iter()) {
                total_methods += 1;
                if let Some(impl_) = method.implementation() {
                    methods_with_impl += 1;
                    total_instructions += impl_.instructions().len();

                    for instr in impl_.instructions() {
                        let opcode = instr.opcode();
                        assert!(!opcode.to_string().is_empty(), "Opcode name should not be empty");
                    }
                }
            }
        }

        eprintln!("DEX #{}: {} methods total, {} with implementations, {} instructions",
            i, total_methods, methods_with_impl, total_instructions);
    }

    assert!(total_methods > 0, "Should have at least one method");
    assert!(methods_with_impl > 0, "Should have at least one method with implementation");
    assert!(total_instructions > 0, "Should have at least one instruction");
}

#[test]
fn test_parse_real_apk_string_table() {
    let apk_path = match get_test_apk_path() {
        Some(p) => p,
        None => { eprintln!("Skipping test_parse_real_apk_string_table: test APK not found at '{}'", TEST_APK_PATH); return; }
    };
    let dex_list = extract_classes_dex(&apk_path);
    assert!(!dex_list.is_empty());

    for (i, dex_data) in dex_list.iter().enumerate() {
        let dex = DexBackedDexFile::from_bytes(dex_data.clone())
            .expect("Failed to parse DEX");

        let string_count = dex.string_count();
        assert!(string_count > 0, "String table should not be empty");

        for idx in 0..string_count.min(100) {
            let s = dex.get_string(idx);
            assert!(s.is_some(), "String at index {} should be parseable", idx);
        }

        eprintln!("DEX #{}: {} strings in table", i, string_count);
    }
}

#[test]
fn test_convert_dexbacked_to_immutable() {
    let apk_path = match get_test_apk_path() {
        Some(p) => p,
        None => { eprintln!("Skipping test_convert_dexbacked_to_immutable: test APK not found at '{}'", TEST_APK_PATH); return; }
    };
    let dex_list = extract_classes_dex(&apk_path);
    assert!(!dex_list.is_empty());

    let dex = DexBackedDexFile::from_bytes(dex_list[0].clone())
        .expect("Failed to parse DEX");

    let classes = dex.parse_classes();
    assert!(!classes.is_empty());

    let mut immutable_classes = 0usize;
    let mut immutable_methods = 0usize;
    let mut immutable_fields = 0usize;

    for class in &classes {
        let mut icd = ImmutableClassDef::new(
            class.type_descriptor(),
            class.access_flags(),
        );
        icd.superclass = class.superclass().map(|s| s.to_string());
        icd.source_file = class.source_file().map(|s| s.to_string());

        for method in class.direct_methods().iter().chain(class.virtual_methods().iter()) {
            let mut im = ImmutableMethod::new(
                method.defining_class(),
                method.name(),
                method.parameter_types().to_vec(),
                method.return_type(),
                method.access_flags(),
            );

            if let Some(impl_) = method.implementation() {
                let instructions: Vec<_> = impl_.instructions()
                    .iter()
                    .map(|i| i.clone_boxed())
                    .collect();
                im.implementation = Some(ImmutableMethodImplementation {
                    register_count: impl_.register_count(),
                    instructions,
                    try_blocks: impl_.try_blocks().to_vec(),
                    debug_items: impl_.debug_items().to_vec(),
                });
            }

            icd.direct_methods.push(Box::new(im));
            immutable_methods += 1;
        }

        for field in class.static_fields() {
            let f = ImmutableField::new(
                field.defining_class(),
                field.name(),
                field.field_type(),
                field.access_flags(),
            );
            icd.static_fields.push(Box::new(f));
            immutable_fields += 1;
        }

        for field in class.instance_fields() {
            let f = ImmutableField::new(
                field.defining_class(),
                field.name(),
                field.field_type(),
                field.access_flags(),
            );
            icd.instance_fields.push(Box::new(f));
            immutable_fields += 1;
        }

        immutable_classes += 1;
    }

    assert_eq!(immutable_classes, classes.len());
    assert!(immutable_methods > 0, "Should have converted methods");
    assert!(immutable_fields > 0, "Should have converted fields");

    eprintln!("Converted {} classes, {} methods, {} fields to immutable",
        immutable_classes, immutable_methods, immutable_fields);
}

#[test]
fn test_builder_modify_real_method() {
    let apk_path = match get_test_apk_path() {
        Some(p) => p,
        None => { eprintln!("Skipping test_builder_modify_real_method: test APK not found at '{}'", TEST_APK_PATH); return; }
    };
    let dex_list = extract_classes_dex(&apk_path);
    assert!(!dex_list.is_empty());

    let dex = DexBackedDexFile::from_bytes(dex_list[0].clone())
        .expect("Failed to parse DEX");

    let classes = dex.parse_classes();
    let mut found_method_with_impl = false;

    for class in classes.iter().take(100) {
        for method in class.direct_methods().iter().chain(class.virtual_methods().iter()) {
            if let Some(impl_) = method.implementation() {
                let original_count = impl_.instructions().len();
                if original_count < 3 {
                    continue;
                }

                let mut mutable = MutableMethodImplementation::from_implementation(impl_);
                assert_eq!(mutable.register_count(), impl_.register_count());
                assert_eq!(mutable.instructions().len(), original_count);

                let first_opcode = mutable.instructions()[0].opcode();
                let last_opcode = mutable.instructions()[original_count - 1].opcode();

                mutable.add_instruction(1, make_nop());
                assert_eq!(mutable.instructions().len(), original_count + 1);
                assert_eq!(mutable.instructions()[1].opcode(), Opcode::Nop);
                assert_eq!(mutable.instructions()[0].opcode(), first_opcode);
                assert_eq!(mutable.instructions()[original_count].opcode(), last_opcode);

                mutable.replace_instruction(1, make_return_void());
                assert_eq!(mutable.instructions()[1].opcode(), Opcode::ReturnVoid);

                mutable.remove_instruction(1);
                assert_eq!(mutable.instructions().len(), original_count);

                found_method_with_impl = true;
                break;
            }
        }
        if found_method_with_impl {
            break;
        }
    }

    assert!(found_method_with_impl, "Should find a method with implementation");
    eprintln!("Builder test: successfully modified real method implementation");
}

#[test]
fn test_writer_roundtrip_real_apk() {
    let apk_path = match get_test_apk_path() {
        Some(p) => p,
        None => { eprintln!("Skipping test_writer_roundtrip_real_apk: test APK not found at '{}'", TEST_APK_PATH); return; }
    };
    let dex_list = extract_classes_dex(&apk_path);
    assert!(!dex_list.is_empty());

    let dex = DexBackedDexFile::from_bytes(dex_list[0].clone())
        .expect("Failed to parse DEX");

    let classes = dex.parse_classes();
    assert!(!classes.is_empty());

    let sample_count = classes.len().min(50);

    let immutable_classes: Vec<ImmutableClassDef> = classes.iter().take(sample_count).map(|class| {
        let mut icd = ImmutableClassDef::new(class.type_descriptor(), class.access_flags());
        icd.superclass = class.superclass().map(|s| s.to_string());
        icd.source_file = class.source_file().map(|s| s.to_string());

        for method in class.direct_methods() {
            let mut im = ImmutableMethod::new(
                method.defining_class(), method.name(),
                method.parameter_types().to_vec(), method.return_type(),
                method.access_flags(),
            );
            if let Some(imp) = method.implementation() {
                let instructions: Vec<_> = imp.instructions().iter()
                    .map(|i| dex.convert_instruction_to_immutable(i.as_ref()))
                    .collect();
                im.implementation = Some(ImmutableMethodImplementation {
                    register_count: imp.register_count(),
                    instructions,
                    try_blocks: imp.try_blocks().to_vec(),
                    debug_items: imp.debug_items().to_vec(),
                });
            }
            icd.direct_methods.push(Box::new(im));
        }
        for method in class.virtual_methods() {
            let mut im = ImmutableMethod::new(
                method.defining_class(), method.name(),
                method.parameter_types().to_vec(), method.return_type(),
                method.access_flags(),
            );
            if let Some(imp) = method.implementation() {
                let instructions: Vec<_> = imp.instructions().iter()
                    .map(|i| dex.convert_instruction_to_immutable(i.as_ref()))
                    .collect();
                im.implementation = Some(ImmutableMethodImplementation {
                    register_count: imp.register_count(),
                    instructions,
                    try_blocks: imp.try_blocks().to_vec(),
                    debug_items: imp.debug_items().to_vec(),
                });
            }
            icd.virtual_methods.push(Box::new(im));
        }
        for field in class.static_fields() {
            let f = ImmutableField::new(
                field.defining_class(), field.name(), field.field_type(), field.access_flags(),
            );
            icd.static_fields.push(Box::new(f));
        }
        for field in class.instance_fields() {
            let f = ImmutableField::new(
                field.defining_class(), field.name(), field.field_type(), field.access_flags(),
            );
            icd.instance_fields.push(Box::new(f));
        }
        icd
    }).collect();

    let mut pool = DexPool::new();
    for icd in &immutable_classes {
        pool.intern_class(icd);
    }

    assert_eq!(pool.class_pool.count(), sample_count as u32);
    assert!(pool.string_pool.count() > 0, "String pool should have entries");
    assert!(pool.type_pool.count() > 0, "Type pool should have entries");

    let written_bytes = pool.to_bytes().expect("Failed to write DEX");
    assert!(!written_bytes.is_empty());
    assert!(written_bytes.len() >= 0x70, "Written DEX should be at least header size");

    assert_eq!(&written_bytes[0..4], b"dex\n", "Written DEX should have valid magic");

    let reparsed = DexBackedDexFile::from_bytes(written_bytes)
        .expect("Failed to re-parse written DEX");

    assert_eq!(reparsed.class_count() as u32, sample_count as u32,
        "Re-parsed class count should match original");

    let reparsed_classes = reparsed.parse_classes();
    assert_eq!(reparsed_classes.len(), sample_count as usize);

    for (i, (orig, reparsed)) in immutable_classes.iter().zip(reparsed_classes.iter()).enumerate() {
        assert_eq!(reparsed.type_descriptor(), orig.type_descriptor(),
            "Class #{} type descriptor mismatch", i);
        assert_eq!(reparsed.access_flags(), orig.access_flags(),
            "Class #{} access flags mismatch", i);
    }

    eprintln!("Writer round-trip: {} classes, {} strings, {} types, {} protos, {} fields, {} methods, {} bytes",
        reparsed.class_count(),
        reparsed.string_count(),
        reparsed.type_count(),
        reparsed.proto_count(),
        reparsed.field_count(),
        reparsed.method_count(),
        reparsed.header().file_size);
}

#[test]
fn test_rewriter_with_real_apk() {
    let apk_path = match get_test_apk_path() {
        Some(p) => p,
        None => { eprintln!("Skipping test_rewriter_with_real_apk: test APK not found at '{}'", TEST_APK_PATH); return; }
    };
    let dex_list = extract_classes_dex(&apk_path);
    assert!(!dex_list.is_empty());

    let dex = DexBackedDexFile::from_bytes(dex_list[0].clone())
        .expect("Failed to parse DEX");

    let classes = dex.parse_classes();
    assert!(!classes.is_empty());

    let sample_count = classes.len().min(30);

    let immutable_classes: Vec<Box<dyn ClassDef>> = classes.iter().take(sample_count).map(|class| {
        let mut icd = ImmutableClassDef::new(class.type_descriptor(), class.access_flags());
        icd.superclass = class.superclass().map(|s| s.to_string());
        icd.source_file = class.source_file().map(|s| s.to_string());

        for method in class.direct_methods() {
            let mut im = ImmutableMethod::new(
                method.defining_class(), method.name(),
                method.parameter_types().to_vec(), method.return_type(),
                method.access_flags(),
            );
            if let Some(imp) = method.implementation() {
                let instructions: Vec<_> = imp.instructions().iter()
                    .map(|i| dex.convert_instruction_to_immutable(i.as_ref()))
                    .collect();
                im.implementation = Some(ImmutableMethodImplementation {
                    register_count: imp.register_count(),
                    instructions,
                    try_blocks: imp.try_blocks().to_vec(),
                    debug_items: imp.debug_items().to_vec(),
                });
            }
            icd.direct_methods.push(Box::new(im));
        }
        for method in class.virtual_methods() {
            let mut im = ImmutableMethod::new(
                method.defining_class(), method.name(),
                method.parameter_types().to_vec(), method.return_type(),
                method.access_flags(),
            );
            if let Some(imp) = method.implementation() {
                let instructions: Vec<_> = imp.instructions().iter()
                    .map(|i| dex.convert_instruction_to_immutable(i.as_ref()))
                    .collect();
                im.implementation = Some(ImmutableMethodImplementation {
                    register_count: imp.register_count(),
                    instructions,
                    try_blocks: imp.try_blocks().to_vec(),
                    debug_items: imp.debug_items().to_vec(),
                });
            }
            icd.virtual_methods.push(Box::new(im));
        }
        for field in class.static_fields() {
            let f = ImmutableField::new(
                field.defining_class(), field.name(), field.field_type(), field.access_flags(),
            );
            icd.static_fields.push(Box::new(f));
        }
        for field in class.instance_fields() {
            let f = ImmutableField::new(
                field.defining_class(), field.name(), field.field_type(), field.access_flags(),
            );
            icd.instance_fields.push(Box::new(f));
        }
        Box::new(icd) as Box<dyn ClassDef>
    }).collect();

    struct PassThrough;
    impl RewriterModule for PassThrough {}

    let module = PassThrough;
    let rewriter = DexRewriter::new(&module);

    let rewritten = rewriter.rewrite_classes(&immutable_classes);
    assert_eq!(rewritten.classes().len(), sample_count as usize);

    for (i, (orig, rewritten)) in immutable_classes.iter().zip(rewritten.classes().iter()).enumerate() {
        assert_eq!(rewritten.type_descriptor(), orig.type_descriptor(),
            "Class #{} type mismatch", i);
        assert_eq!(rewritten.access_flags(), orig.access_flags(),
            "Class #{} access flags mismatch", i);
        assert_eq!(rewritten.direct_methods().len(), orig.direct_methods().len(),
            "Class #{} direct method count mismatch", i);
        assert_eq!(rewritten.virtual_methods().len(), orig.virtual_methods().len(),
            "Class #{} virtual method count mismatch", i);
    }

    eprintln!("Rewriter pass-through: {} classes rewritten successfully", sample_count);
}

#[test]
fn test_smali_assemble_and_reparse() {
    let smali = r#"
.class public Lcom/test/SmaliTest;
.super Ljava/lang/Object;

.method public static hookSignature(Ljava/lang/String;I)V
    .registers 4
    const-string v0, "Hooked: "
    invoke-static {v0}, Lcom/Helper;->log(Ljava/lang/String;)V
    const v1, 0x10
    add-int/lit8 v2, v1, 0x5
    return-void
.end method

.method public doSomething()V
    .registers 2
    const-string v0, "Doing something"
    invoke-static {v0}, Lcom/Logger;->log(Ljava/lang/String;)V
    return-void
.end method
"#;

    let result = assemble_smali(smali);
    assert!(result.is_ok(), "Smali assembly should succeed");
    let dex_bytes = result.unwrap();
    assert!(dex_bytes.len() >= 0x70);
    assert_eq!(&dex_bytes[0..4], b"dex\n");

    let reparsed = DexBackedDexFile::from_bytes(dex_bytes.clone())
        .expect("Should re-parse assembled DEX");

    let classes = reparsed.parse_classes();
    assert_eq!(classes.len(), 1);
    assert_eq!(classes[0].type_descriptor(), "Lcom/test/SmaliTest;");
    assert_eq!(classes[0].superclass(), Some("Ljava/lang/Object;"));

    assert_eq!(classes[0].direct_methods().len(), 1);
    assert_eq!(classes[0].virtual_methods().len(), 1);

    let direct_method = &classes[0].direct_methods()[0];
    assert_eq!(direct_method.name(), "hookSignature");
    assert_eq!(direct_method.return_type(), "V");
    assert_eq!(direct_method.parameter_types().len(), 2);
    assert!(direct_method.implementation().is_some());

    let virtual_method = &classes[0].virtual_methods()[0];
    assert_eq!(virtual_method.name(), "doSomething");
    assert!(virtual_method.implementation().is_some());

    eprintln!("Smali assembly round-trip: 1 class, 2 methods, {} bytes", dex_bytes.len());
}