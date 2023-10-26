use crate::instructions::{data::CreateTable, Instructions};

use super::Db;

#[test]
fn database_exec_query_create_table_test() -> core::result::Result<(), Box<dyn std::error::Error>> {
    // initialize database
    let db = Db::init("./db")?;

    assert_eq!(
        db.exec_query(Instructions::<()>::CreateTable(CreateTable {
            table: "table1".to_string()
        }))
        .unwrap(),
        ()
    );

    Ok(())
}
