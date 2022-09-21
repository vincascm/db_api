use std::{borrow::Cow, collections::HashMap};

use anyhow::Result;
use heck::CamelCase;
use sqlx::MySqlPool;

use crate::config;

use super::{
    database::{Columns, Table},
    LiteralRustType,
};

const RUST_RESERVED: &[&str] = &["type"];

pub struct DbLiteral {
    pub rust_code: String,
    pub use_date: bool,
    pub use_time: bool,
    pub use_datetime: bool,
    pub use_decimal: bool,
    pub use_json: bool,
}

impl Columns {
    fn to_rust_code(
        &self,
        config: Option<&config::Column>,
        replace_type: Option<&HashMap<String, String>>,
    ) -> Result<DbLiteral> {
        let mut use_date = false;
        let mut use_time = false;
        let mut use_datetime = false;
        let mut use_decimal = false;
        let mut use_json = false;
        let data_type: LiteralRustType = self.column_type.parse()?;
        match &data_type {
            LiteralRustType::NaiveDateTime => use_datetime = true,
            LiteralRustType::NaiveDate => use_date = true,
            LiteralRustType::NaiveTime => use_time = true,
            LiteralRustType::Decimal => use_decimal = true,
            LiteralRustType::Json => use_json = true,
            _ => (),
        }
        let mut data_type: Cow<'_, str> = {
            let mut n = data_type.rust_type_name();
            if let Some(replace_type) = replace_type {
                if let Some(r) = replace_type.get(n) {
                    n = r;
                }
            }
            n.into()
        };
        if let Some(c) = config {
            if let Some(json_type) = &c.json_type {
                use_json = true;
                data_type = format!("Json<{}>", json_type).into();
            }
            if let Some(custom_type) = &c.custom_type {
                data_type = custom_type.into();
            }
        }
        if self.is_nullable == "YES" {
            data_type = format!("Option<{}>", data_type).into();
        }
        let column_name = if RUST_RESERVED.contains(&self.column_name.as_str()) {
            format!("r#{}", self.column_name)
        } else {
            self.column_name.to_string()
        };
        let rust_code = format!(
            "    /// {}\n    pub {}: {},",
            self.column_comment, column_name, data_type
        );
        Ok(DbLiteral {
            rust_code,
            use_date,
            use_time,
            use_datetime,
            use_decimal,
            use_json,
        })
    }
}

impl Table {
    pub async fn to_rust_code(&self, config: &config::Config, db: &MySqlPool) -> Result<DbLiteral> {
        let mut use_date = false;
        let mut use_time = false;
        let mut use_datetime = false;
        let mut use_decimal = false;
        let mut use_json = false;
        let mut c = Vec::new();
        let mut table_config = None;

        if let Some(database_config) = &config.config {
            if let Some(tables) = &database_config.tables {
                table_config = tables.get(&self.name);
            }
        }
        for i in &self.columns(db).await? {
            let mut column_config = None;
            if let Some(table_config) = table_config {
                if let Some(columns) = &table_config.columns {
                    column_config = columns.get(&i.column_name);
                }
            }
            let i = i.to_rust_code(column_config, config.replace_type.as_ref())?;
            if i.use_date {
                use_date = true;
            }
            if i.use_time {
                use_time = true;
            }
            if i.use_datetime {
                use_datetime = true;
            }
            if i.use_decimal {
                use_decimal = true;
            }
            if i.use_json {
                use_json = true;
            }
            c.push(i.rust_code);
        }
        let columns_str = c.join("\n");
        let rust_code = format!("\n/// {}\n#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]\npub struct {} {{\n{}\n}}", self.comment, self.name.to_camel_case(), columns_str);
        Ok(DbLiteral {
            rust_code,
            use_date,
            use_time,
            use_datetime,
            use_decimal,
            use_json,
        })
    }
}
