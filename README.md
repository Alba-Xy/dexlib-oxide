# dexlib-oxide

> 纯 Rust 实现的 DEX（Dalvik Executable）文件解析、修改、写入库。
>
> **作者：ALBA**
  玩了几年dexlib2了 用rust重构一下 这款工具我自己的项目也在使用
---

## 简介

**dexlib-oxide** 是一个用 Rust 编写的 DEX 文件处理库，灵感来源于 [smali/dexlib2](https://github.com/JesusFreke/smali)。它提供了完整的 DEX 文件读取、指令级修改、以及合法 DEX 二进制写入能力。

### 核心能力

- **解析** — 从原始字节解析完整 DEX 文件结构（Header、String/Type/Proto/Field/Method IDs、Class Defs、Code Items）
- **指令解码** — 支持全部 256 条 Dalvik 操作码的解码，覆盖 Format10x ~ Format51l
- **重写** — 基于 Rewriter 模式的指令级修改（替换字符串引用、方法引用、字段引用等）
- **写入** — DexPool 收集器 + 排序 + 序列化，生成合法 DEX 二进制（含 SHA-1、Adler32）
- **Smali** — 内置 Smali 汇编器，支持 Smali 文本 → DEX 指令
- **不可变/可变** — 完整的 Immutable 数据结构 + MutableMethodImplementation 构建器

### 模块结构

```
src/
├── base/          # 基础类型：LEB128、MUTF-8、AccessFlags、Opcode
├── iface/         # 核心 trait 接口层
├── dexbacked/     # DEX 二进制解析
├── immutable/     # 不可变数据结构
├── builder/       # 可变构建器（指令增删改）
├── rewriter/      # DEX 重写引擎
├── writer/        # DEX 写入器（DexPool + 序列化）
└── smali/         # Smali 汇编器
```

---

## 快速开始

### 添加依赖

在 `Cargo.toml` 中：

```toml
[dependencies]
dexlib-oxide = { git = "https://github.com/Alba-Xy/dexlib-oxide" }
```

### 解析 DEX 文件

```rust
use dexlib_oxide::dexbacked::DexBackedDexFile;

let dex_data = std::fs::read("classes.dex")?;
let dex = DexBackedDexFile::from_bytes(&dex_data)?;

println!("DEX 版本: {:?}", dex.header.magic);
println!("类数量: {}", dex.classes.len());
println!("字符串数量: {}", dex.strings.len());
```

### 遍历类和方法

```rust
use dexlib_oxide::iface::ClassDef;

for class in &dex.classes {
    println!("类: {}", class.type_descriptor());
    for method in class.methods() {
        println!("  方法: {}", method.name());
        if let Some(imp) = method.implementation() {
            println!("    指令数: {}", imp.instructions().len());
        }
    }
}
```

### 修改 DEX 并写回

```rust
use dexlib_oxide::rewriter::{DexRewriter, RewriterModule};
use dexlib_oxide::writer::DexPool;
use dexlib_oxide::iface::instruction::Instruction;
use dexlib_oxide::base::opcode::Opcode;

struct StringReplacer {
    old_str: String,
    new_str: String,
}

impl RewriterModule for StringReplacer {
    fn rewrite_instruction(&self, instr: &dyn Instruction) -> Box<dyn Instruction> {
        // 在此处修改指令（替换字符串引用、方法引用等）
        instr.clone_boxed()
    }
}

// 重写 DEX
let rewriter = DexRewriter::new(&StringReplacer {
    old_str: "hello".into(),
    new_str: "world".into(),
});
let rewritten = rewriter.rewrite_dex_file(&dex);

// 写入新 DEX
let mut pool = DexPool::new(dex.opcodes());
for class in rewritten.classes() {
    pool.intern_class(class);
}
let mut store = dexlib_oxide::writer::MemoryDataStore::new();
pool.write_to(&mut store)?;
std::fs::write("output.dex", store.get_buffer())?;
```

### Smali 汇编

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

## 测试

项目包含 110 个单元测试和集成测试。集成测试需要提供一个 APK 文件用于真实 DEX 数据验证：

```
dexlibApk/app-release-unsigned.apk
```

将测试 APK 放入上述路径后运行：

```bash
cargo test
```

---

## 许可证

[Apache License 2.0](LICENSE)
