use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub tables: HashMap<String, TableSchema>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableSchema {
    pub columns: HashMap<String, ColumnType> 
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ColumnType {
    Integer, 
    Float, 
    String,
    Boolean
}