use tempfile::TempDir; 
use std::path::Path; 

use rust_sqlite::datastore::DataStore;

fn setup_test_datastore() -> (DataStore, TempDir) {
    let temp_dir = TempDir::new().unwrap();
    let data_store = DataStore::new(temp_dir.path()).unwrap();
    (data_store, temp_dir)
}

#[cfg(test)]
mod tests {
    use rust_sqlite::schema::{TableSchema, ColumnType};

    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_create_table() {
        let (mut data_store, _temp_dir) = setup_test_datastore();
        let mut columns = HashMap::new();
        columns.insert("id".to_string(), ColumnType::Integer);
        columns.insert("name".to_string(), ColumnType::String);
        
        let schema = TableSchema { columns };
        assert!(data_store.create_table("users".to_string(), schema.clone()).is_ok());

        let result = data_store.create_table("users".to_string(), schema);
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_and_select() {
        let (mut data_store, _temp_dir) = setup_test_datastore();
        let mut columns = HashMap::new();
        columns.insert("id".to_string(), ColumnType::Integer);
        columns.insert("name".to_string(), ColumnType::String);

        let schema = TableSchema { columns };
        data_store.create_table("users".to_string(), schema).unwrap();

        let mut row = HashMap::new();
        row.insert("id".to_string(), "1".to_string());
        row.insert("name".to_string(), "Alice".to_string());

        assert!(data_store.insert_row("users", row).is_ok());

        let result = data_store.select("users", &["id".to_string(), "name".to_string()], None).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].get("id"), Some(&"1".to_string()));
        assert_eq!(result[0].get("name"), Some(&"Alice".to_string()));
    }
}