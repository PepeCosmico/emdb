use crate::instructions::{data::SelectData, Instructions};

use super::Db;

#[test]
fn database_exec_query_test() -> core::result::Result<(), Box<dyn std::error::Error>> {
    // initialize database
    let db = Db::init("./db")?;

    assert_eq!(
        db.exec_query(Instructions::<()>::Select(SelectData {
            table: "hola".to_string(),
            filters: None
        })),
        1
    );

    Ok(())
}
