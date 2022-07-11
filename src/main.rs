use furdb::{FurColumn, FurDB, FurDBInfo, FurDataType, FurTable, FurTableInfo};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_path = PathBuf::from("D:\\Home\\Repositories\\FurDB\\TestDBs\\PersonData");
    let db_info = FurDBInfo::new("Dictionary")?;

    let db = FurDB::new(db_path, Some(db_info));

    let table_info = FurTableInfo::new();

    Ok(())
}
