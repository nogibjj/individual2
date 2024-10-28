use rusqlite::{params, Connection, Result};
use crate::error::AppError;

pub fn create_connection() -> Result<Connection, AppError> {
    let conn = Connection::open("database/my_database.db")?;
    Ok(conn)
}

pub fn create_transactions_table(conn: &Connection) -> Result<(), AppError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            transaction_time TIMESTAMP NOT NULL,
            transaction_amount FLOAT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn insert_transaction(conn: &Connection, time: &str, amount: f64) -> Result<(), AppError> {
    conn.execute(
        "INSERT INTO transactions (transaction_time, transaction_amount) VALUES (?1, ?2)",
        params![time, amount],
    )?;
    Ok(())
}

pub fn update_transaction(conn: &Connection, id: i64, new_amount: f64) -> Result<(), AppError> {
    let affected_rows = conn.execute(
        "UPDATE transactions SET transaction_amount = ?1 WHERE id = ?2",
        params![new_amount, id],
    )?;
    if affected_rows == 0 {
        return Err(AppError::InvalidInput(format!(
            "No transaction found with ID {}",
            id
        )));
    }
    Ok(())
}

pub fn delete_transaction(conn: &Connection, id: i64) -> Result<(), AppError> {
    let affected_rows = conn.execute("DELETE FROM transactions WHERE id = ?1", params![id])?;
    if affected_rows == 0 {
        return Err(AppError::InvalidInput(format!(
            "No transaction found with ID {}",
            id
        )));
    }
    Ok(())
}

pub fn list_transactions(conn: &Connection) -> Result<Vec<(i64, String, f64)>, AppError> {
    let mut stmt = conn.prepare("SELECT id, transaction_time, transaction_amount FROM transactions")?;
    let transactions = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(transactions)
}
