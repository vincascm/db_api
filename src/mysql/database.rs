use std::{collections::HashMap, fmt::Display};

use anyhow::{Context, Result};
use sqlx::{query_as, query_scalar, FromRow, MySqlPool};

use crate::config::Config;

pub struct Database;

impl Database {
    pub async fn generate_rust_code(config: &Config, db: &MySqlPool) -> Result<String> {
        let database_name: Option<String> = query_scalar("select database()").fetch_one(db).await?;
        let database_name = database_name.context("missing database in db_url")?;
        let mut db_tables = HashMap::new();
        for i in Database::tables(database_name, db).await? {
            db_tables.insert(i.name.to_string(), i);
        }
        let mut tables = Vec::new();
        match &config.just_table {
            Some(just_table) => {
                for name in just_table {
                    if let Some(item) = db_tables.get(name) {
                        tables.push(item);
                    }
                }
            }
            None => {
                for item in db_tables.values() {
                    if let Some(exclude_table) = &config.exclude_table {
                        if exclude_table.contains(&item.name) {
                            continue;
                        }
                    }
                    tables.push(item);
                }
            }
        }

        //let mut use_date = false;
        //let mut use_time = false;
        //let mut use_datetime = false;
        //let mut use_decimal = false;
        let mut use_json = false;
        let mut tables_str = Vec::new();
        for i in tables {
            let t = i.to_rust_code(config, db).await?;
            //if t.use_date {
            //use_date = true;
            //}
            //if t.use_time {
            //use_time = true;
            //}
            //if t.use_datetime {
            //use_datetime = true;
            //}
            //if t.use_decimal {
                //use_decimal = true;
            //}
            if t.use_json {
                use_json = true;
            }
            tables_str.push(t.rust_code);
        }
        let mut use_statement = vec!["use serde::{Serialize, Deserialize};", "use sqlx::FromRow;"];
        /*
        if use_date {
            use_statement.push("use chrono::NaiveDate;");
        }
        if use_time {
            use_statement.push("use chrono::NaiveTime;");
        }
        if use_datetime {
            use_statement.push("use chrono::{DateTime, Local, NaiveDateTime};");
        }
        if use_decimal {
            use_statement.push("use sqlx::types::Decimal;");
        }
        */
        let super_json = if use_json {
            use_statement.push("use sqlx::types::Json;");
            "\n\nuse super::json;"
        } else {
            ""
        };
        let use_statement = use_statement.join("\n");
        Ok(format!(
            "{}{}\n\n{}",
            use_statement,
            super_json,
            tables_str.join("\n")
        ))
    }

    pub async fn tables(
        database_name: impl AsRef<str> + Display,
        db: &MySqlPool,
    ) -> Result<Vec<Table>> {
        let q = format!("SELECT TABLE_SCHEMA, TABLE_NAME, TABLE_COMMENT FROM information_schema.TABLES WHERE TABLE_SCHEMA='{}'", database_name);
        Ok(query_as(&q).fetch_all(db).await?)
    }
}

#[derive(Debug, FromRow)]
pub struct Table {
    #[sqlx(rename = "TABLE_SCHEMA")]
    pub schema: String,
    #[sqlx(rename = "TABLE_NAME")]
    pub name: String,
    #[sqlx(rename = "TABLE_COMMENT")]
    pub comment: String,
}

impl Table {
    pub async fn columns(&self, db: &MySqlPool) -> Result<Vec<Columns>> {
        let q = format!("SELECT * FROM information_schema.COLUMNS WHERE TABLE_SCHEMA='{}' and TABLE_NAME='{}' order by ordinal_position", self.schema, self.name);
        Ok(query_as(&q).fetch_all(db).await?)
    }
}

#[derive(Debug, FromRow)]
pub struct Columns {
    #[sqlx(rename = "COLUMN_NAME")]
    pub column_name: String,
    #[sqlx(rename = "ORDINAL_POSITION")]
    pub ordinal_position: u64,
    #[sqlx(rename = "COLUMN_DEFAULT")]
    pub column_default: Option<String>,
    #[sqlx(rename = "IS_NULLABLE")]
    pub is_nullable: String,
    #[sqlx(rename = "DATA_TYPE")]
    pub data_type: String,
    #[sqlx(rename = "CHARACTER_MAXIMUM_LENGTH")]
    pub character_maximum_length: Option<u64>,
    #[sqlx(rename = "CHARACTER_OCTET_LENGTH")]
    pub character_octet_length: Option<u64>,
    #[sqlx(rename = "NUMERIC_PRECISION")]
    pub numeric_precision: Option<u64>,
    #[sqlx(rename = "NUMERIC_SCALE")]
    pub numeric_scale: Option<u64>,
    #[sqlx(rename = "DATETIME_PRECISION")]
    pub datetime_precision: Option<u64>,
    #[sqlx(rename = "CHARACTER_SET_NAME")]
    pub character_set_name: Option<String>,
    #[sqlx(rename = "COLLATION_NAME")]
    pub collation_name: Option<String>,
    #[sqlx(rename = "COLUMN_TYPE")]
    pub column_type: String,
    #[sqlx(rename = "COLUMN_KEY")]
    pub column_key: String,
    #[sqlx(rename = "EXTRA")]
    pub extra: String,
    #[sqlx(rename = "PRIVILEGES")]
    pub privileges: String,
    #[sqlx(rename = "COLUMN_COMMENT")]
    pub column_comment: String,
}
