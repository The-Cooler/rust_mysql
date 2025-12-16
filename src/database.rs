use mysql::*;
use mysql::prelude::*;
use anyhow::Result;

pub struct DatabaseConnection {
    pool: Pool,
}

impl DatabaseConnection {
    pub fn new(host: &str, port: u16, username: &str, password: &str) -> Result<Self> {
        let url = format!("mysql://{}:{}@{}:{}", username, password, host, port);
        let pool = Pool::new(url.as_str())?;
        Ok(DatabaseConnection { pool })
    }
    
    pub fn get_databases(&self) -> Result<Vec<String>> {
        let mut conn = self.pool.get_conn()?;
        let databases: Vec<String> = conn.query_map("SHOW DATABASES", |row: Row| {
            let db: String = row.get(0).unwrap_or_default();
            db
        })?;
        Ok(databases)
    }
    
    pub fn get_tables(&self, database: &str) -> Result<Vec<String>> {
        let mut conn = self.pool.get_conn()?;
        let query = format!("SHOW TABLES FROM `{}`", database);
        let tables: Vec<String> = conn.query_map(query, |row: Row| {
            let table: String = row.get(0).unwrap_or_default();
            table
        })?;
        Ok(tables)
    }
    
    pub fn get_table_data(&self, database: &str, table: &str) -> Result<(Vec<String>, Vec<Vec<String>>)> {
        let mut conn = self.pool.get_conn()?;
        
        // 获取列名
        let columns_query = format!("SELECT COLUMN_NAME FROM INFORMATION_SCHEMA.COLUMNS WHERE TABLE_SCHEMA = '{}' AND TABLE_NAME = '{}' ORDER BY ORDINAL_POSITION", database, table);
        let columns: Vec<String> = conn.query_map(columns_query, |row: Row| {
            let col: String = row.get(0).unwrap_or_default();
            col
        })?;
        
        // 获取数据
        let data_query = format!("SELECT * FROM `{}`.`{}` LIMIT 1000", database, table);
        let rows: Vec<Row> = conn.query(data_query)?;
        let mut data = Vec::new();
        
        for row in rows {
            let mut row_data = Vec::new();
            for i in 0..row.len() {
                let value: Option<String> = row.get(i);
                row_data.push(value.unwrap_or_else(|| "NULL".to_string()));
            }
            data.push(row_data);
        }
        
        Ok((columns, data))
    }
}

