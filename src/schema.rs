use std::collections::HashMap;

pub struct DatabaseSchema {
    pub tables: HashMap<String, TableSchema>
}

pub struct TableSchema {
    pub columns: HashMap<String, ColumnType> 
}

pub enum ColumnType {
    Integer, 
    Float, 
    String,
    Boolean
}