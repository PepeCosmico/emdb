use super::*;

#[test]
fn db_init() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let db = Db::init("./db")?;
    assert_eq!(db.path, "./db/config.json");
    assert!(db.tables.is_empty());
    Ok(())
}
