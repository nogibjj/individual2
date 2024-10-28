mod db;
mod error;
use std::env;
use error::AppError;

fn main() -> Result<(), AppError> {
    let args: Vec<String> = env::args().collect();
    let conn = db::create_connection()?;
    db::create_transactions_table(&conn)?;

    match args.get(1).map(String::as_str) {
        Some("add") if args.len() == 4 => {
            let time = &args[2];
            let amount: f64 = args[3].parse()
                .map_err(|_| AppError::InvalidInput("Invalid amount".to_string()))?;
            db::insert_transaction(&conn, time, amount)?;
            println!("✅ Transaction added: {} - ${}", time, amount);
        }
        Some("update") if args.len() == 4 => {
            let id: i64 = args[2].parse()
                .map_err(|_| AppError::InvalidInput("Invalid ID".to_string()))?;
            let new_amount: f64 = args[3].parse()
                .map_err(|_| AppError::InvalidInput("Invalid amount".to_string()))?;
            db::update_transaction(&conn, id, new_amount)?;
            println!("✅ Transaction ID {} updated to ${}", id, new_amount);
        }
        Some("delete") if args.len() == 3 => {
            let id: i64 = args[2].parse()
                .map_err(|_| AppError::InvalidInput("Invalid ID".to_string()))?;
            db::delete_transaction(&conn, id)?;
            println!("✅ Transaction ID {} deleted", id);
        }
        Some("list") => {
            let transactions = db::list_transactions(&conn)?;
            for (id, time, amount) in transactions {
                println!("ID: {}, Time: {}, Amount: ${}", id, time, amount);
            }
        }
        _ => {
            println!("Usage:");
            println!("  add <time> <amount> - Add a new transaction");
            println!("  update <id> <amount> - Update a transaction's amount");
            println!("  delete <id> - Delete a transaction by ID");
            println!("  list - List all transactions");
        }
    }

    Ok(())  // Make sure the function returns Ok(())
}
