use super::Pool;

#[test]
fn pool_init_to_path_test() -> core::result::Result<(), Box<dyn std::error::Error>> {
    Pool::init()?;
    Ok(())
}
