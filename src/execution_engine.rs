use crate::ast::{
    ASTNode, ComparisonOperator, DeleteStatement, InsertStatement, SelectStatement, UpdateStatement, Value,
    OrderByClause, OrderDirection, Condition
};

use crate::datastore::{ DataStore };

pub struct ExecutionEngine {
    data_store: DataStore 
}

impl ExecutionEngine {
    pub fn new (data_store: DataStore) -> Self {
        ExecutionEngine { data_store }
    }

    pub fn execute(&mut self, ast: &ASTNode) -> Result<QueryResult, ExecutionError> {
        match ast {
            ASTNode::Select(stmt) => self.execute_select(stmt),
            ASTNode::Insert(stmt) => self.execute_insert(stmt),
            ASTNode::Update(stmt) => self.execute_update(stmt),
            ASTNode::Delete(stmt) => self.execute_delete(stmt)
        }
    }

    fn execute_select(&self, stmt: &SelectStatement) -> Result<QueryResult, ExecutionError> {
        let table = self.data_store.get_table(&stmt.table)
            .ok_or_else(|| ExecutionError::TableNotFound(stmt.table.clone()));

        let mut result_rows = Vec::new();

        for row in table.rows() {
            if self.evaluate_condition(&stmt.condition, row)? {
                let selected_values = self.project_columns(&stmt.columns, row)?;
                result_rows.push(Row { values: selected_values });
            }
        }

        if !stmt.order_by.is_empty() {
            self.apply_order_by(&mut result_rows, &stmt.order_by)?;
        }

        Ok(QueryResult::Select(result_rows))
    }

    fn execute_insert(&self, stmt: &InsertStatement) -> Result<QueryResult, ExecutionError> {
        let table = self.data_store.get_table_mut(&stmt.table)
            .ok_or_else(|| ExecutionError::TableNotFound(stmt.table.clone()));
        
        let new_row = Row { values: stmt.values.clone() };
        table.insert(new_row);

        Ok(QueryResult::Insert(1))
    }

    fn execute_update(&self, stmt: &UpdateStatement) -> Result<QueryResult, ExecutionError> {
        let table = self.data_store.get_table_mut(&stmt.table)
            .ok_or_else(|| ExecutionError::TableNotFound(stmt.table.clone()))?;

        let mut updated_count = 0;
        for row in table.rows_mut() {
            if self.evaluate_condition(&stmt.condition, row)? {
                for (column, value) in &stmt.updates {
                    let column_index = self.get_column.index(&stmt.table, column)?;
                    row.values[column_index] = value.clone();
                }
                updated_count += 1;
            }
        }

        Ok(QueryResult::Update(updated_count))
    }

    fn execute_delete(&self, stmt: &DeleteStatement) -> Result<QueryResult, ExecutionError> {
        let table = self.data_store.get_table_mut(&stmt.table)
            .ok_or_else(|| ExecutionError::TableNotFound(stmt.table.clone()))?;

        let initial_count = table.row_count();
        table.retain(|row| !self.evaluate_condition(&stmt.condition, row).unwrap_or(false));
        let deleted_count = initial_count - table.row_count();

        Ok(QueryResult::Delete(deleted_count))
    }

    fn evaluate_condition(&self, condition: &Option<Condition>, row: &Row) -> Result<bool, ExecutionError> {
        match condition {
            Some(Condition::Comparison(column, op, value)) => {
                let column_index = self.get_column_index(&stmt.table, column)?;
                let row_value = &row.values[column_index];
                match op {
                    ComparisonOperator::Equals => Ok(row_value == value),
                    ComparisonOperator::NotEquals => Ok(row_value != value),
                    ComparisonOperator::GreaterThan => Ok(row_value > value),
                    ComparisonOperator::GreaterEqualThan => Ok(row_value >= value),
                    ComparisonOperator::LessThan => Ok(row_value < value),
                    ComparisonOperator::LessEqualThan => Ok(row_value <= value)
                }
            }
        }
    }

    fn project_columns(&self, columns: &[String], row: &Row) -> Result<Vec<Value>, ExecutionError> {
        if columns.contains(&"*".to_string()) {
            Ok(row_values.clone())
        } else {
            columns.iter()
                .map(|col| self.get_column_value(&stmt.table, col, row))
                .collect()
        }
    }

    fn apply_order_by(&self, rows: &mut [Row], order_by: &[OrderByClause]) -> Result<(), ExecutionError> {
        rows.sort_by(|a, b| {
            for clause in order_by {
                let column_index = self.get_column_index(&stmt.table, &clause.column).unwrap();
                let cmp = a.values[column_index].cmp(&b.values[column_index]);
                if cmp != std::cmp::Ordering::Equal {
                    return if clause.order == OrderDirection::Asc { cmp } else { cmp.reverse() };
                }
            }
            std::cmp::Ordering::Equal 
        });
        Ok(())
    }
}

pub enum QueryResult {
    Select(Vec<Row>),
    Insert(usize),
    Update(usize),
    Delete(usize)
}

pub struct Row {
    pub values: Vec<Value>
}

pub enum ExecutionError {
    TableNotFound(String),
    ColumnNotFound(String),
    TypeMismatch(String)
}