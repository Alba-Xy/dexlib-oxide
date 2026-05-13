use crate::base::access_flags::AccessFlags;

#[derive(Debug, Clone)]
pub struct SmaliClass {
    pub access_flags: AccessFlags,
    pub type_descriptor: String,
    pub superclass: Option<String>,
    pub interfaces: Vec<String>,
    pub source_file: Option<String>,
    pub methods: Vec<SmaliMethod>,
    pub fields: Vec<SmaliField>,
}

#[derive(Debug, Clone)]
pub struct SmaliMethod {
    pub access_flags: AccessFlags,
    pub name: String,
    pub return_type: String,
    pub parameter_types: Vec<String>,
    pub register_count: u16,
    pub instructions: Vec<SmaliInstruction>,
}

#[derive(Debug, Clone)]
pub struct SmaliField {
    pub access_flags: AccessFlags,
    pub name: String,
    pub field_type: String,
    pub initial_value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SmaliInstruction {
    Nop,
    ReturnVoid,
    ConstString { register: u8, value: String },
    Const { register: u8, value: i32 },
    ConstWide { register: u8, value: i64 },
    InvokeStatic { method_ref: String, registers: Vec<u8> },
    InvokeVirtual { method_ref: String, registers: Vec<u8> },
    InvokeDirect { method_ref: String, registers: Vec<u8> },
    InvokeSuper { method_ref: String, registers: Vec<u8> },
    InvokeInterface { method_ref: String, registers: Vec<u8> },
    MoveResult { register: u8 },
    MoveResultObject { register: u8 },
    MoveResultWide { register: u8 },
    Move { dest: u8, src: u8 },
    MoveWide { dest: u8, src: u8 },
    MoveObject { dest: u8, src: u8 },
    Goto { offset: i16 },
    IfEqz { register: u8, offset: i16 },
    IfNez { register: u8, offset: i16 },
    NewInstance { register: u8, type_ref: String },
    CheckCast { register: u8, type_ref: String },
    IGet { dest: u8, obj: u8, field_ref: String },
    IPut { src: u8, obj: u8, field_ref: String },
    SGet { dest: u8, field_ref: String },
    SPut { src: u8, field_ref: String },
    Return { register: u8 },
    ReturnObject { register: u8 },
    Throw { register: u8 },
    Label { name: String },
}

#[derive(Debug, Clone)]
pub struct SmaliParseError {
    pub line: usize,
    pub message: String,
}

impl std::fmt::Display for SmaliParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Smali parse error at line {}: {}", self.line, self.message)
    }
}

impl std::error::Error for SmaliParseError {}

pub fn parse_smali(text: &str) -> Result<SmaliClass, SmaliParseError> {
    let lines: Vec<&str> = text.lines().collect();
    let mut class = SmaliClass {
        access_flags: AccessFlags::empty(),
        type_descriptor: String::new(),
        superclass: None,
        interfaces: Vec::new(),
        source_file: None,
        methods: Vec::new(),
        fields: Vec::new(),
    };

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }

        if let Some(rest) = line.strip_prefix(".class ") {
            let (flags, descriptor) = parse_access_flags_and_descriptor(rest.trim());
            class.access_flags = flags;
            class.type_descriptor = descriptor;
        } else if let Some(rest) = line.strip_prefix(".super ") {
            class.superclass = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix(".implements ") {
            class.interfaces.push(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix(".source ") {
            class.source_file = Some(rest.trim().trim_matches('"').to_string());
        } else if line.starts_with(".method ") {
            let (method, end_idx) = parse_method(&lines, i)?;
            class.methods.push(method);
            i = end_idx;
        } else if line.starts_with(".field ") {
            let field = parse_field(line, i)?;
            class.fields.push(field);
        }

        i += 1;
    }

    Ok(class)
}

fn parse_access_flags_and_descriptor(text: &str) -> (AccessFlags, String) {
    let parts: Vec<&str> = text.split_whitespace().collect();
    let mut flags = AccessFlags::empty();
    let mut descriptor = String::new();

    for part in parts {
        match part {
            "public" => flags |= AccessFlags::ACC_PUBLIC,
            "private" => flags |= AccessFlags::ACC_PRIVATE,
            "protected" => flags |= AccessFlags::ACC_PROTECTED,
            "static" => flags |= AccessFlags::ACC_STATIC,
            "final" => flags |= AccessFlags::ACC_FINAL,
            "abstract" => flags |= AccessFlags::ACC_ABSTRACT,
            "synthetic" => flags |= AccessFlags::ACC_SYNTHETIC,
            "annotation" => flags |= AccessFlags::ACC_ANNOTATION,
            "interface" => flags |= AccessFlags::ACC_INTERFACE,
            "enum" => flags |= AccessFlags::ACC_ENUM,
            s if s.starts_with('L') || s.starts_with('[') => {
                descriptor = s.to_string();
            }
            _ => {}
        }
    }

    (flags, descriptor)
}

fn parse_method(lines: &[&str], start: usize) -> Result<(SmaliMethod, usize), SmaliParseError> {
    let header = lines[start].trim();
    let header_rest = header.strip_prefix(".method ").unwrap_or(header).trim();

    let (flags, method_signature) = parse_access_flags_and_signature(header_rest);
    let (name, return_type, parameter_types) = parse_method_signature(method_signature)?;

    let mut method = SmaliMethod {
        access_flags: flags,
        name,
        return_type,
        parameter_types,
        register_count: 0,
        instructions: Vec::new(),
    };

    let mut end_idx = start;
    for j in (start + 1)..lines.len() {
        let line = lines[j].trim();
        if line == ".end method" {
            end_idx = j;
            break;
        }
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(rest) = line.strip_prefix(".locals ") {
            if let Ok(count) = rest.trim().parse::<u16>() {
                method.register_count = count;
            }
            continue;
        }
        if let Some(rest) = line.strip_prefix(".registers ") {
            if let Ok(count) = rest.trim().parse::<u16>() {
                method.register_count = count;
            }
            continue;
        }
        if line.starts_with('.') {
            continue;
        }

        if let Some(instr) = parse_instruction(line, j)? {
            method.instructions.push(instr);
        }
    }

    Ok((method, end_idx))
}

fn parse_access_flags_and_signature(text: &str) -> (AccessFlags, &str) {
    let parts: Vec<&str> = text.split_whitespace().collect();
    let mut flags = AccessFlags::empty();
    let mut sig = "";

    for part in parts {
        match part {
            "public" => flags |= AccessFlags::ACC_PUBLIC,
            "private" => flags |= AccessFlags::ACC_PRIVATE,
            "protected" => flags |= AccessFlags::ACC_PROTECTED,
            "static" => flags |= AccessFlags::ACC_STATIC,
            "final" => flags |= AccessFlags::ACC_FINAL,
            "abstract" => flags |= AccessFlags::ACC_ABSTRACT,
            "synthetic" => flags |= AccessFlags::ACC_SYNTHETIC,
            "native" => flags |= AccessFlags::ACC_NATIVE,
            "synchronized" => flags |= AccessFlags::ACC_SYNCHRONIZED,
            "varargs" => flags |= AccessFlags::ACC_VARARGS,
            "constructor" => flags |= AccessFlags::ACC_CONSTRUCTOR,
            s if s.contains('(') => sig = s,
            _ => {}
        }
    }

    (flags, sig)
}

fn parse_method_signature(sig: &str) -> Result<(String, String, Vec<String>), SmaliParseError> {
    let paren_open = sig.find('(').ok_or_else(|| SmaliParseError {
        line: 0,
        message: format!("Invalid method signature: {}", sig),
    })?;
    let paren_close = sig.find(')').ok_or_else(|| SmaliParseError {
        line: 0,
        message: format!("Invalid method signature: {}", sig),
    })?;

    let name = sig[..paren_open].to_string();
    let params_str = &sig[paren_open + 1..paren_close];
    let return_type = sig[paren_close + 1..].to_string();

    let parameter_types = parse_type_list(params_str);

    Ok((name, return_type, parameter_types))
}

fn parse_type_list(types: &str) -> Vec<String> {
    let mut result = Vec::new();
    let chars: Vec<char> = types.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            'V' | 'Z' | 'B' | 'S' | 'C' | 'I' | 'J' | 'F' | 'D' => {
                result.push(chars[i].to_string());
                i += 1;
            }
            'L' => {
                let start = i;
                while i < chars.len() && chars[i] != ';' {
                    i += 1;
                }
                if i < chars.len() {
                    i += 1;
                }
                result.push(chars[start..i].iter().collect());
            }
            '[' => {
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] == '[' {
                    i += 1;
                }
                if i < chars.len() {
                    if chars[i] == 'L' {
                        while i < chars.len() && chars[i] != ';' {
                            i += 1;
                        }
                        if i < chars.len() {
                            i += 1;
                        }
                    } else {
                        i += 1;
                    }
                }
                result.push(chars[start..i].iter().collect());
            }
            _ => {
                i += 1;
            }
        }
    }

    result
}

fn parse_field(line: &str, line_num: usize) -> Result<SmaliField, SmaliParseError> {
    let rest = line.strip_prefix(".field ").unwrap_or(line).trim();

    let mut flags = AccessFlags::empty();
    let parts: Vec<&str> = rest.split_whitespace().collect();
    let mut field_desc_start = 0;

    for (idx, part) in parts.iter().enumerate() {
        match *part {
            "public" => flags |= AccessFlags::ACC_PUBLIC,
            "private" => flags |= AccessFlags::ACC_PRIVATE,
            "protected" => flags |= AccessFlags::ACC_PROTECTED,
            "static" => flags |= AccessFlags::ACC_STATIC,
            "final" => flags |= AccessFlags::ACC_FINAL,
            "volatile" => flags |= AccessFlags::ACC_VOLATILE,
            "transient" => flags |= AccessFlags::ACC_TRANSIENT,
            "synthetic" => flags |= AccessFlags::ACC_SYNTHETIC,
            "enum" => flags |= AccessFlags::ACC_ENUM,
            _ => {
                field_desc_start = idx;
                break;
            }
        }
    }

    if field_desc_start >= parts.len() {
        return Err(SmaliParseError {
            line: line_num,
            message: format!("Invalid field declaration: {}", line),
        });
    }

    let field_desc = parts[field_desc_start..].join(" ");
    let colon_pos = field_desc.find(':').ok_or_else(|| SmaliParseError {
        line: line_num,
        message: format!("Invalid field declaration: {}", line),
    })?;

    let name = field_desc[..colon_pos].trim().to_string();
    let type_and_init: Vec<&str> = field_desc[colon_pos + 1..].trim().splitn(2, " = ").collect();
    let field_type = type_and_init[0].trim().to_string();
    let initial_value = type_and_init.get(1).map(|s| s.trim().to_string());

    Ok(SmaliField {
        access_flags: flags,
        name,
        field_type,
        initial_value,
    })
}

fn parse_instruction(line: &str, line_num: usize) -> Result<Option<SmaliInstruction>, SmaliParseError> {
    let line = line.split('#').next().unwrap_or(line).trim();
    if line.is_empty() {
        return Ok(None);
    }

    let parts: Vec<&str> = line.splitn(2, ' ').collect();
    let opcode_str = parts[0];
    let operands_str = parts.get(1).map(|s| s.trim()).unwrap_or("");

    let instr = match opcode_str {
        "nop" => SmaliInstruction::Nop,
        "return-void" => SmaliInstruction::ReturnVoid,
        "const-string" => {
            let (reg, value) = parse_const_string_operands(operands_str, line_num)?;
            SmaliInstruction::ConstString { register: reg, value }
        }
        "const" | "const/4" | "const/16" | "const/high16" => {
            let (reg, val) = parse_register_literal(operands_str, line_num)?;
            SmaliInstruction::Const { register: reg, value: val }
        }
        "const-wide" | "const-wide/16" | "const-wide/32" | "const-wide/high16" => {
            let (reg, val) = parse_register_literal_wide(operands_str, line_num)?;
            SmaliInstruction::ConstWide { register: reg, value: val }
        }
        "invoke-static" => {
            let (regs, method_ref) = parse_invoke_operands(operands_str, line_num)?;
            SmaliInstruction::InvokeStatic { method_ref, registers: regs }
        }
        "invoke-virtual" => {
            let (regs, method_ref) = parse_invoke_operands(operands_str, line_num)?;
            SmaliInstruction::InvokeVirtual { method_ref, registers: regs }
        }
        "invoke-direct" => {
            let (regs, method_ref) = parse_invoke_operands(operands_str, line_num)?;
            SmaliInstruction::InvokeDirect { method_ref, registers: regs }
        }
        "invoke-super" => {
            let (regs, method_ref) = parse_invoke_operands(operands_str, line_num)?;
            SmaliInstruction::InvokeSuper { method_ref, registers: regs }
        }
        "invoke-interface" => {
            let (regs, method_ref) = parse_invoke_operands(operands_str, line_num)?;
            SmaliInstruction::InvokeInterface { method_ref, registers: regs }
        }
        "move-result" => {
            let reg = parse_single_register(operands_str, line_num)?;
            SmaliInstruction::MoveResult { register: reg }
        }
        "move-result-object" => {
            let reg = parse_single_register(operands_str, line_num)?;
            SmaliInstruction::MoveResultObject { register: reg }
        }
        "move-result-wide" => {
            let reg = parse_single_register(operands_str, line_num)?;
            SmaliInstruction::MoveResultWide { register: reg }
        }
        "move" | "move/from16" | "move/16" => {
            let (dest, src) = parse_two_registers(operands_str, line_num)?;
            SmaliInstruction::Move { dest, src }
        }
        "move-wide" | "move-wide/from16" | "move-wide/16" => {
            let (dest, src) = parse_two_registers(operands_str, line_num)?;
            SmaliInstruction::MoveWide { dest, src }
        }
        "move-object" | "move-object/from16" | "move-object/16" => {
            let (dest, src) = parse_two_registers(operands_str, line_num)?;
            SmaliInstruction::MoveObject { dest, src }
        }
        "goto" | "goto/16" | "goto/32" => {
            let offset = parse_offset(operands_str, line_num)?;
            SmaliInstruction::Goto { offset }
        }
        "if-eqz" => {
            let (reg, offset) = parse_if_reg_offset(operands_str, line_num)?;
            SmaliInstruction::IfEqz { register: reg, offset }
        }
        "if-nez" => {
            let (reg, offset) = parse_if_reg_offset(operands_str, line_num)?;
            SmaliInstruction::IfNez { register: reg, offset }
        }
        "new-instance" => {
            let (reg, type_ref) = parse_reg_type(operands_str, line_num)?;
            SmaliInstruction::NewInstance { register: reg, type_ref }
        }
        "check-cast" => {
            let (reg, type_ref) = parse_reg_type(operands_str, line_num)?;
            SmaliInstruction::CheckCast { register: reg, type_ref }
        }
        "iget" | "iget-wide" | "iget-object" | "iget-boolean" | "iget-byte"
        | "iget-char" | "iget-short" => {
            let (dest, obj, field_ref) = parse_field_access_operands(operands_str, line_num)?;
            SmaliInstruction::IGet { dest, obj, field_ref }
        }
        "iput" | "iput-wide" | "iput-object" | "iput-boolean" | "iput-byte"
        | "iput-char" | "iput-short" => {
            let (src, obj, field_ref) = parse_field_access_operands(operands_str, line_num)?;
            SmaliInstruction::IPut { src, obj, field_ref }
        }
        "sget" | "sget-wide" | "sget-object" | "sget-boolean" | "sget-byte"
        | "sget-char" | "sget-short" => {
            let (reg, field_ref) = parse_sfield_access_operands(operands_str, line_num)?;
            SmaliInstruction::SGet { dest: reg, field_ref }
        }
        "sput" | "sput-wide" | "sput-object" | "sput-boolean" | "sput-byte"
        | "sput-char" | "sput-short" => {
            let (reg, field_ref) = parse_sfield_access_operands(operands_str, line_num)?;
            SmaliInstruction::SPut { src: reg, field_ref }
        }
        "return" => {
            let reg = parse_single_register(operands_str, line_num)?;
            SmaliInstruction::Return { register: reg }
        }
        "return-object" => {
            let reg = parse_single_register(operands_str, line_num)?;
            SmaliInstruction::ReturnObject { register: reg }
        }
        "throw" => {
            let reg = parse_single_register(operands_str, line_num)?;
            SmaliInstruction::Throw { register: reg }
        }
        s if s.starts_with(':') => {
            SmaliInstruction::Label { name: s[1..].to_string() }
        }
        _ => {
            return Ok(None);
        }
    };

    Ok(Some(instr))
}

fn parse_const_string_operands(text: &str, line_num: usize) -> Result<(u8, String), SmaliParseError> {
    let comma_pos = text.find(',').ok_or_else(|| SmaliParseError {
        line: line_num,
        message: "const-string: expected comma after register".to_string(),
    })?;

    let reg_str = text[..comma_pos].trim();
    let reg = parse_vreg(reg_str, line_num)?;

    let value_str = text[comma_pos + 1..].trim();
    let value = if value_str.starts_with('"') && value_str.ends_with('"') {
        value_str[1..value_str.len() - 1].to_string()
    } else {
        value_str.to_string()
    };

    Ok((reg, value))
}

fn parse_register_literal(text: &str, line_num: usize) -> Result<(u8, i32), SmaliParseError> {
    let parts: Vec<&str> = text.splitn(2, ',').collect();
    if parts.len() < 2 {
        return Err(SmaliParseError {
            line: line_num,
            message: "const: expected register, literal".to_string(),
        });
    }

    let reg = parse_vreg(parts[0].trim(), line_num)?;
    let literal_str = parts[1].trim();
    let literal = if literal_str.starts_with("0x") || literal_str.starts_with("0X") {
        i32::from_str_radix(&literal_str[2..], 16).unwrap_or(0)
    } else if literal_str.starts_with('-') {
        literal_str.parse::<i32>().unwrap_or(0)
    } else {
        literal_str.parse::<i32>().unwrap_or(0)
    };

    Ok((reg, literal))
}

fn parse_register_literal_wide(text: &str, line_num: usize) -> Result<(u8, i64), SmaliParseError> {
    let parts: Vec<&str> = text.splitn(2, ',').collect();
    if parts.len() < 2 {
        return Err(SmaliParseError {
            line: line_num,
            message: "const-wide: expected register, literal".to_string(),
        });
    }

    let reg = parse_vreg(parts[0].trim(), line_num)?;
    let literal_str = parts[1].trim();
    let literal = if literal_str.starts_with("0x") || literal_str.starts_with("0X") {
        i64::from_str_radix(&literal_str[2..], 16).unwrap_or(0)
    } else {
        literal_str.parse::<i64>().unwrap_or(0)
    };

    Ok((reg, literal))
}

fn parse_invoke_operands(text: &str, line_num: usize) -> Result<(Vec<u8>, String), SmaliParseError> {
    let brace_open = text.find('{').ok_or_else(|| SmaliParseError {
        line: line_num,
        message: "invoke: expected {".to_string(),
    })?;
    let brace_close = text.find('}').ok_or_else(|| SmaliParseError {
        line: line_num,
        message: "invoke: expected }".to_string(),
    })?;

    let regs_str = &text[brace_open + 1..brace_close];
    let mut registers = Vec::new();
    for reg_str in regs_str.split(',') {
        let trimmed = reg_str.trim();
        if !trimmed.is_empty() {
            registers.push(parse_vreg(trimmed, line_num)?);
        }
    }

    let after_brace = text[brace_close + 1..].trim();
    let method_ref = after_brace.strip_prefix(", ").unwrap_or(after_brace).to_string();

    Ok((registers, method_ref))
}

fn parse_single_register(text: &str, line_num: usize) -> Result<u8, SmaliParseError> {
    parse_vreg(text.trim(), line_num)
}

fn parse_two_registers(text: &str, line_num: usize) -> Result<(u8, u8), SmaliParseError> {
    let parts: Vec<&str> = text.splitn(2, ',').collect();
    if parts.len() < 2 {
        return Err(SmaliParseError {
            line: line_num,
            message: "Expected two registers".to_string(),
        });
    }
    let dest = parse_vreg(parts[0].trim(), line_num)?;
    let src = parse_vreg(parts[1].trim(), line_num)?;
    Ok((dest, src))
}

fn parse_offset(text: &str, line_num: usize) -> Result<i16, SmaliParseError> {
    let trimmed = text.trim();
    if trimmed.starts_with(':') {
        return Ok(0);
    }
    trimmed.parse::<i16>().map_err(|_| SmaliParseError {
        line: line_num,
        message: format!("Invalid offset: {}", trimmed),
    })
}

fn parse_if_reg_offset(text: &str, line_num: usize) -> Result<(u8, i16), SmaliParseError> {
    let parts: Vec<&str> = text.splitn(2, ',').collect();
    if parts.len() < 2 {
        return Err(SmaliParseError {
            line: line_num,
            message: "if-*z: expected register, offset".to_string(),
        });
    }
    let reg = parse_vreg(parts[0].trim(), line_num)?;
    let offset_str = parts[1].trim();
    let offset = if offset_str.starts_with(':') {
        0
    } else {
        offset_str.parse::<i16>().unwrap_or(0)
    };
    Ok((reg, offset))
}

fn parse_reg_type(text: &str, line_num: usize) -> Result<(u8, String), SmaliParseError> {
    let parts: Vec<&str> = text.splitn(2, ',').collect();
    if parts.len() < 2 {
        return Err(SmaliParseError {
            line: line_num,
            message: "Expected register, type".to_string(),
        });
    }
    let reg = parse_vreg(parts[0].trim(), line_num)?;
    let type_ref = parts[1].trim().to_string();
    Ok((reg, type_ref))
}

fn parse_field_access_operands(text: &str, line_num: usize) -> Result<(u8, u8, String), SmaliParseError> {
    let parts: Vec<&str> = text.splitn(3, ',').collect();
    if parts.len() < 3 {
        return Err(SmaliParseError {
            line: line_num,
            message: "Expected dest, obj, field_ref".to_string(),
        });
    }
    let dest = parse_vreg(parts[0].trim(), line_num)?;
    let obj = parse_vreg(parts[1].trim(), line_num)?;
    let field_ref = parts[2].trim().to_string();
    Ok((dest, obj, field_ref))
}

fn parse_sfield_access_operands(text: &str, line_num: usize) -> Result<(u8, String), SmaliParseError> {
    let parts: Vec<&str> = text.splitn(2, ',').collect();
    if parts.len() < 2 {
        return Err(SmaliParseError {
            line: line_num,
            message: "Expected register, field_ref".to_string(),
        });
    }
    let reg = parse_vreg(parts[0].trim(), line_num)?;
    let field_ref = parts[1].trim().to_string();
    Ok((reg, field_ref))
}

fn parse_vreg(text: &str, line_num: usize) -> Result<u8, SmaliParseError> {
    let trimmed = text.trim();
    if let Some(num_str) = trimmed.strip_prefix('v') {
        num_str.parse::<u8>().map_err(|_| SmaliParseError {
            line: line_num,
            message: format!("Invalid register: {}", trimmed),
        })
    } else if let Some(num_str) = trimmed.strip_prefix('p') {
        num_str.parse::<u8>().map_err(|_| SmaliParseError {
            line: line_num,
            message: format!("Invalid register: {}", trimmed),
        })
    } else {
        trimmed.parse::<u8>().map_err(|_| SmaliParseError {
            line: line_num,
            message: format!("Invalid register: {}", trimmed),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_class() {
        let smali = r#"
.class public Lcom/test/HelloWorld;
.super Ljava/lang/Object;

.method public static main()V
    .registers 2
    const-string v0, "Hello"
    return-void
.end method
"#;
        let class = parse_smali(smali).unwrap();
        assert_eq!(class.type_descriptor, "Lcom/test/HelloWorld;");
        assert_eq!(class.superclass, Some("Ljava/lang/Object;".to_string()));
        assert!(class.access_flags.contains(AccessFlags::ACC_PUBLIC));
        assert_eq!(class.methods.len(), 1);
        assert_eq!(class.methods[0].name, "main");
        assert_eq!(class.methods[0].register_count, 2);
        assert_eq!(class.methods[0].instructions.len(), 2);
    }

    #[test]
    fn test_parse_const_string() {
        let smali = r#"
.class public Lcom/Test;
.super Ljava/lang/Object;

.method public foo()V
    .registers 1
    const-string v0, "hooked"
    return-void
.end method
"#;
        let class = parse_smali(smali).unwrap();
        let instr = &class.methods[0].instructions[0];
        match instr {
            SmaliInstruction::ConstString { register, value } => {
                assert_eq!(*register, 0);
                assert_eq!(value, "hooked");
            }
            _ => panic!("Expected ConstString instruction"),
        }
    }

    #[test]
    fn test_parse_invoke_static() {
        let smali = r#"
.class public Lcom/Test;
.super Ljava/lang/Object;

.method public foo()V
    .registers 2
    invoke-static {}, Lcom/Helper;->doStuff()V
    return-void
.end method
"#;
        let class = parse_smali(smali).unwrap();
        let instr = &class.methods[0].instructions[0];
        match instr {
            SmaliInstruction::InvokeStatic { method_ref, registers } => {
                assert_eq!(method_ref, "Lcom/Helper;->doStuff()V");
                assert!(registers.is_empty());
            }
            _ => panic!("Expected InvokeStatic instruction"),
        }
    }

    #[test]
    fn test_parse_field() {
        let smali = r#"
.class public Lcom/Test;
.super Ljava/lang/Object;

.field public static NAME:Ljava/lang/String;
"#;
        let class = parse_smali(smali).unwrap();
        assert_eq!(class.fields.len(), 1);
        assert_eq!(class.fields[0].name, "NAME");
        assert_eq!(class.fields[0].field_type, "Ljava/lang/String;");
    }

    #[test]
    fn test_parse_method_signature() {
        let (name, ret, params) = parse_method_signature("hookSignature()V").unwrap();
        assert_eq!(name, "hookSignature");
        assert_eq!(ret, "V");
        assert!(params.is_empty());

        let (name, ret, params) = parse_method_signature("foo(IJLjava/lang/String;)V").unwrap();
        assert_eq!(name, "foo");
        assert_eq!(ret, "V");
        assert_eq!(params, vec!["I", "J", "Ljava/lang/String;"]);
    }

    #[test]
    fn test_parse_implements() {
        let smali = r#"
.class public Lcom/Test;
.super Ljava/lang/Object;
.implements Ljava/io/Serializable;
"#;
        let class = parse_smali(smali).unwrap();
        assert_eq!(class.interfaces, vec!["Ljava/io/Serializable;"]);
    }
}
