# dexlib-oxide

> A pure Rust library for DEX (Dalvik Executable) file parsing, modification, and writing.
>
> **Author: ALBA**

---

## Overview

**dexlib-oxide** is a Rust library for DEX file processing, inspired by [smali/dexlib2](https://github.com/JesusFreke/smali). It provides full DEX file reading, instruction-level modification, and valid DEX binary writing.

### Features

- **Parsing** — Complete DEX file structure parsing from raw bytes
- **Instruction Decoding** — All 256 Dalvik opcodes across Format10x through Format51l
- **Rewriting** — Instruction-level modification via Rewriter pattern
- **Writing** — DexPool collector + sorting + serialization with SHA-1 and Adler32
- **Smali** — Built-in Smali assembler (Smali text → DEX)
- **Immutable/Mutable** — Full immutable data structures + MutableMethodImplementation builder

### Module Structure

```
src/
├── base/          # Primitive types: LEB128, MUTF-8, AccessFlags, Opcode
├── iface/         # Core trait interface layer
├── dexbacked/     # DEX binary parsing
├── immutable/     # Immutable data structures
├── builder/       # Mutable builder (instruction add/remove/modify)
├── rewriter/      # DEX rewriting engine
├── writer/        # DEX writer (DexPool + serialization)
└── smali/         # Smali assembler
```

---

## Quick Start

### Add Dependency

```toml
[dependencies]
dexlib-oxide = { git = "https://github.com/Alba-Xy/dexlib-oxide" }
```

### Parse a DEX File

```rust
use dexlib_oxide::dexbacked::DexBackedDexFile;

let dex_data = std::fs::read("classes.dex")?;
let dex = DexBackedDexFile::from_bytes(&dex_data)?;

println!("DEX version: {:?}", dex.header.magic);
println!("Classes: {}", dex.classes.len());
println!("Strings: {}", dex.strings.len());
```

### Iterate Classes and Methods

```rust
use dexlib_oxide::iface::ClassDef;

for class in &dex.classes {
    println!("Class: {}", class.type_descriptor());
    for method in class.methods() {
        println!("  Method: {}", method.name());
        if let Some(imp) = method.implementation() {
            println!("    Instructions: {}", imp.instructions().len());
        }
    }
}
```

### Modify and Write Back

```rust
use dexlib_oxide::rewriter::{DexRewriter, RewriterModule};
use dexlib_oxide::writer::DexPool;
use dexlib_oxide::iface::instruction::Instruction;

struct StringReplacer { old_str: String, new_str: String }

impl RewriterModule for StringReplacer {
    fn rewrite_instruction(&self, instr: &dyn Instruction) -> Box<dyn Instruction> {
        instr.clone_boxed()
    }
}

let rewriter = DexRewriter::new(&StringReplacer {
    old_str: "hello".into(), new_str: "world".into(),
});
let rewritten = rewriter.rewrite_dex_file(&dex);

let mut pool = DexPool::new(dex.opcodes());
for class in rewritten.classes() {
    pool.intern_class(class);
}
let mut store = dexlib_oxide::writer::MemoryDataStore::new();
pool.write_to(&mut store)?;
std::fs::write("output.dex", store.get_buffer())?;
```

### Smali Assembly

```rust
use dexlib_oxide::smali::assemble_smali;

let smali = r#"
.class public LHello;
.super Ljava/lang/Object;
.method public static main([Ljava/lang/String;)V
    .registers 2
    const-string v0, "Hello, dexlib-oxide!"
    return-void
.end method
"#;

let dex_bytes = assemble_smali(smali)?;
```

---

## Testing

The project includes 110 unit and integration tests. Integration tests require an APK file for real DEX data validation:

```
dexlibApk/app-release-unsigned.apk
```

Place a test APK at the above path, then run:

```bash
cargo test
```

---

## License

[Apache License 2.0](LICENSE)