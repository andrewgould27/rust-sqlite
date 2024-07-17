use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use csv::{ReaderBuilder, WriterBuilder };
use tempfile::NamedTempFile;

use crate::schema::TableSchema;

pub struct Row {
    pub values: Vec<String>
}

pub enum ColumnType {
    Integer, 
    Float,
    String, 
    Boolean
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
        let schema_file = self.data_directory.join("schemas.json");
        if schema_file.exists() {
            let file = File::open(schema_file)?;
            self.schemas = serde_json::from_reader(file)?;
        }
        Ok(())
    }

    fn save_schemas(&mut self) -> io::Result<()> {
        let schema_file = self.data_directory.join("schemas.json");
        let file = File::create(schema_file)?;
        serde_json::to_writer(file, &self.schemas)?;
        Ok(())
    }

    pub fn create_table(&mut self, name: String, schema: TableSchema) -> io::Result<()> {
        if self.schemas.contains_key(&name) {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Table already exists"));
        }

        let data_file_path = self.data_directory.join(format!("{}.csv", name));
        let data_file = File::create(data_file_path)?;
        let mut writer = WriterBuilder::new().from_writer(data_file);
        writer.write_record(schema.columns.keys())?;
        
        self.schemas.insert(name, schema);
        self.save_schemas()?;
        Ok(())
    }

    pub fn insert_row(&mut self, table_name: &str, row: HashMap<String, String>) -> io::Result<()> {
        let table_schema = self.schemas.get(table_name)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Table not found"))?;
        
        if row.len() != table_schema.columns.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Row does not match schema"));
        }
        
        let file_path = self.data_directory.join(format!("{}.csv", table_name));
        let file = OpenOptions::new().append(true).open(file_path)?;
        let mut writer = WriterBuilder::new().from_writer(file);
        
        let record: Vec<String> = table_schema.columns.keys()
            .map(|col| row.get(col).cloned().unwrap_or_default())
            .collect();
        
        writer.write_record(&record)?;
        
        Ok(())
    }

    pub fn select(&self, table_name: &str, columns: &[String], condition: Option<&dyn Fn(&HashMap<String, String>) -> bool>) -> io::Result<Vec<HashMap<String, String>>> {
        let _table_schema = self.schemas.get(table_name)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Table not found"))?;
        
        let file_path = self.data_directory.join(format!("{}.csv", table_name));
        let file = File::open(file_path)?;
        let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file));

        let headers: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();

        let mut result = Vec::new();
        for record in reader.records() {
            let record = record?;
            let row: HashMap<String, String> = headers.iter().zip(record.iter())
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect();
            
            if condition.map_or(true, |cond| cond(&row)) {
                let selected_row: HashMap<String, String> = columns.iter()
                    .filter_map(|col| row.get(col).map(|val| (col.clone(), val.clone())))
                    .collect();
                result.push(selected_row);
            }
        }

        Ok(result)
    }

    pub fn update(&mut self, table_name: &str, updates: HashMap<String, String>, condition: impl Fn(&HashMap<String, String>) -> bool) -> io::Result<usize> {
        let _table_schema = self.schemas.get(table_name)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Table not found"))?;
        
        let file_path = self.data_directory.join(format!("{}.csv", table_name));
        let file = File::open(&file_path)?;
        let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file));
        
        let headers: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();
        
        let temp_file = NamedTempFile::new_in(&self.data_directory)?;
        let mut writer = WriterBuilder::new().from_writer(BufWriter::new(&temp_file));
        
        writer.write_record(&headers)?;
        
        let mut updated_count = 0;
        for result in reader.records() {
            let record = result?;
            let mut row: HashMap<String, String> = headers.iter()
                .zip(record.iter())
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect();
            
            if condition(&row) {
                for (col, value) in &updates {
                    if let Some(_index) = headers.iter().position(|h| h == col) {
                        row.insert(col.clone(), value.clone());
                    }
                }
                updated_count += 1;
            }
            
            writer.write_record(&record)?;
        }
        
        drop(writer);

        std::fs::rename(temp_file.path(), &file_path)?;
        
        Ok(updated_count)
    }

    pub fn delete(&mut self, table_name: &str, condition: impl Fn(&HashMap<String, String>) -> bool) -> io::Result<usize> {
        let _table_schema = self.schemas.get(table_name)
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Table not found"))?;
        
        let file_path = self.data_directory.join(format!("{}.csv", table_name));
        let file = File::open(&file_path)?;
        let mut reader = ReaderBuilder::new().from_reader(BufReader::new(file));
        
        let headers: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();
        
        let temp_file = NamedTempFile::new_in(&self.data_directory)?;
        let mut writer = WriterBuilder::new().from_writer(BufWriter::new(&temp_file));
        
        writer.write_record(&headers)?;
        
        let mut deleted_count = 0;
        for result in reader.records() {
            let record = result?;
            let row: HashMap<String, String> = headers.iter()
                .zip(record.iter())
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect();
            
            if !condition(&row) {
                writer.write_record(&record)?;
            } else {
                deleted_count += 1;
            }
        }
        
        drop(writer);

        std::fs::rename(temp_file.path(), &file_path)?;
        
        Ok(deleted_count)
    }

    pub fn get_table_schema(&self, table_name: &str) -> Option<&TableSchema> {
        self.schemas.get(table_name)
    }

    pub fn get_table_schema_mut(&mut self, table_name: &str) -> Option<&mut TableSchema> {
        self.schemas.get_mut(table_name)
    }

    pub fn table_exists(&self, table_name: &str) -> bool {
        self.schemas.contains_key(table_name)
    }
}