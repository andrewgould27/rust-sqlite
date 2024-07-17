use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub tables: HashMap<String, TableSchema>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub columns: HashMap<String, ColumnType> 
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ColumnType {
    Integer, 
    Float, 
    String,
    Boolean
}