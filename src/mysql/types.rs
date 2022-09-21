use std::str::FromStr;

use anyhow::{Error, Result};

#[derive(Debug)]
pub enum LiteralRustType {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    VecU8,
    DateTimeLocal,
    NaiveDateTime,
    NaiveDate,
    NaiveTime,
    Decimal,
    Json,
}

impl LiteralRustType {
    pub fn rust_type_name(&self) -> &'static str {
        match self {
            LiteralRustType::Bool => "bool",
            LiteralRustType::I8 => "i8",
            LiteralRustType::I16 => "i16",
            LiteralRustType::I32 => "i32",
            LiteralRustType::I64 => "i64",
            LiteralRustType::U8 => "u8",
            LiteralRustType::U16 => "u16",
            LiteralRustType::U32 => "u32",
            LiteralRustType::U64 => "u64",
            LiteralRustType::F32 => "f32",
            LiteralRustType::F64 => "f64",
            LiteralRustType::String => "String",
            LiteralRustType::VecU8 => "Vec<u8>",
            LiteralRustType::DateTimeLocal => "DateTime<Local>",
            LiteralRustType::NaiveDateTime => "NaiveDateTime",
            LiteralRustType::NaiveDate => "NaiveDate",
            LiteralRustType::NaiveTime => "NaiveTime",
            LiteralRustType::Decimal => "Decimal",
            LiteralRustType::Json => "Json",
        }
    }
}

impl FromStr for LiteralRustType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_uppercase();
        let r = if s == "TINYINT(1)" {
            LiteralRustType::Bool
        } else if s.contains("TINYINT") {
            if s.contains("UNSIGNED") {
                LiteralRustType::U8
            } else {
                LiteralRustType::I8
            }
        } else if s.contains("SMALLINT") {
            if s.contains("UNSIGNED") {
                LiteralRustType::U16
            } else {
                LiteralRustType::I16
            }
        } else if s.contains("BIGINT") {
            if s.contains("UNSIGNED") {
                LiteralRustType::U64
            } else {
                LiteralRustType::I64
            }
        } else if s.contains("INT") {
            if s.contains("UNSIGNED") {
                LiteralRustType::U32
            } else {
                LiteralRustType::I32
            }
        } else if s.contains("FLOAT") {
            LiteralRustType::F32
        } else if s.contains("DOUBLE") {
            LiteralRustType::F64
        } else if s.contains("VARCHAR") || s.contains("CHAR") || s.contains("TEXT") {
            LiteralRustType::String
        } else if s.contains("VARBINARY") || s.contains("BINARY") || s.contains("BLOB") {
            LiteralRustType::VecU8
        } else if s.contains("TIMESTAMP") {
            LiteralRustType::DateTimeLocal
        } else if s.contains("DATETIME") {
            LiteralRustType::NaiveDateTime
        } else if s.contains("DATE") {
            LiteralRustType::NaiveDate
        } else if s.contains("TIME") {
            LiteralRustType::NaiveTime
        } else if s.contains("DECIMAL") {
            LiteralRustType::Decimal
        } else if s.contains("JSON") {
            LiteralRustType::Json
        } else {
            panic!("undefined type: {}", s)
        };
        Ok(r)
    }
}
