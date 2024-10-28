use rusqlite::{params, Connection, Result};

pub fn create_connection() -> Result<Connection> {
    let conn = Connection::open("database/my_database.db")?;
    Ok(conn)
}

pub fn create_transactions_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            transaction_time TIMESTAMP,
            transaction_amount FLOAT
        )",
        [],
    )?;
    Ok(())
}
pub fn insert_transaction(conn: &Connection, time: &str, amount: f64) -> Result<()> {
    conn.execute(
        "INSERT INTO transactions (transaction_time, transaction_amount) VALUES (?1, ?2)",
        params![time, amount],
    )?;
    Ok(())
}

// Calculate a  bit
pub fn get_rolling_avg_for_jan_31(conn: &Connection) -> Result<f64> {
    let mut stmt = conn.prepare(
        "
        WITH daily_totals AS (
            SELECT 
                DATE(transaction_time) AS transaction_date,
                SUM(transaction_amount) AS total_amount
            FROM transactions
            GROUP BY transaction_date
        ),
        rolling_avg AS (
            SELECT 
                transaction_date,
                total_amount,
                AVG(total_amount) OVER (
                    ORDER BY transaction_date 
                    ROWS BETWEEN 2 PRECEDING AND CURRENT ROW
                ) AS rolling_3_day_avg
            FROM daily_totals
        )
        SELECT round(rolling_3_day_avg, 2)
        FROM rolling_avg
        WHERE transaction_date < '2021-01-31';
        "
    )?;

    let avg: f64 = stmt.query_row([], |row| row.get(0))?;
    Ok(avg)
}
