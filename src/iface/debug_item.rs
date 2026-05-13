use std::fmt;

#[derive(Debug, Clone)]
pub enum DebugItem {
    StartLocal(StartLocal),
    EndLocal(EndLocal),
    RestartLocal(RestartLocal),
    PrologueEnd,
    EpilogueBegin,
    SetSourceFile(SetSourceFile),
    LineNumber(LineNumber),
}

#[derive(Debug, Clone)]
pub struct StartLocal {
    pub code_address: u32,
    pub register: u16,
    pub name: Option<String>,
    pub type_descriptor: Option<String>,
    pub signature: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EndLocal {
    pub code_address: u32,
    pub register: u16,
}

#[derive(Debug, Clone)]
pub struct RestartLocal {
    pub code_address: u32,
    pub register: u16,
}

#[derive(Debug, Clone)]
pub struct SetSourceFile {
    pub source_file: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LineNumber {
    pub code_address: u32,
    pub line: u32,
}

impl DebugItem {
    pub fn code_address(&self) -> u32 {
        match self {
            DebugItem::StartLocal(item) => item.code_address,
            DebugItem::EndLocal(item) => item.code_address,
            DebugItem::RestartLocal(item) => item.code_address,
            DebugItem::PrologueEnd => 0,
            DebugItem::EpilogueBegin => 0,
            DebugItem::SetSourceFile(_) => 0,
            DebugItem::LineNumber(item) => item.code_address,
        }
    }
}

impl fmt::Display for DebugItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugItem::StartLocal(item) => write!(f, ".locals v{}", item.register),
            DebugItem::EndLocal(item) => write!(f, ".end local v{}", item.register),
            DebugItem::RestartLocal(item) => write!(f, ".restart local v{}", item.register),
            DebugItem::PrologueEnd => write!(f, ".prologue"),
            DebugItem::EpilogueBegin => write!(f, ".epilogue"),
            DebugItem::SetSourceFile(item) => write!(f, ".source {:?}", item.source_file),
            DebugItem::LineNumber(item) => write!(f, ".line {}", item.line),
        }
    }
}
