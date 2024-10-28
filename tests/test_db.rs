use individual2::db;
use rusqlite::Connection;

#[test]
fn test_create_connection() {
    let conn = db::create_connection();
    assert!(conn.is_ok());
}

#[test]
fn test_insert_transaction() {
    let conn = db::create_connection().unwrap();
    let result = db::insert_transaction(&conn, "2021-01-31 00:09:01", 23.10);
    assert!(result.is_ok());
}

#[test]
fn test_list_transactions() {
    let conn = db::create_connection().unwrap();
    db::insert_transaction(&conn, "2021-01-31 00:09:01", 23.10).unwrap();
    let transactions = db::list_transactions(&conn).unwrap();
    assert!(transactions.len() > 0); // Ensure there is at least one transaction
}

#[test]
fn test_update_transaction() {
    let conn = db::create_connection().unwrap();
    db::insert_transaction(&conn, "2021-01-31 00:09:01", 23.10).unwrap();

    // Retrieve the ID of the inserted transaction
    let transactions = db::list_transactions(&conn).unwrap();
    let id = transactions[0].0; // Get the first transaction's ID

    // Update the inserted transaction
    let result = db::update_transaction(&conn, id, 100.00);
    assert!(result.is_ok(), "Failed to update transaction");

    // Verify the update
    let updated_transactions = db::list_transactions(&conn).unwrap();
    assert_eq!(updated_transactions[0].2, 100.00); // Check if the amount is updated
}


#[test]
fn test_delete_transaction() {
    let conn = db::create_connection().unwrap();

    db::insert_transaction(&conn, "2021-01-31 00:09:01", 23.10).unwrap();

    let transactions = db::list_transactions(&conn).unwrap();
    println!("Transactions before delete: {:?}", transactions);

 
    assert!(!transactions.is_empty(), "No transactions found to delete");

    let id = transactions[0].0;
    println!("Attempting to delete transaction with ID: {}", id);

    let delete_result = db::delete_transaction(&conn, id);
    assert!(delete_result.is_ok(), "Failed to delete transaction");

    let remaining_transactions = db::list_transactions(&conn).unwrap();
    println!("Transactions after delete: {:?}", remaining_transactions);

    assert!(
        remaining_transactions.iter().all(|(tid, _, _)| *tid != id),
        "Transaction with ID {} was not deleted",
        id
    );
}
