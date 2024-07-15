use crate::schema::{DatabaseSchema, TableSchema, ColumnType};
use crate::ast::{ASTNode, Condition, Value};
use crate::ast::{DeleteStatement, InsertStatement, SelectStatement, UpdateStatement};

pub struct SemanticAnalyzer {
    schema: DatabaseSchema 
}

impl SemanticAnalyzer {
    pub fn new(schema: DatabaseSchema) -> Self {
        SemanticAnalyzer { schema }
    }

    pub fn analyze(&self, ast: &ASTNode) -> Result<(), SemanticError> {
        match ast {
            ASTNode::Select(stmt) => self.analyze_select(stmt),
            ASTNode::Update(stmt) => self.analyze_update(stmt),
            ASTNode::Insert(stmt) => self.analyze_insert(stmt),
            ASTNode::Delete(stmt) => self.analyze_delete(stmt)
        }
    }

    pub fn analyze_select(&self, stmt: &SelectStatement) -> Result<(), SemanticError> {
        let table_schema = self.schema.tables.get(&stmt.table) 
            .ok_or(SemanticError::TableNotFound(stmt.table.clone()))?;

        for col in &stmt.columns {
            if col != "*" && !table_schema.columns.contains_key(col) {
                return Err(SemanticError::ColumnNotFound(col.clone()));
            }
        }

        if let Some(condition) = &stmt.condition {
            self.analyze_condition(condition, table_schema)?;
        }

        for order_by in &stmt.order_by {
            if !table_schema.columns.contains_key(&order_by.column) {
                return Err(SemanticError::ColumnNotFound(order_by.column.clone()))
            }
        }

        Ok(())
    }

    pub fn analyze_update(&self, stmt: &UpdateStatement) -> Result<(), SemanticError> {
        let table_schema = self.schema.tables.get(&stmt.table)
            .ok_or(SemanticError::TableNotFound(stmt.table.clone()))?;

        for update in &stmt.updates {
            let (col, val) = update;
            if !table_schema.columns.contains_key(col) {
                return Err(SemanticError::ColumnNotFound(col.clone()));
            }
        }

        if let Some(condition) = &stmt.condition {
            self.analyze_condition(condition, table_schema)?;
        }

        Ok(())
    }

    pub fn analyze_insert(&self, stmt: &InsertStatement) -> Result<(), SemanticError> {
        let table_schema = self.schema.tables.get(&stmt.table)
            .ok_or(SemanticError::TableNotFound(stmt.table.clone()));

        
        Ok(())
    }

    pub fn analyze_delete(&self, stmt: &DeleteStatement) -> Result<(), SemanticError> {
        Ok(())
    }

    pub fn analyze_condition(&self, condition: &Condition, table_schema: &TableSchema) -> Result<(), SemanticError> {
        match condition {
            Condition::Comparison(col, op, value) => {
                let col_type = table_schema.columns.get(col)
                    .ok_or(SemanticError::ColumnNotFound(col.clone()))?;

                match (col_type, value) {
                    (ColumnType::Integer, Value::Number(_)) => Ok(()),
                    (ColumnType::Float, Value::Number(_)) => Ok(()),
                    (ColumnType::String, Value::String(_)) => Ok(()),
                    _ => Err(SemanticError::TypeMismatch(col.clone()))
                }
            }
        }
    }
}

pub enum SemanticError {
    TableNotFound(String),
    ColumnNotFound(String),
    TypeMismatch(String)
}