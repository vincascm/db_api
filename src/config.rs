use std::{collections::HashMap, fs::File};

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_url: String,
    pub exclude_table: Option<Vec<String>>,
    pub just_table: Option<Vec<String>>,
    pub replace_type: Option<HashMap<String, String>>,
    pub config: Option<Database>,
}

impl Config {
    pub fn from_file(f: &str) -> Result<Config> {
        let f = File::open(f).context("open config file failure")?;
        let c: Config = serde_yaml::from_reader(f).context("parse config file failure")?;
        Ok(c)
    }
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub tables: Option<HashMap<String, Table>>,
}

#[derive(Debug, Deserialize)]
pub struct Table {
    pub columns: Option<HashMap<String, Column>>,
}

#[derive(Debug, Deserialize)]
pub struct Column {
    pub json_type: Option<String>,
    pub custom_type: Option<String>,
}
