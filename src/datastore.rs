use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use csv::{ReaderBuilder, WriterBuilder};
use serde::{Deserialize, Serialize};


use crate::schema::{
    TableSchema,
    ColumnType
};

pub struct Row {
    pub values: Vec<String>
}

pub struct DataStore {
    data_directory: PathBuf,
    schemas: HashMap<String, TableSchema> 
}

impl DataStore {
    pub fn new<P: AsRef<Path>>(data_directory: P) -> io::Result<Self> {
        let data_dir = data_directory.as_ref().to_path_buf();
        std::fs::create_dir_all(&data_dir)?;

        let mut store = DataStore {
            data_directory: data_dir,
            schemas: HashMap::new()
        };

        store.load_schemas()?;
        Ok(store)
    }

    fn load_schemas(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn save_schemas(&mut self) -> io::Result<()> {
        Ok(())
    }

    pub fn create_table(&mut self, name: String, schema: TableSchema) -> io::Result<()> {
        Ok(())
    }

    pub fn insert_row(&mut self, table_name: &str, row: Row) -> io::Result<()> {
        Ok(())
    }

    pub fn select(&self, table_name: &str, columns: &[String], condition: Option<&dyn Fn(&Row) -> bool>) -> io::Result<Vec<Row>> {
        let mut result = Vec::new();
        Ok(result)
    }

    pub fn update(&mut self, table_name: &str, updates: &[(String, String)], condition: Option<&dyn Fn(&Row) -> bool>) -> io::Result<usize> {
        Ok(0)
    }

    pub fn delete(&mut self, table_name: &str, condition: Option<&dyn Fn(&Row) -> bool>) -> io::Result<usize> {
        Ok(0)
    }
}