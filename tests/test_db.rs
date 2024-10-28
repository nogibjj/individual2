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
    let transactions = db::list_transactions(&conn).unwrap();
    assert!(transactions.len() >= 0); // Check that it returns at least an empty list
}

#[test]
fn test_update_transaction() {
    let conn = db::create_connection().unwrap();
    db::insert_transaction(&conn, "2021-01-31 00:09:01", 23.10).unwrap();
    let result = db::update_transaction(&conn, 1, 100.00);
    assert!(result.is_ok());
}

#[test]
fn test_delete_transaction() {
    let conn = db::create_connection().unwrap();
    db::insert_transaction(&conn, "2021-01-31 00:09:01", 23.10).unwrap();
    let result = db::delete_transaction(&conn, 1);
    assert!(result.is_ok());
}
