mod db;
use rusqlite::Connection;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conn = db::create_connection().expect("Failed to connect to database");
    db::create_transactions_table(&conn).expect("Failed to create table");

    match args.get(1).map(String::as_str) {
        Some("add") if args.len() == 4 => {
            let time = &args[2];
            let amount: f64 = args[3].parse().expect("Amount must be a valid number");
            match db::insert_transaction(&conn, time, amount) {
                Ok(_) => println!("✅ Transaction added: {} - ${}", time, amount),
                Err(e) => eprintln!("❌ Failed to add transaction: {}", e),
            }
        }
        Some("avg") => {
            match db::get_rolling_avg_for_jan_31(&conn) {
                Ok(avg) => println!("📊 3-Day Rolling Average for 2021-01-31: ${}", avg),
                Err(e) => eprintln!("❌ Failed to calculate rolling average: {}", e),
            }
        }
        _ => {
            println!("Usage:");
            println!("  add <time> <amount> - Add a new transaction");
            println!("  avg                 - Get the 3-day rolling average for 2021-01-31");
        }
    }
}
